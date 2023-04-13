use crate::common::{OggPacket, OggStream, VorbisComments, VorbisError, VorbisInfo};
use crate::encoder::{VorbisBitrateManagementStrategy, VorbisEncodingState};

use std::borrow::Cow;
use std::io::Write;
use std::mem::MaybeUninit;
use std::num::{NonZeroU32, NonZeroU8};
use std::{ptr, slice};

use aotuv_lancer_vorbis_sys::{
	vorbis_analysis, vorbis_analysis_blockout, vorbis_analysis_buffer, vorbis_analysis_wrote,
	vorbis_bitrate_addblock, vorbis_bitrate_flushpacket
};

/// An encoder that transforms blocks of planar, single-precision float audio
/// samples to a perceptually-encoded Ogg Vorbis stream.
pub struct VorbisEncoder<W: Write> {
	ogg_stream: OggStream,
	vorbis_encoding_state: VorbisEncodingState,
	sink: Option<W>,
	minimum_page_data_size: Option<u16>,
	finished: bool
}

impl<W: Write> VorbisEncoder<W> {
	/// Creates a new Vorbis encoder with the specified metadata, for the
	/// specified signal, using some bitrate management strategy and minimum
	/// Ogg page size, that will write data to the provided sink.
	///
	/// This method validates input parameters, initializes the encoder, and
	/// writes header data to the specified sink. Therefore, it may fail even
	/// if the input parameters are valid due to I/O errors.
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
			sink: Some(sink),
			minimum_page_data_size,
			finished: false
		})
	}

	/// Submits the specified audio block for encoding by Vorbis. Encoded data will be written
	/// to the configured sink automatically as it becomes available.
	///
	/// The audio block is expected to be in planar format (i.e., one vector of samples per
	/// channel). The order of channels is defined by the Vorbis I specification.
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
			libvorbis_return_value_to_result!(vorbis_analysis_wrote(
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
					OggPacket::new(ogg_packet.assume_init()).submit(&mut self.ogg_stream)?;

					let sink = self.sink.as_mut().expect("sink while decoder is alive");
					self.ogg_stream
						.write_pending_pages(sink, self.minimum_page_data_size)?;
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
	///
	/// Returns the owned writer back to the caller.
	pub fn finish(mut self) -> Result<W, VorbisError> {
		// SAFETY: we assume that vorbis_analysis_wrote follows its documented contract
		unsafe {
			libvorbis_return_value_to_result!(vorbis_analysis_wrote(
				&mut *self.vorbis_encoding_state.vorbis_dsp_state,
				0
			))?
		};

		let write_pending_result = self.write_pending_blocks();
		self.finished = true; // Avoids implicit finish on drop
		let sink = self.sink.take().unwrap();
		write_pending_result.map(move |_| sink)
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
