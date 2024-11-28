use std::{mem::MaybeUninit, num::NonZeroU32};

use aotuv_lancer_vorbis_sys::{
	vorbis_analysis_headerout, vorbis_analysis_init, vorbis_block, vorbis_block_clear,
	vorbis_block_init, vorbis_dsp_clear, vorbis_dsp_state
};

use crate::common::{OggPacket, VorbisComments, VorbisError, VorbisInfo};

/// A high-level abstraction that holds all the needed state for a Vorbis encoder.
pub struct VorbisEncodingState {
	pub(crate) vorbis_info: VorbisInfo,
	pub(crate) vorbis_dsp_state: *mut vorbis_dsp_state,
	pub(crate) vorbis_block: *mut vorbis_block
}

impl VorbisEncodingState {
	/// Creates a new Vorbis encoder state from the specified Vorbis stream
	/// information, which should be initialized for encoding.
	pub fn new(vorbis_info: VorbisInfo) -> Result<Self, VorbisError> {
		let vorbis_dsp_state = Box::into_raw(Box::<vorbis_dsp_state>::new_uninit());
		let vorbis_block = Box::into_raw(Box::<vorbis_block>::new_uninit());

		// SAFETY: we assume vorbis_analysis_init and vorbis_block_init follow
		// their documented contract. The structs are allocated in the heap to ensure
		// they live at constant addresses in memory, which is necessary because
		// internal libvorbis encoding state stores pointers to such addresses, and
		// eagerly extracted from `Box`es to guarantee that no lack of aliasing and
		// interior mutability assumptions made by `Box` are violated (i.e., `Box` is
		// an owned unique pointer, see #22). We take the ownership of vorbis_info even
		// though we don't use it to ensure its lifetime is as long as the lifetime of
		// the encoding state and that client code does not corrupt it
		let init_error = 'init: {
			if let Err(err) = unsafe {
				libvorbis_return_value_to_result!(vorbis_analysis_init(
					vorbis_dsp_state.cast(),
					vorbis_info.vorbis_info
				))
			} {
				break 'init Some(err);
			}
			// vorbis_dsp_state is now initialized

			if let Err(err) = unsafe {
				libvorbis_return_value_to_result!(vorbis_block_init(
					vorbis_dsp_state.cast(),
					vorbis_block.cast()
				))
			} {
				break 'init Some(err);
			}
			// vorbis_block is now initialized

			None
		};

		match init_error {
			Some(err) => {
				unsafe {
					drop(Box::from_raw(vorbis_dsp_state));
					drop(Box::from_raw(vorbis_block));
				}

				Err(err)?
			}
			None => Ok(Self {
				vorbis_info,
				vorbis_dsp_state: vorbis_dsp_state.cast(),
				vorbis_block: vorbis_block.cast()
			})
		}
	}

	/// Returns the header packets needed to inform decoders about the encoding
	/// parameters and identify this stream as Vorbis, alongside storing minimal
	/// metadata to it, in the form of user comments.
	pub fn get_header_packets(
		&mut self,
		vorbis_comments: &mut VorbisComments
	) -> Result<[OggPacket; 3], VorbisError> {
		let mut identification_header = MaybeUninit::uninit();
		let mut comment_header = MaybeUninit::uninit();
		let mut setup_header = MaybeUninit::uninit();

		// SAFETY: we assume vorbis_analysis_headerout follows its documented contract,
		// that the returned packets do not reference data from vorbis_comments, and
		// that vorbis_analysis_headerout does not write to the
		// vorbis_comments.vorbis_comment field reference
		unsafe {
			libvorbis_return_value_to_result!(vorbis_analysis_headerout(
				self.vorbis_dsp_state,
				&raw mut vorbis_comments.vorbis_comment,
				identification_header.as_mut_ptr(),
				comment_header.as_mut_ptr(),
				setup_header.as_mut_ptr()
			))?;

			Ok([
				OggPacket::new(identification_header.assume_init()),
				OggPacket::new(comment_header.assume_init()),
				OggPacket::new(setup_header.assume_init())
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
			vorbis_block_clear(self.vorbis_block);
			vorbis_dsp_clear(self.vorbis_dsp_state);

			drop(Box::from_raw(self.vorbis_block));
			drop(Box::from_raw(self.vorbis_dsp_state));
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
		/// [-0.2, 1] range.
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

impl Default for VorbisBitrateManagementStrategy {
	fn default() -> Self {
		Self::QualityVbr {
			// ~ 80 kbit/s for stereo, 44.1 kHz signals
			target_quality: 0.5
		}
	}
}
