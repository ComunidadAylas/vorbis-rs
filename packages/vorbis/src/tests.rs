use super::*;
use std::io::Cursor;

#[test]
fn reencoding_works() {
	let mut source_ogg_data =
		&include_bytes!("../../vorbis_sys/src/8khz_500ms_mono_400hz_sine_wave.ogg")[..];
	let mut encoded_ogg_data = vec![];

	// Decode and recode the input file
	let mut decoder =
		VorbisDecoder::new(&mut source_ogg_data).expect("Error instantiating decoder");
	let mut encoder = VorbisEncoder::new(
		0,
		[("", ""); 0],
		decoder.sampling_frequency(),
		decoder.channels(),
		VorbisBitrateManagementStrategy::Vbr {
			target_bitrate: NonZeroU32::new(32000).unwrap()
		},
		None,
		&mut encoded_ogg_data
	)
	.expect("Error instantiating encoder");

	while let Some(decoded_block) = decoder.decode_audio_block().expect("Decode error") {
		encoder
			.encode_audio_block(decoded_block.samples())
			.expect("Encode error");
	}

	encoder.finish().expect("Error finalizing encoding");

	// Decode the generated input as a sanity check
	let mut decoder =
		VorbisDecoder::new(Cursor::new(encoded_ogg_data)).expect("Error instantiating decoder");

	while decoder
		.decode_audio_block()
		.expect("Decode error on encoded data")
		.is_some()
	{}
}
