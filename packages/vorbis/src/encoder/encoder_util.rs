use vorbis_sys::{
	vorbis_analysis_headerout, vorbis_analysis_init, vorbis_block, vorbis_block_clear,
	vorbis_block_init, vorbis_dsp_clear, vorbis_dsp_state
};

use std::mem::MaybeUninit;
use std::num::NonZeroU32;

use crate::common::{
	assume_init_box, vorbis_error_code_to_string, OggPacket, VorbisComments, VorbisError,
	VorbisInfo
};

/// A high-level abstraction that holds all the needed state for a Vorbis encoder.
pub struct VorbisEncodingState {
	pub(crate) vorbis_info: VorbisInfo,
	pub(crate) vorbis_dsp_state: Box<vorbis_dsp_state>,
	pub(crate) vorbis_block: Box<vorbis_block>
}

impl VorbisEncodingState {
	/// Creates a new Vorbis encoder state from the specified Vorbis stream
	/// information, which should be initialized for encoding.
	pub fn new(mut vorbis_info: VorbisInfo) -> Result<Self, VorbisError> {
		// NOTE: stable-friendly version of Box::new_uninit
		let mut vorbis_dsp_state = Box::new(MaybeUninit::uninit());
		let mut vorbis_block = Box::new(MaybeUninit::uninit());

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
			let mut vorbis_dsp_state = assume_init_box(vorbis_dsp_state);

			libvorbis_return_value_to_result!(vorbis_block_init(
				&mut *vorbis_dsp_state,
				vorbis_block.as_mut_ptr()
			))?;

			Ok(Self {
				vorbis_info,
				vorbis_dsp_state,
				vorbis_block: assume_init_box(vorbis_block)
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
