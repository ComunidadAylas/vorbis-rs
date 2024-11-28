use std::{
	borrow::Cow,
	ffi::CString,
	mem::MaybeUninit,
	num::{NonZeroU32, NonZeroU8},
	os::raw::{c_int, c_long},
	ptr
};

use aotuv_lancer_vorbis_sys::{
	vorbis_comment, vorbis_comment_add_tag, vorbis_comment_clear, vorbis_comment_init,
	vorbis_encode_ctl, vorbis_encode_init, vorbis_encode_init_vbr, vorbis_encode_setup_init,
	vorbis_encode_setup_managed, vorbis_info, vorbis_info_clear, vorbis_info_init,
	OV_ECTL_RATEMANAGE2_SET
};

use crate::common::VorbisError;

/// A high-level abstraction for a Vorbis stream information struct.
pub struct VorbisInfo {
	pub(crate) vorbis_info: *mut vorbis_info
}

impl VorbisInfo {
	/// Performs basic initialization of a new `VorbisInfo` struct. Most client code
	/// interested in encoding audio will want to complete this initialization by
	/// calling more methods on the returned struct, such as
	/// [`encode_init_vbr`](Self::encode_init_vbr).
	pub fn new() -> Self {
		let vorbis_info = Box::into_raw(Box::<vorbis_info>::new_uninit()).cast();

		// SAFETY: we assume vorbis_info_init follows its documented contract.
		// vorbis_info is allocated on the heap to ensure it lives at a constant
		// address in memory (i.e., not moved between stack frames), which is
		// necessary because internal libvorbis encoding state may store pointers
		// to it. After this call, the struct pointed to is fully initialized
		unsafe { vorbis_info_init(vorbis_info) };

		Self { vorbis_info }
	}

	/// Prepares this Vorbis codec information struct to encode an audio signal
	/// in pure VBR quality mode selected by a target bitrate (in bit/s). The
	/// bitrate management engine is not enabled.
	///
	/// An error may be returned if the encoder doesn't know any modes to use to
	/// encode the specified signal at the requested bitrate, and is thus unable to
	/// comply with the request.
	pub fn encode_init_vbr(
		&mut self,
		sampling_frequency: NonZeroU32,
		channels: NonZeroU8,
		target_bitrate: NonZeroU32
	) -> Result<(), VorbisError> {
		// SAFETY: we assume these functions follow the documented contract
		unsafe {
			libvorbisenc_return_value_to_result!(vorbis_encode_setup_managed(
				self.vorbis_info,
				channels.get() as c_long,
				sampling_frequency.get().try_into()?,
				-1,
				target_bitrate.get().try_into()?,
				-1
			))?;

			// Disable bitrate management engine to select a true VBR quality mode
			// based on its expected bitrate
			libvorbisenc_return_value_to_result!(vorbis_encode_ctl(
				self.vorbis_info,
				OV_ECTL_RATEMANAGE2_SET as c_int,
				ptr::null_mut()
			))?;

			libvorbisenc_return_value_to_result!(vorbis_encode_setup_init(self.vorbis_info))?;
		}

		Ok(())
	}

	/// Like [`encode_init_vbr`](Self::encode_init_vbr), but selects the quality mode
	/// directly from the specified quality factor, without internally converting a
	/// target bitrate to a quality factor. The valid range for this factor is
	/// [-0.2, 1].
	///
	/// This encoding strategy fixes the output subjective quality level, but lets
	/// Vorbis vary the target bitrate depending on the qualities of the input signal.
	/// An upside of this approach is that Vorbis can automatically increase or
	/// decrease the target bitrate according to how difficult the signal is to encode,
	/// which guarantees perceptually-consistent results while using an optimal bitrate.
	/// Another upside is that there always is some mode to encode audio at a given
	/// quality level. The downside is that the output bitrate is harder to predict
	/// across different types of audio signals.
	pub fn encode_init_quality_vbr(
		&mut self,
		sampling_frequency: NonZeroU32,
		channels: NonZeroU8,
		quality_factor: f32
	) -> Result<(), VorbisError> {
		unsafe {
			// SAFETY: we assume vorbis_encode_init_vbr follows its documented contract
			libvorbisenc_return_value_to_result!(vorbis_encode_init_vbr(
				self.vorbis_info,
				channels.get() as c_long,
				sampling_frequency.get().try_into()?,
				quality_factor
			))?;
		}

		Ok(())
	}

	/// Prepares this Vorbis codec information struct to encode an audio signal
	/// in ABR mode selected by an average bitrate (in bit/s). The bitrate management
	/// engine is enabled to ensure that the instantaneous bitrate does not divert
	/// significantly from the specified average, but no hard bitrate limits are imposed.
	///
	/// An error may be returned if the encoder doesn't know any modes to use to
	/// encode the specified signal at the requested bitrate, and is thus unable to
	/// comply with the request.
	pub fn encode_init_abr(
		&mut self,
		sampling_frequency: NonZeroU32,
		channels: NonZeroU8,
		average_bitrate: NonZeroU32
	) -> Result<(), VorbisError> {
		// SAFETY: we assume vorbis_encode_init follows its documented contract
		unsafe {
			libvorbisenc_return_value_to_result!(vorbis_encode_init(
				self.vorbis_info,
				channels.get() as c_long,
				sampling_frequency.get().try_into()?,
				-1,
				average_bitrate.get().try_into()?,
				-1
			))?;
		}

		Ok(())
	}

	/// Prepares this Vorbis codec information struct to encode an audio signal
	/// in constrained ABR mode selected by a hard maximum bitrate (in bit/s).
	/// The bitrate management engine is enabled to ensure that the instantaneous
	/// bitrate never exceeds the specified maximum bitrate, which is a hard
	/// limit. Internally, the encoder will target an average bitrate that's
	/// slightly lower than the specified maximum bitrate.
	///
	/// An error may be returned if the encoder doesn't know any modes to use to
	/// encode the specified signal at the requested bitrate, and is thus unable to
	/// comply with the request.
	pub fn encode_init_constrained_abr(
		&mut self,
		sampling_frequency: NonZeroU32,
		channels: NonZeroU8,
		maximum_bitrate: NonZeroU32
	) -> Result<(), VorbisError> {
		// SAFETY: we assume vorbis_encode_init follows its documented contract
		unsafe {
			libvorbisenc_return_value_to_result!(vorbis_encode_init(
				self.vorbis_info,
				channels.get() as c_long,
				sampling_frequency.get().try_into()?,
				maximum_bitrate.get().try_into()?,
				-1,
				-1
			))?;
		}

		Ok(())
	}

	/// Returns the number of channels of the audio signal this Vorbis stream
	/// information struct was initialized for.
	pub fn channels(&self) -> NonZeroU8 {
		// SAFETY: the Vorbis I specification allows up to 255 channels, so this
		// cast is always safe. It also requires at least one channel, and so do
		// we on VorbisInfo constructors.
		unsafe { NonZeroU8::new_unchecked((*self.vorbis_info).channels as u8) }
	}
}

impl Drop for VorbisInfo {
	fn drop(&mut self) {
		// SAFETY: when this struct is dropped we have a valid Vorbis info struct to clear,
		// and no encode or decode state struct is referencing it
		unsafe {
			vorbis_info_clear(self.vorbis_info);
			drop(Box::from_raw(self.vorbis_info));
		}
	}
}

/// A high-level abstraction for a list of Vorbis user comments.
pub struct VorbisComments {
	pub(crate) vorbis_comment: vorbis_comment
}

impl VorbisComments {
	/// Creates a new list of Vorbis user comments.
	pub fn new() -> Self {
		let mut vorbis_comment = MaybeUninit::uninit();

		// SAFETY: we assume vorbis_comment_init follows its documented contract
		unsafe {
			vorbis_comment_init(vorbis_comment.as_mut_ptr());

			Self {
				vorbis_comment: vorbis_comment.assume_init()
			}
		}
	}

	/// Adds the specified tag-value pair to the list of user comments held by this struct.
	pub fn add_tag<'tag, 'value>(
		&mut self,
		tag: impl Into<Cow<'tag, str>>,
		value: impl Into<Cow<'value, str>>
	) -> Result<(), VorbisError> {
		fn cow_string_to_byte_vec(string: Cow<'_, str>) -> Vec<u8> {
			match string {
				Cow::Owned(string) => string.into(),
				Cow::Borrowed(string) => string.into()
			}
		}

		let tag_c_string = CString::new(cow_string_to_byte_vec(tag.into()))?;
		let value_c_string = CString::new(cow_string_to_byte_vec(value.into()))?;
		// SAFETY: we assume vorbis_comment_add_tag follows its documented contract
		unsafe {
			vorbis_comment_add_tag(
				&mut self.vorbis_comment,
				tag_c_string.as_ptr(),
				value_c_string.as_ptr()
			);
		};

		Ok(())
	}
}

impl Drop for VorbisComments {
	fn drop(&mut self) {
		// SAFETY: when this struct is dropped we have a valid Vorbis comment struct to clear
		unsafe { vorbis_comment_clear(&mut self.vorbis_comment) };
	}
}
