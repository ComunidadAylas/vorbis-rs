//! Tests for ensuring past issues are kept fixed.

use std::num::{NonZeroU8, NonZeroU32};

use vorbis_rs::{VorbisDecoder, VorbisEncoderBuilder};

#[doc = concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/17")]
#[test]
fn issue_17() {
	let mut encoded_ogg = vec![];
	let mut encoder = VorbisEncoderBuilder::new(
		NonZeroU32::new(48000).unwrap(),
		NonZeroU8::new(1).unwrap(),
		&mut encoded_ogg
	)
	.unwrap()
	.build()
	.unwrap();

	let lots_of_samples = vec![0.; 48000 * 100]; // 100 s of audio
	let mut expected_samples_count = 0;

	encoder.encode_audio_block([&lots_of_samples]).unwrap();
	expected_samples_count += lots_of_samples.len();

	encoder.encode_audio_block([&lots_of_samples]).unwrap();
	expected_samples_count += lots_of_samples.len();

	encoder
		.encode_audio_block([&lots_of_samples[..512]])
		.unwrap();
	expected_samples_count += 512;

	encoder
		.encode_audio_block([&lots_of_samples[..1024]])
		.unwrap();
	expected_samples_count += 1024;

	drop(encoder);

	let mut decoder = VorbisDecoder::<&_>::new(&*encoded_ogg).unwrap();

	let mut samples_count = 0;
	while let Some(decoded_block) = decoder.decode_audio_block().unwrap() {
		assert!(decoded_block.samples().iter().all(|samples| {
			samples.iter().all(|sample| {
				samples_count += 1;
				// Vorbis would be an awful codec if compressing silence was lossy, wouldn't it? :)
				*sample == 0.0
			})
		}));
	}

	assert_eq!(
		expected_samples_count, samples_count,
		"The number of encoded samples must match the number of decoded samples"
	);
}
