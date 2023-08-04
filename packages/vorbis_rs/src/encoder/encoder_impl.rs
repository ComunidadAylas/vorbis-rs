use std::{
	borrow::Cow,
	io::Write,
	mem::{size_of, MaybeUninit},
	num::{NonZeroU32, NonZeroU8},
	ptr, slice
};

use aotuv_lancer_vorbis_sys::{
	vorbis_analysis, vorbis_analysis_blockout, vorbis_analysis_buffer, vorbis_analysis_wrote,
	vorbis_bitrate_addblock, vorbis_bitrate_flushpacket
};

use crate::{
	common::{OggPacket, OggStream, VorbisComments, VorbisError, VorbisInfo},
	encoder::{VorbisBitrateManagementStrategy, VorbisEncodingState}
};

/// Builds a [`VorbisEncoder`] with configurable Vorbis encoding and
/// Ogg stream encapsulation options.
pub struct VorbisEncoderBuilder<W: Write> {
	sampling_frequency: NonZeroU32,
	channels: NonZeroU8,
	sink: Option<W>,
	stream_serial: i32,
	bitrate_management_strategy: VorbisBitrateManagementStrategy,
	comments: VorbisComments,
	minimum_page_data_size: Option<u16>
}

impl<W: Write> VorbisEncoderBuilder<W> {
	/// Creates a new Vorbis encoder builder for a signal with the specified sampling frequency
	/// and channels that will be encoded to the specified sink.
	///
	/// The serial of the generated Ogg Vorbis stream will be randomly generated, as dictated
	/// by the Ogg specification. If this behavior is not desirable, please use
	/// [`new_with_serial`](Self::new_with_serial) instead.
	#[cfg(feature = "stream-serial-rng")]
	pub fn new(
		sampling_frequency: NonZeroU32,
		channels: NonZeroU8,
		sink: W
	) -> Result<Self, VorbisError> {
		// We use a RNG seeded with an unknown seed to minimize unintended metadata leakage:
		// if we set the seed to a value from a better-known source (e.g., the current timestamp),
		// an attacker could more practically make educated guesses about the seed value range
		// and the RNG algorithm used to bruteforce the seed value from the serial. Note that
		// the security in this scenario comes from how costly and unpredictable the RNG is,
		// not whether it's cryptographically-secure
		Ok(Self::_new(sampling_frequency, channels, sink, {
			let mut stream_serial_buf = [MaybeUninit::uninit(); size_of::<i32>()];
			i32::from_ne_bytes(
				getrandom::getrandom_uninit(&mut stream_serial_buf)?
					.try_into()
					.unwrap()
			)
		}))
	}

	/// Creates a new Vorbis encoder builder for a signal with the specified sampling frequency
	/// and channels that will be encoded to the specified sink. The serial of the generated Ogg
	/// Vorbis stream will be set to `stream_serial`.
	pub fn new_with_serial(
		sampling_frequency: NonZeroU32,
		channels: NonZeroU8,
		sink: W,
		stream_serial: i32
	) -> Self {
		Self::_new(sampling_frequency, channels, sink, stream_serial)
	}

	/// Common initialization code for [`VorbisEncoderBuilder`] constructors.
	fn _new(
		sampling_frequency: NonZeroU32,
		channels: NonZeroU8,
		sink: W,
		stream_serial: i32
	) -> Self {
		Self {
			sampling_frequency,
			channels,
			sink: Some(sink),
			stream_serial,
			bitrate_management_strategy: Default::default(),
			comments: VorbisComments::new(),
			minimum_page_data_size: None
		}
	}

	/// Sets the sampling frequency of the signal to encode, in Hertz (Hz).
	pub fn sampling_frequency(&mut self, sampling_frequency: NonZeroU32) -> &mut Self {
		self.sampling_frequency = sampling_frequency;
		self
	}

	/// Sets the number of channels of the signal to encode.
	pub fn channels(&mut self, channels: NonZeroU8) -> &mut Self {
		self.channels = channels;
		self
	}

	/// Sets the sink to encode the audio signal to.
	///
	/// It is necessary to set a sink to build an encoder after using this
	/// builder to build an encoder (i.e., after the
	/// [`build`](VorbisEncoderBuilder::build) method is called, before
	/// the next [`build`](VorbisEncoderBuilder::build) call).
	pub fn sink(&mut self, sink: W) -> &mut Self {
		self.sink = Some(sink);
		self
	}

	/// Sets the serial of the Ogg Vorbis stream to be generated.
	///
	/// The [Ogg specification] states that stream serials are to be
	/// randomly generated, but the only hard requirement is that they
	/// are unique within a physical bitstream (i.e., file). Failure to
	/// use random serials will not cause invalid streams to be generated,
	/// but it will be more difficult to [properly chain them by using
	/// generic tools such as `cat`](https://linux.die.net/man/1/oggz-merge),
	/// as serial number collisions will be much more likely to occur.
	///
	/// [Ogg specification]: https://www.xiph.org/ogg/doc/rfc3533.txt
	pub fn stream_serial(&mut self, stream_serial: i32) -> &mut Self {
		self.stream_serial = stream_serial;
		self
	}

	/// Sets the bitrate management strategy to use, determining the tradeoff
	/// between audio quality and stream size and bandwidth requirements.
	pub fn bitrate_management_strategy(
		&mut self,
		bitrate_management_strategy: VorbisBitrateManagementStrategy
	) -> &mut Self {
		self.bitrate_management_strategy = bitrate_management_strategy;
		self
	}

	/// Adds a single comment tag to the Vorbis comments header that will be
	/// generated.
	///
	/// By default, no Vorbis comments are generated.
	pub fn comment_tag<'tag, 'value>(
		&mut self,
		tag: impl Into<Cow<'tag, str>>,
		value: impl Into<Cow<'value, str>>
	) -> Result<&mut Self, VorbisError> {
		self.comment_tags([(tag, value)])
	}

	/// Adds several comment tags to the Vorbis comments header that will be
	/// generated.
	///
	/// By default, no Vorbis comments are generated.
	pub fn comment_tags<'tag, 'value, T: Into<Cow<'tag, str>>, V: Into<Cow<'value, str>>>(
		&mut self,
		tags: impl IntoIterator<Item = (T, V)>
	) -> Result<&mut Self, VorbisError> {
		for (tag, value) in tags {
			self.comments.add_tag(tag, value)?;
		}

		Ok(self)
	}

	/// Specifies the minimum size of Vorbis stream data to put into each Ogg page, except
	/// for some header pages, which have to be cut short to conform to the Ogg Vorbis
	/// specification.
	///
	/// This value controls the tradeoff between Ogg encapsulation overhead and ease of
	/// seeking and packet loss concealment. By default, it is set to `None`, which lets
	/// the encoder decide.
	pub fn minimum_page_data_size(&mut self, minimum_page_data_size: Option<u16>) -> &mut Self {
		self.minimum_page_data_size = minimum_page_data_size;
		self
	}

	/// Creates the configured [`VorbisEncoder`], validating all the parameters and writing
	/// header data to the specified sink. Errors may be returned when either the parameters
	/// are invalid or an I/O failure happens.
	///
	/// The sink this builder was configured with will be consumed, so you must set up a new
	/// one via the [`sink`](Self::sink) method if you intend to continue building encoders
	/// with this builder. Failure to do so will cause errors to be returned.
	pub fn build(&mut self) -> Result<VorbisEncoder<W>, VorbisError> {
		let mut sink = self
			.sink
			.take()
			.ok_or(VorbisError::ConsumedEncoderBuilderSink)?;

		// Tear up the Ogg stream
		let mut ogg_stream = OggStream::new(self.stream_serial)?;

		// Tear up the Vorbis encoder
		let mut vorbis_info = VorbisInfo::new();
		match self.bitrate_management_strategy {
			VorbisBitrateManagementStrategy::Vbr { target_bitrate } => {
				vorbis_info.encode_init_vbr(self.sampling_frequency, self.channels, target_bitrate)
			}
			VorbisBitrateManagementStrategy::QualityVbr { target_quality } => vorbis_info
				.encode_init_quality_vbr(self.sampling_frequency, self.channels, target_quality),
			VorbisBitrateManagementStrategy::Abr { average_bitrate } => {
				vorbis_info.encode_init_abr(self.sampling_frequency, self.channels, average_bitrate)
			}
			VorbisBitrateManagementStrategy::ConstrainedAbr { maximum_bitrate } => vorbis_info
				.encode_init_constrained_abr(
					self.sampling_frequency,
					self.channels,
					maximum_bitrate
				)
		}?;

		let mut vorbis_encoding_state = VorbisEncodingState::new(vorbis_info)?;

		// Get the Vorbis header packets and submit them for encapsulation
		for mut header_packet in vorbis_encoding_state.get_header_packets(&mut self.comments)? {
			header_packet.submit(&mut ogg_stream)?;
		}

		// Force the header packets we submitted to be written, and the first audio packet to begin
		// on its own page, as mandated by the Vorbis I spec
		ogg_stream.flush(&mut sink)?;

		Ok(VorbisEncoder {
			ogg_stream,
			vorbis_encoding_state,
			sink: Some(sink),
			minimum_page_data_size: self.minimum_page_data_size
		})
	}
}

/// An encoder that transforms blocks of planar, single-precision float audio
/// samples to a perceptually-encoded Ogg Vorbis stream. Instances of this
/// encoder can be obtained from a [`VorbisEncoderBuilder`].
pub struct VorbisEncoder<W: Write> {
	ogg_stream: OggStream,
	vorbis_encoding_state: VorbisEncodingState,
	sink: Option<W>,
	minimum_page_data_size: Option<u16>
}

impl<W: Write> VorbisEncoder<W> {
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
	/// to Ogg pages as they become available. This method expects that `self.sink`
	/// is `Some` (i.e., `finish` was not run).
	fn write_pending_blocks(&mut self) -> Result<(), VorbisError> {
		let sink = self.sink.as_mut().unwrap();

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

					self.ogg_stream
						.write_pending_pages(&mut *sink, self.minimum_page_data_size)?;
				}
			}
		}

		Ok(())
	}

	/// Finishes the encoded Vorbis stream, signalling its end and writing any pending
	/// data to the configured sink.
	///
	/// This is automatically done when the encoder is dropped, but calling `finish`
	/// explicitly is necessary for handling errors on finish explicitly.
	///
	/// Returns the owned sink back to the caller.
	pub fn finish(mut self) -> Result<W, VorbisError> {
		// SAFETY: we assume that vorbis_analysis_wrote follows its documented contract
		unsafe {
			libvorbis_return_value_to_result!(vorbis_analysis_wrote(
				&mut *self.vorbis_encoding_state.vorbis_dsp_state,
				0
			))?
		};

		self.write_pending_blocks()
			.map(|_| self.sink.take().unwrap())
	}
}

impl<W: Write> Drop for VorbisEncoder<W> {
	fn drop(&mut self) {
		// Finishing the stream twice generates two EOS packets, which is illegal,
		// so check whether we have a sink to finish: the sink is set to None if
		// and only if the stream was finished explicitly
		if self.sink.is_some() {
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
