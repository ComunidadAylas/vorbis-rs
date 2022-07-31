//! Safe, ergonomic, and high-quality Rust bindings to the `vorbisfile`, `libvorbisenc`, and
//! `libvorbis`  C libraries, focused on satisfying the audio processing requirements of
//! [PackSquash](https://github.com/ComunidadAylas/PackSquash).
//!
//! These bindings depend on the custom, low-level FFI bindings of the accompanying `ogg_sys` and
//! `vorbis_sys` crates, which link to the latest versions of the upstream `libogg` and patched
//! upstream `libvorbis` codebases, respectively. The upstream `libvorbis` codebase is patched
//! with the latest aoTuV and Lancer patches, which improve coding efficiency and performance.

#![feature(new_uninit)]
#![feature(iter_collect_into)]

#[cfg(test)]
mod tests;

/// A constant whose numeric value represents a libvorbis `OV_EBADHEADER` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EBADHEADER;
/// A constant whose numeric value represents a libvorbis `OV_EBADLINK` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EBADLINK;
/// A constant whose numeric value represents a libvorbis `OV_EBADPACKET` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EBADPACKET;
/// A constant whose numeric value represents a libvorbis `OV_EFAULT` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EFAULT;
/// A constant whose numeric value represents a libvorbis `OV_EIMPL` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EIMPL;
/// A constant whose numeric value represents a libvorbis `OV_EINVAL` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EINVAL;
/// A constant whose numeric value represents a libvorbis `OV_ENOSEEK` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_ENOSEEK;
/// A constant whose numeric value represents a libvorbis `OV_ENOTAUDIO` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_ENOTAUDIO;
/// A constant whose numeric value represents a libvorbis `OV_ENOTVORBIS` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_ENOTVORBIS;
/// A constant whose numeric value represents a libvorbis `OV_EOF` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EOF;
/// A constant whose numeric value represents a libvorbis `OV_EREAD` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EREAD;
/// A constant whose numeric value represents a libvorbis `OV_EVERSION` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EVERSION;
/// A constant whose numeric value represents a libvorbis `OV_FALSE` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_FALSE;
/// A constant whose numeric value represents a libvorbis `OV_HOLE` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_HOLE;

use core::slice;
use errno::{set_errno, Errno};
use std::borrow::Cow;
use std::convert::Infallible;
use std::ffi::{c_void, CString, NulError};
use std::hint::unreachable_unchecked;
use std::io::{Read, Write};
use std::mem::MaybeUninit;
use std::num::{NonZeroU32, NonZeroU8, TryFromIntError};
use std::os::raw::{c_int, c_long};
use std::{io, ptr};

use ogg_sys::{
	ogg_packet, ogg_page, ogg_stream_clear, ogg_stream_flush, ogg_stream_init, ogg_stream_packetin,
	ogg_stream_pageout, ogg_stream_pageout_fill, ogg_stream_state
};
use vorbis_sys::{
	ov_callbacks, ov_clear, ov_open_callbacks, ov_read_float, vorbis_analysis,
	vorbis_analysis_blockout, vorbis_analysis_buffer, vorbis_analysis_headerout,
	vorbis_analysis_init, vorbis_analysis_wrote, vorbis_bitrate_addblock,
	vorbis_bitrate_flushpacket, vorbis_block, vorbis_block_clear, vorbis_block_init,
	vorbis_comment, vorbis_comment_add_tag, vorbis_comment_clear, vorbis_comment_init,
	vorbis_dsp_clear, vorbis_dsp_state, vorbis_encode_ctl, vorbis_encode_init,
	vorbis_encode_init_vbr, vorbis_encode_setup_init, vorbis_encode_setup_managed, vorbis_info,
	vorbis_info_clear, vorbis_info_init, OggVorbis_File, OV_ECTL_RATEMANAGE2_SET
};

use thiserror::Error;
use tinyvec::{tiny_vec, TinyVec};

/// An error condition that may happen during an encoding or decoding operation.
#[derive(Error, Debug)]
pub enum VorbisError {
	/// A Vorbis C library function returned an error code.
	#[error("{__error_message}{__error_reason}")]
	LibraryError {
		/// This field is an implementation detail. DO NOT USE.
		#[doc(hidden)]
		__error_message: Cow<'static, str>,
		/// This field is an implementation detail. DO NOT USE.
		#[doc(hidden)]
		__error_reason: Cow<'static, str>,
		/// The raw error code returned by the library function whose execution failed.
		error_code: i32
	},
	/// The expected and actual count of channels in an audio block did not match.
	#[error("Expected {expected} channels in audio block, got {actual}")]
	InvalidAudioBlockChannelCount { expected: usize, actual: usize },
	/// The expected and actual count of samples in an audio block channel did not match.
	#[error("Expected {expected} samples in audio block channel, got {actual}")]
	InvalidAudioBlockSampleCount { expected: usize, actual: usize },
	/// The input stream is a chained Ogg Vorbis stream, which are not supported by either
	/// Minecraft and these bindings.
	#[error("Chained Ogg Vorbis streams are not supported. Please combine them into a single logical stream")]
	UnsupportedStreamChaining,
	/// A requested Vorbis user comment string contains a NUL character, which is not allowed.
	#[error("Invalid comment string: {0}")]
	InvalidCommentString(#[from] NulError),
	/// A number had a value that exceeded platform-dependent numerical limits.
	#[error("Integer outside of expected range: {0}")]
	RangeExceeded(#[from] TryFromIntError),
	/// An I/O error occurred.
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

impl From<Infallible> for VorbisError {
	fn from(_: Infallible) -> Self {
		// SAFETY: infallible can't be instantiated, so this conversion will never happen
		unsafe { unreachable_unchecked() }
	}
}

/// Converts the status code returned by a bound C function to an idiomatic [`VorbisError`].
macro_rules! return_value_to_result {
	( $func:ident ( $($arg:expr),* ), $is_ok:expr, $lib_name:literal, $result_to_error_string:expr ) => {{
		let return_value = $func($($arg),*) as i32;
		if $is_ok(return_value) {
			Ok(return_value)
		} else {
			Err(VorbisError::LibraryError {
				__error_message: concat!($lib_name, " error calling ", stringify!($func), ". Reason: ").into(),
				__error_reason: $result_to_error_string(return_value).into(),
				error_code: return_value
			})
		}
	}};
	( $func:ident ( $($arg:expr),* ), $lib_name:literal ) => {
		return_value_to_result!(
			$func($($arg),*),
			|return_value: i32| return_value >= 0,
			$lib_name,
			|return_value: i32| vorbis_error_code_to_string(return_value)
		)
	};
}

/// Converts the status code returned by a bound `libogg` C function to an idiomatic [`VorbisError`].
macro_rules! libogg_return_value_to_result {
	( $func:ident ( $($arg:expr),* ) ) => {
		return_value_to_result!($func($($arg),*), "libogg")
	};
}

/// Converts the status code returned by a bound `libvorbis` C function to an idiomatic [`VorbisError`].
macro_rules! libvorbis_return_value_to_result {
	( $func:ident ( $($arg:expr),* ) ) => {
		return_value_to_result!($func($($arg),*), "libvorbis")
	};
}

macro_rules! libvorbisenc_return_value_to_result {
	( $func:ident ( $($arg:expr),* ) ) => {
		return_value_to_result!($func($($arg),*), "libvorbisenc")
	};
}

/// Converts the status code returned by a bound `vorbisfile` C function to an idiomatic [`VorbisError`].
macro_rules! vorbisfile_return_value_to_result {
	( $func:ident ( $($arg:expr),* ) ) => {
		return_value_to_result!($func($($arg),*), "vorbisfile")
	};
}

/// Converts a status code returned by a bound C function to a more user-friendly string representation.
const fn vorbis_error_code_to_string(error_code: i32) -> &'static str {
	// List from
	// https://github.com/xiph/vorbis/blob/84c023699cdf023a32fa4ded32019f194afcdad0/include/vorbis/codec.h#L221-L235
	match error_code {
		OV_FALSE => "OV_FALSE",
		OV_EOF => "OV_EOF (stream at end of file)",
		OV_HOLE => "OV_HOLE (data interruption. Try repairing the file)",
		OV_EREAD => "OV_EREAD (I/O error)",
		OV_EFAULT => "OV_EFAULT (internal error)",
		OV_EIMPL => "OV_EIMPL (not implemented. Maybe try other parameters?)",
		OV_EINVAL => "OV_EINVAL (invalid parameter)",
		OV_ENOTVORBIS => "OV_ENOTVORBIS (not Vorbis data)",
		OV_EBADHEADER => "OV_EBADHEADER (invalid Vorbis stream header)",
		OV_EVERSION => "OV_EVERSION (Vorbis version mismatch)",
		OV_ENOTAUDIO => "OV_ENOTAUDIO (not audio data)",
		OV_EBADPACKET => "OV_EBADPACKET (invalid packet)",
		OV_EBADLINK => "OV_EBADLINK (invalid stream)",
		OV_ENOSEEK => "OV_ENOSEEK (not seekable)",
		_ => "unknown"
	}
}

/// A high-level abstraction for an Ogg stream.
struct OggStream {
	ogg_stream: ogg_stream_state
}

impl OggStream {
	/// Creates a new Ogg stream with the specified serial number.
	fn new(stream_serial: i32) -> Result<Self, VorbisError> {
		let mut ogg_stream = MaybeUninit::uninit();

		// SAFETY: we assume ogg_stream_init follows its documented contract,
		// and that C code does not assume that the struct stays at the same
		// memory location
		unsafe {
			libogg_return_value_to_result!(ogg_stream_init(
				ogg_stream.as_mut_ptr(),
				stream_serial
			))?;

			Ok(Self {
				ogg_stream: ogg_stream.assume_init()
			})
		}
	}

	/// Pulls the pending pages to write to an Ogg stream, and then writes them
	/// out to the specified sink. Optionally, it waits until there are at least
	/// `minimum_page_data_size` packet bytes to write or the page size limit
	/// is reached before yielding any pages.
	///
	/// This is a facade for the `ogg_stream_pageout` and `ogg_stream_pageout_fill`
	/// functions.
	fn write_pending_pages<W: Write>(
		&mut self,
		mut sink: W,
		minimum_page_data_size: Option<u16>
	) -> Result<(), VorbisError> {
		let mut ogg_page = MaybeUninit::uninit();

		loop {
			// SAFETY: we assume ogg_stream_pageout and ogg_stream_pageout_fill follow
			// their documented contract
			unsafe {
				let no_pending_page = if let Some(minimum_page_data_size) = minimum_page_data_size {
					ogg_stream_pageout_fill(
						&mut self.ogg_stream,
						ogg_page.as_mut_ptr(),
						minimum_page_data_size as i32
					)
				} else {
					ogg_stream_pageout(&mut self.ogg_stream, ogg_page.as_mut_ptr())
				} == 0;

				if no_pending_page {
					break;
				}

				OggPage {
					ogg_page: ogg_page.assume_init()
				}
				.write(&mut sink)?;
			}
		}

		Ok(())
	}

	/// Ends the current Ogg page and writes it to the stream immediately, if possible,
	/// even if little or no packet data was submitted since the last page.
	///
	/// This is a facade for the `ogg_stream_flush` function.
	fn flush<W: Write>(&mut self, mut sink: W) -> Result<(), VorbisError> {
		let mut ogg_page = MaybeUninit::uninit();

		// SAFETY: we assume ogg_stream_flush follows its documented contract
		unsafe {
			while ogg_stream_flush(&mut self.ogg_stream, ogg_page.as_mut_ptr()) != 0 {
				OggPage {
					ogg_page: ogg_page.assume_init()
				}
				.write(&mut sink)?;
			}
		}

		Ok(())
	}
}

impl Drop for OggStream {
	fn drop(&mut self) {
		// SAFETY: when this struct is dropped we have a valid Ogg stream to clear,
		// and there are no references to it
		unsafe { ogg_stream_clear(&mut self.ogg_stream) };
	}
}

/// A high-level abstraction for an Ogg packet.
struct OggPacket {
	ogg_packet: ogg_packet
}

impl OggPacket {
	/// Submits this Ogg packet to be written to the specified Ogg stream. The
	/// packet won't actually be written anywhere until
	/// [its pending pages are written](OggStream::write_pending_pages) or
	/// [flushed](OggStream::flush).
	///
	/// This is a facade for the `ogg_stream_packetin` function.
	fn submit(&mut self, ogg_stream: &mut OggStream) -> Result<(), VorbisError> {
		// SAFETY: we assume ogg_stream_packetin follows its documented contract
		unsafe {
			libogg_return_value_to_result!(ogg_stream_packetin(
				&mut ogg_stream.ogg_stream,
				&mut self.ogg_packet
			))?;
		}

		Ok(())
	}
}

/// A high-level abstraction for an Ogg page.
struct OggPage {
	ogg_page: ogg_page
}

impl OggPage {
	/// Writes this Ogg page to the specified sink.
	fn write<W: Write>(&self, mut sink: W) -> Result<(), VorbisError> {
		// SAFETY: we reinterpret an initialized, aligned C pointer that outlives
		// this stack frame with a buffer length to a Rust slice for read-only
		// operations, which is safe. header_len is guaranteed by the Ogg spec
		// to be at most 252, which can be safely casted to a usize. body_len is
		// also guaranteed by spec to fit in an unsigned 16 bits integer, and
		// usize can be assumed to always be at least 16 bits wide
		unsafe {
			sink.write_all(slice::from_raw_parts(
				self.ogg_page.header,
				self.ogg_page.header_len as usize
			))?;
			sink.write_all(slice::from_raw_parts(
				self.ogg_page.body,
				self.ogg_page.body_len as usize
			))?;
		}

		Ok(())
	}
}

/// A high-level abstraction for a Vorbis stream information struct.
struct VorbisInfo {
	vorbis_info: Box<vorbis_info>
}

impl VorbisInfo {
	/// Performs basic initialization of a new VorbisInfo struct. Most client code
	/// interested in encoding audio will want to complete this initialization by
	/// calling more methods on the returned struct, such as
	/// [`encode_init_vbr`](Self::encode_init_vbr).
	fn new() -> Self {
		let mut vorbis_info = Box::new_uninit();

		// SAFETY: we assume vorbis_info_init follows its documented contract.
		// vorbis_info is wrapped in a box to ensure it lives at a constant address
		// in memory (i.e., not moved between stack frames), which is necessary
		// because internal libvorbis encoding state may store pointers to it
		unsafe {
			vorbis_info_init(vorbis_info.as_mut_ptr());

			Self {
				vorbis_info: vorbis_info.assume_init()
			}
		}
	}

	/// Prepares this Vorbis codec information struct to encode an audio signal
	/// in pure VBR quality mode selected by a target bitrate (in bit/s). The
	/// bitrate management engine is not enabled.
	///
	/// An error may be returned if the encoder doesn't know any modes to use to
	/// encode the specified signal at the requested bitrate, and is thus unable to
	/// comply with the request.
	fn encode_init_vbr(
		&mut self,
		sampling_frequency: NonZeroU32,
		channels: NonZeroU8,
		target_bitrate: NonZeroU32
	) -> Result<(), VorbisError> {
		// SAFETY: we assume these functions follow the documented contract
		unsafe {
			libvorbisenc_return_value_to_result!(vorbis_encode_setup_managed(
				&mut *self.vorbis_info,
				channels.get() as c_long,
				sampling_frequency.get().try_into()?,
				-1,
				target_bitrate.get().try_into()?,
				-1
			))?;

			// Disable bitrate management engine to select a true VBR quality mode
			// based on its expected bitrate
			libvorbisenc_return_value_to_result!(vorbis_encode_ctl(
				&mut *self.vorbis_info,
				OV_ECTL_RATEMANAGE2_SET as c_int,
				ptr::null_mut()
			))?;

			libvorbisenc_return_value_to_result!(vorbis_encode_setup_init(&mut *self.vorbis_info))?;
		}

		Ok(())
	}

	/// Like [`encode_init_vbr`](Self::encode_init_vbr), but selects the quality mode
	/// directly from the specified quality factor, without internally converting a
	/// target bitrate to a quality factor. The valid range for this factor is
	/// [-0.1, 1].
	///
	/// This encoding strategy fixes the output subjective quality level, but lets
	/// Vorbis vary the target bitrate depending on the qualities of the input signal.
	/// An upside of this approach is that Vorbis can automatically increase or
	/// decrease the target bitrate according to how difficult the signal is to encode,
	/// which guarantees perceptually-consistent results while using an optimal bitrate.
	/// Another upside is that there always is some mode to encode audio at a given
	/// quality level. The downside is that the output bitrate is harder to predict
	/// across different types of audio signals.
	fn encode_init_quality_vbr(
		&mut self,
		sampling_frequency: NonZeroU32,
		channels: NonZeroU8,
		quality_factor: f32
	) -> Result<(), VorbisError> {
		unsafe {
			// SAFETY: we assume vorbis_encode_init_vbr follows its documented contract
			libvorbisenc_return_value_to_result!(vorbis_encode_init_vbr(
				&mut *self.vorbis_info,
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
	fn encode_init_abr(
		&mut self,
		sampling_frequency: NonZeroU32,
		channels: NonZeroU8,
		average_bitrate: NonZeroU32
	) -> Result<(), VorbisError> {
		// SAFETY: we assume vorbis_encode_init follows its documented contract
		unsafe {
			libvorbisenc_return_value_to_result!(vorbis_encode_init(
				&mut *self.vorbis_info,
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
	fn encode_init_constrained_abr(
		&mut self,
		sampling_frequency: NonZeroU32,
		channels: NonZeroU8,
		maximum_bitrate: NonZeroU32
	) -> Result<(), VorbisError> {
		// SAFETY: we assume vorbis_encode_init follows its documented contract
		unsafe {
			libvorbisenc_return_value_to_result!(vorbis_encode_init(
				&mut *self.vorbis_info,
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
	fn channels(&self) -> NonZeroU8 {
		// SAFETY: the Vorbis I specification allows up to 255 channels, so this
		// cast is always safe. It also requires at least one channel, and so do
		// we on VorbisInfo constructors
		unsafe { NonZeroU8::new(self.vorbis_info.channels as u8).unwrap_unchecked() }
	}
}

impl Drop for VorbisInfo {
	fn drop(&mut self) {
		// SAFETY: when this struct is dropped we have a valid Vorbis info struct to clear,
		// and no encode or decode state struct is referencing it
		unsafe { vorbis_info_clear(&mut *self.vorbis_info) };
	}
}

/// A high-level abstraction for a list of Vorbis user comments.
struct VorbisComments {
	vorbis_comment: vorbis_comment
}

impl VorbisComments {
	/// Creates a new list of Vorbis user comments.
	fn new() -> Self {
		let mut vorbis_comment = MaybeUninit::uninit();

		// SAFETY: we assume vorbis_comment_init follows its documented contract
		unsafe {
			vorbis_comment_init(vorbis_comment.as_mut_ptr());

			Self {
				vorbis_comment: vorbis_comment.assume_init()
			}
		}
	}

	/// Adds the specified pair of tags to the list of user comments held by this struct.
	fn add_tag<'tag, 'value>(
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
			)
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

/// A high-level abstraction that holds all the needed state for a Vorbis encoder.
struct VorbisEncodingState {
	vorbis_info: VorbisInfo,
	vorbis_dsp_state: Box<vorbis_dsp_state>,
	vorbis_block: Box<vorbis_block>
}

impl VorbisEncodingState {
	/// Creates a new Vorbis encoder state from the specified Vorbis stream
	/// information, which should be initialized for encoding.
	fn new(mut vorbis_info: VorbisInfo) -> Result<Self, VorbisError> {
		let mut vorbis_dsp_state = Box::new_uninit();
		let mut vorbis_block = Box::new_uninit();

		// SAFETY: we assume vorbis_analysis_init and vorbis_block_init follow
		// their documented contract. The structs are wrapped in boxes to ensure
		// they live at constant addresses in memory, which is necessary because
		// internal libvorbis encoding state stores pointers to such addresses. We
		// take the ownership of vorbis_info even though we don't use it to ensure
		// its lifetime is as long as the lifetime of the encoding state and that
		// client code does not corrupt it
		unsafe {
			libvorbis_return_value_to_result!(vorbis_analysis_init(
				vorbis_dsp_state.as_mut_ptr(),
				&mut *vorbis_info.vorbis_info
			))?;
			let mut vorbis_dsp_state = vorbis_dsp_state.assume_init();

			libvorbis_return_value_to_result!(vorbis_block_init(
				&mut *vorbis_dsp_state,
				vorbis_block.as_mut_ptr()
			))?;

			Ok(Self {
				vorbis_info,
				vorbis_dsp_state,
				vorbis_block: vorbis_block.assume_init()
			})
		}
	}

	/// Returns the header packets needed to inform decoders about the encoding
	/// parameters and identify this stream as Vorbis, alongside storing minimal
	/// metadata to it, in the form of user comments.
	fn get_header_packets(
		&mut self,
		vorbis_comments: &mut VorbisComments
	) -> Result<[OggPacket; 3], VorbisError> {
		let mut identification_header = MaybeUninit::uninit();
		let mut comment_header = MaybeUninit::uninit();
		let mut setup_header = MaybeUninit::uninit();

		// SAFETY: we assume vorbis_analysis_headerout follows its documented contract
		// and that the returned packets do not reference data from vorbis_comments
		unsafe {
			libvorbis_return_value_to_result!(vorbis_analysis_headerout(
				&mut *self.vorbis_dsp_state,
				&mut vorbis_comments.vorbis_comment,
				identification_header.as_mut_ptr(),
				comment_header.as_mut_ptr(),
				setup_header.as_mut_ptr()
			))?;

			Ok([
				OggPacket {
					ogg_packet: identification_header.assume_init()
				},
				OggPacket {
					ogg_packet: comment_header.assume_init()
				},
				OggPacket {
					ogg_packet: setup_header.assume_init()
				}
			])
		}
	}
}

impl Drop for VorbisEncodingState {
	fn drop(&mut self) {
		// SAFETY: when this struct is dropped we have structs in valid state to clear.
		// Rust destructor execution order guarantees that VorbisInfo is dropped after
		// this, which is necessary because clearing these structs requires data
		// from VorbisInfo
		unsafe {
			vorbis_block_clear(&mut *self.vorbis_block);
			vorbis_dsp_clear(&mut *self.vorbis_dsp_state);
		}
	}
}

/// Represents a bitrate management strategy that a Vorbis encoder can use.
#[derive(Copy, Clone, Debug)]
pub enum VorbisBitrateManagementStrategy {
	/// Pure VBR quality mode, selected by a target bitrate (in bit/s). The
	/// bitrate management engine is not enabled. The average bitrate will
	/// usually be close to the target, but there are no guarantees. Easier
	/// or harder than expected to encode audio may be encoded at a
	/// significantly different bitrate.
	Vbr {
		/// The bitrate to target with this strategy.
		target_bitrate: NonZeroU32
	},
	/// Similar to [`Vbr`](Self::Vbr), this encoding strategy fixes the output
	/// subjective quality level, but lets Vorbis vary the target bitrate
	/// depending on the qualities of the input signal. An upside of this
	/// approach is that Vorbis can automatically increase or decrease the target
	/// bitrate according to how difficult the signal is to encode, which
	/// guarantees perceptually-consistent results while using an optimal bitrate.
	/// Another upside is that there always is some mode to encode audio at a given
	/// quality level. The downside is that the output bitrate is harder to predict
	/// across different types of audio signals.
	QualityVbr {
		/// The perceptual quality factor to target with this strategy, in the
		/// [-0.1, 1] range.
		target_quality: f32
	},
	/// ABR mode, selected by an average bitrate (in bit/s). The bitrate
	/// management engine is enabled to ensure that the instantaneous bitrate
	/// does not divert significantly from the specified average over time,
	/// but no hard bitrate limits are imposed. Any bitrate fluctuations are
	/// guaranteed to be minor and short.
	Abr {
		/// The bitrate to target with this strategy.
		average_bitrate: NonZeroU32
	},
	/// Constrained ABR mode, selected by a hard maximum bitrate (in bit/s).
	/// The bitrate management engine is enabled to ensure that the instantaneous
	/// bitrate never exceeds the specified maximum bitrate, which is a hard
	/// limit. Internally, the encoder will target an average bitrate that's
	/// slightly lower than the specified maximum bitrate. The stream is
	/// guaranteed to never go above the specified bitrate, at the cost of a
	/// lower bitrate, and thus lower audio quality, on average.
	ConstrainedAbr {
		/// The bitrate to target with this strategy.
		maximum_bitrate: NonZeroU32
	}
}

/// An encoder that transforms blocks of planar, single-precision float audio
/// samples to a perceptually-encoded Ogg Vorbis stream.
pub struct VorbisEncoder<W: Write> {
	ogg_stream: OggStream,
	vorbis_encoding_state: VorbisEncodingState,
	sink: W,
	minimum_page_data_size: Option<u16>,
	finished: bool
}

impl<W: Write> VorbisEncoder<W> {
	/// Creates a new Vorbis encoder with the specified metadata, for the
	/// specified signal, using some bitrate management strategy and minimum
	/// Ogg page size, that will write data to the provided sink.
	pub fn new<'tags, 'values, T: Into<Cow<'tags, str>>, V: Into<Cow<'values, str>>>(
		stream_serial: i32,
		comment_tags: impl IntoIterator<Item = (T, V)>,
		sampling_frequency: NonZeroU32,
		channels: NonZeroU8,
		bitrate_management_strategy: VorbisBitrateManagementStrategy,
		minimum_page_data_size: Option<u16>,
		mut sink: W
	) -> Result<Self, VorbisError> {
		// Tear up the Ogg stream
		let mut ogg_stream = OggStream::new(stream_serial)?;

		// Tear up the Vorbis encoder
		let mut vorbis_info = VorbisInfo::new();
		match bitrate_management_strategy {
			VorbisBitrateManagementStrategy::Vbr { target_bitrate } => {
				vorbis_info.encode_init_vbr(sampling_frequency, channels, target_bitrate)
			}
			VorbisBitrateManagementStrategy::QualityVbr { target_quality } => {
				vorbis_info.encode_init_quality_vbr(sampling_frequency, channels, target_quality)
			}
			VorbisBitrateManagementStrategy::Abr { average_bitrate } => {
				vorbis_info.encode_init_abr(sampling_frequency, channels, average_bitrate)
			}
			VorbisBitrateManagementStrategy::ConstrainedAbr { maximum_bitrate } => vorbis_info
				.encode_init_constrained_abr(sampling_frequency, channels, maximum_bitrate)
		}?;

		let mut vorbis_encoding_state = VorbisEncodingState::new(vorbis_info)?;

		// Store our user comments, if any
		let mut vorbis_comments = VorbisComments::new();
		for (tag, value) in comment_tags {
			vorbis_comments.add_tag(tag, value)?;
		}

		// Get the Vorbis header packets and submit them for encapsulation
		for mut header_packet in vorbis_encoding_state.get_header_packets(&mut vorbis_comments)? {
			header_packet.submit(&mut ogg_stream)?;
		}

		// Force the header packets we submitted to be written, and the first audio packet to begin
		// on its own page, as mandated by the Vorbis I spec
		ogg_stream.flush(&mut sink)?;

		Ok(Self {
			ogg_stream,
			vorbis_encoding_state,
			sink,
			minimum_page_data_size,
			finished: false
		})
	}

	/// Submits the specified audio block for encoding by Vorbis. Encoded data will be written
	/// to the configured sink automatically as it becomes available.
	pub fn encode_audio_block<B: AsRef<[S]>, S: AsRef<[f32]>>(
		&mut self,
		audio_block: B
	) -> Result<(), VorbisError> {
		let audio_block = audio_block.as_ref();
		let audio_channels = self.vorbis_encoding_state.vorbis_info.channels().get() as usize;

		if audio_channels != audio_block.len() {
			return Err(VorbisError::InvalidAudioBlockChannelCount {
				actual: audio_channels,
				expected: audio_block.len()
			});
		}

		let sample_count = audio_block[0].as_ref().len();
		let encoder_buffer = unsafe {
			slice::from_raw_parts_mut(
				vorbis_analysis_buffer(
					&mut *self.vorbis_encoding_state.vorbis_dsp_state,
					sample_count.try_into()?
				),
				audio_channels
			)
		};

		for (channel_samples, channel_encode_buffer) in
			audio_block.iter().zip(encoder_buffer.iter_mut())
		{
			let channel_samples = channel_samples.as_ref();

			if channel_samples.len() != sample_count {
				return Err(VorbisError::InvalidAudioBlockSampleCount {
					actual: channel_samples.len(),
					expected: sample_count
				});
			}

			// SAFETY: both the source and destination locations are valid.
			// They do not overlap each other because they belong to different
			// memory allocations
			unsafe {
				channel_samples
					.as_ptr()
					.copy_to_nonoverlapping(*channel_encode_buffer, sample_count);
			}
		}

		// SAFETY: we assume vorbis_analysis_wrote follows its documented contract
		unsafe {
			libogg_return_value_to_result!(vorbis_analysis_wrote(
				&mut *self.vorbis_encoding_state.vorbis_dsp_state,
				sample_count as i32
			))?;
		}

		self.write_pending_blocks()
	}

	/// Asks the low-level Vorbis encoder for pending packets, and writes them out
	/// to Ogg pages as they become available.
	fn write_pending_blocks(&mut self) -> Result<(), VorbisError> {
		// SAFETY: we assume the functions inside this unsafe block follow their
		// documented contract
		unsafe {
			while libvorbis_return_value_to_result!(vorbis_analysis_blockout(
				&mut *self.vorbis_encoding_state.vorbis_dsp_state,
				&mut *self.vorbis_encoding_state.vorbis_block
			))? == 1
			{
				libvorbis_return_value_to_result!(vorbis_analysis(
					&mut *self.vorbis_encoding_state.vorbis_block,
					ptr::null_mut()
				))?;
				libvorbis_return_value_to_result!(vorbis_bitrate_addblock(
					&mut *self.vorbis_encoding_state.vorbis_block
				))?;

				let mut ogg_packet = MaybeUninit::uninit();
				while libvorbis_return_value_to_result!(vorbis_bitrate_flushpacket(
					&mut *self.vorbis_encoding_state.vorbis_dsp_state,
					ogg_packet.as_mut_ptr()
				))? == 1
				{
					OggPacket {
						ogg_packet: ogg_packet.assume_init()
					}
					.submit(&mut self.ogg_stream)?;

					self.ogg_stream
						.write_pending_pages(&mut self.sink, self.minimum_page_data_size)?;
				}
			}
		}

		Ok(())
	}

	/// Finishes the encoded Vorbis stream, signalling its end and writing any pending
	/// data to the configured sink.
	///
	/// This is automatically done when the encoder is dropped, but calling `finish`
	/// explicitly is recommended for cases where handling errors here is necessary.
	pub fn finish(mut self) -> Result<(), VorbisError> {
		// SAFETY: we assume that vorbis_analysis_wrote follows its documented contract
		unsafe {
			libvorbis_return_value_to_result!(vorbis_analysis_wrote(
				&mut *self.vorbis_encoding_state.vorbis_dsp_state,
				0
			))?
		};

		let write_pending_result = self.write_pending_blocks();
		self.finished = true; // Avoids implicit finish on drop
		write_pending_result
	}
}

impl<W: Write> Drop for VorbisEncoder<W> {
	fn drop(&mut self) {
		// Finishing the stream after it has been already finished generates two
		// EOS packets, which is illegal
		if !self.finished {
			// SAFETY: we assume that vorbis_analysis_wrote follows its documented contract
			if unsafe {
				libvorbis_return_value_to_result!(vorbis_analysis_wrote(
					&mut *self.vorbis_encoding_state.vorbis_dsp_state,
					0
				))
			}
			.is_ok()
			{
				self.write_pending_blocks().ok();
			}
		}
	}
}

/// A decoder that turns a perceptually-encoded, non-chained Ogg Vorbis stream into
/// blocks of planar, single-precision float audio samples.
pub struct VorbisDecoder {
	ogg_vorbis_file: Box<OggVorbis_File>,
	last_audio_block: Option<VorbisAudioSamples>
}

impl VorbisDecoder {
	/// Creates a new Vorbis decoder that will read an Ogg Vorbis stream from the
	/// specified source.
	pub fn new<R: Read, S: Into<Box<R>>>(source: S) -> Result<Self, VorbisError> {
		let mut ogg_vorbis_file = Box::new_uninit();
		// SAFETY: we assume ov_open_callbacks follows its documented contract.
		// OggVorbis_File must be boxed because the C code assumes it doesn't
		// move around in memory
		unsafe {
			vorbisfile_return_value_to_result!(ov_open_callbacks(
				// Transfer the ownership of the Read opaque object to C code.
				// As far as we know, we can't properly use generics across the C
				// boundary, so we have to resort to dynamic dispatch and trait
				// objects. We use double indirection here to turn the unsized,
				// fat pointer to a trait object to a sized, FFI safe pointer
				Box::into_raw(Box::new(source.into() as Box<dyn Read>)) as *mut c_void,
				ogg_vorbis_file.as_mut_ptr(),
				ptr::null(),
				0,
				ov_callbacks {
					read_func: {
						// This read callback should match the stdio fread behavior.
						// See: https://man7.org/linux/man-pages/man3/fread.3p.html
						unsafe extern "C" fn read_func(
							ptr: *mut c_void,
							size: usize,
							count: usize,
							datasource: *mut c_void
						) -> usize {
							let source = &mut *(datasource as *mut Box<dyn Read>);
							let buf = slice::from_raw_parts_mut(ptr as *mut u8, size * count);
							match source.read(buf) {
								Ok(n) => n / size,
								Err(err) => {
									// vorbisfile checks errno to tell EOF apart from read errors:
									// https://xiph.org/vorbis/doc/vorbisfile/callbacks.html
									// Rust Read trait implementations are not required to set
									// errno, so make sure we set errno with the most informative
									// value possible, falling back to a non-zero errno, which is
									// implied by the C standard to signal some condition
									set_errno(Errno(err.raw_os_error().unwrap_or(i32::MAX)));

									0
								}
							}
						}
						Some(read_func)
					},
					seek_func: None,
					close_func: {
						unsafe extern "C" fn close_func(datasource: *mut c_void) -> c_int {
							// Drop the Read when it's no longer needed by vorbisfile.
							// This is called by ov_clear
							drop(Box::from_raw(datasource as *mut Box<dyn Read>));

							0
						}
						Some(close_func)
					},
					tell_func: None
				}
			))?;

			Ok(Self {
				ogg_vorbis_file: ogg_vorbis_file.assume_init(),
				last_audio_block: None
			})
		}
	}

	/// Decodes the next block of samples from this Vorbis stream, in planar format (i.e., one
	/// vector of samples per channel). The order of channels is defined by the Vorbis I
	/// specification. `Ok(None)` will be returned when the stream ends, and thus there are no
	/// more samples to decode.
	pub fn decode_audio_block(&mut self) -> Result<Option<&VorbisAudioSamples>, VorbisError> {
		let mut current_bitstream = MaybeUninit::uninit();
		let mut sample_buf = MaybeUninit::uninit();

		// SAFETY: we assume ov_read_float follows its documented contract. See the
		// VorbisAudioSamples implementation for more safety information
		unsafe {
			let samples_read = vorbisfile_return_value_to_result!(ov_read_float(
				&mut *self.ogg_vorbis_file,
				sample_buf.as_mut_ptr(),
				2048, // Most stereo Ogg Vorbis files in the wild use a maximum block size of 2048 samples
				current_bitstream.as_mut_ptr()
			))?;

			if samples_read > 0 {
				// This is not documented, but we can only assume the current bitstream number was
				// initialized if we read some sample; else, ov_read_float may not write to
				// current_bitstream. Read the ov_read_float source code
				if current_bitstream.assume_init() != 0 {
					return Err(VorbisError::UnsupportedStreamChaining);
				}

				self.last_audio_block = Some(VorbisAudioSamples::new(
					sample_buf.assume_init(),
					(*self.ogg_vorbis_file.vi).channels as usize,
					samples_read as usize
				));

				Ok(Some(self.last_audio_block.as_ref().unwrap()))
			} else {
				Ok(None)
			}
		}
	}

	/// Returns the number of channels of the audio signal decoded by this decoder.
	pub fn channels(&mut self) -> NonZeroU8 {
		// SAFETY: as per the Vorbis I specification, the channel count must be greater than
		// zero and at most 255
		unsafe { NonZeroU8::new((*self.ogg_vorbis_file.vi).channels as u8).unwrap_unchecked() }
	}

	/// Returns the sampling frequency of the audio signal decoded by this decoder.
	pub fn sampling_frequency(&mut self) -> NonZeroU32 {
		// SAFETY: as per the Vorbis I specification, the sampling frequency must be greater
		// than zero and fit in an unsigned 32-bit integer
		unsafe { NonZeroU32::new((*self.ogg_vorbis_file.vi).rate as u32).unwrap_unchecked() }
	}
}

impl Drop for VorbisDecoder {
	fn drop(&mut self) {
		unsafe { ov_clear(&mut *self.ogg_vorbis_file) };
	}
}

/// Contains a block of audio samples. This struct is returned by a [`VorbisDecoder`].
pub struct VorbisAudioSamples {
	// This static lifetime is not really accurate. See safety comments below for more details
	audio_samples: TinyVec<[&'static [f32]; 2]>
}

impl VorbisAudioSamples {
	/// Creates a new block of audio samples.
	///
	/// # Safety
	/// This function assumes that it is called by trusted code on [`VorbisDecoder`] only.
	/// For more information, please read the safety comment in its source code.
	fn new(buf: *mut *mut f32, channels: usize, samples_read: usize) -> Self {
		// SAFETY: ov_read_float initializes a pointer to an array of pointers. The pointers in
		// the array point to the array of samples for each channel. The idiomatic Rust equivalent
		// to that double indirection is a slice of slices. However, slices are fat pointers,
		// while the pointers we got from C are thin, thus we need to convert each channel
		// sample pointer in the array to a fat slice pointer. To iterate over that pointer
		// array idiomatically, we also convert the first indirection level to a slice.
		//
		// All of these pointers point to memory owned by vorbisfile. The documentation is not
		// crystal-clear about this, but after examining the source code and more pieces of
		// documentation it can be assumed that this memory is valid until the next audio block
		// is decoded or ov_clear is called, whichever happens first.
		//
		// Both of those lifetime-ending actions imply that we stop being accessible, because
		// either the borrow checker prevents clients from using a reference to us if they
		// call other mutable methods on the decoder, or the VorbisDecoder whose lifetime we're
		// linked with is being dropped: we can't outlive it. We can assume all of this only
		// because the trusted code in VorbisDecoder instantiates us
		let mut audio_samples = tiny_vec!([&[f32]; 2]);
		unsafe {
			slice::from_raw_parts(buf, channels)
				.iter()
				.map(|channel_samples| {
					slice::from_raw_parts(*channel_samples, samples_read as usize)
				})
				.collect_into(&mut audio_samples);
		}

		Self { audio_samples }
	}

	/// Returns a reference to the buffer of planar, single-precision float samples held
	/// by this audio block.
	pub fn samples(&self) -> &[&[f32]] {
		&self.audio_samples
	}
}
