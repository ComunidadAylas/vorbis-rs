//! Tests for ensuring past issues are kept fixed.
use std::{
	mem::{self, MaybeUninit},
	num::{NonZeroU8, NonZeroU32},
	slice
};

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

#[doc = concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/35")]
#[test]
fn issue_35() {
	// To reliably showcase this issue about use-after-free due to a move, we need to store the
	// decoder at a memory address where we can loudly invalidate any reads made to the address it
	// was at before the move. A stack-allocated variable is inconvenient for this because the stack
	// frames of any called function stay valid while they are part of the call stack, and the
	// specifics of how their storage gets reused for new stack frames or variables is
	// platform-specific. Therefore, we use a heap allocation through a `Vec` (see comments below
	// for why this instead of, e.g., a `Box`)
	let mut heap_decoder_buf = vec![
		VorbisDecoder::<&[u8]>::new(
			&include_bytes!(
				"../../aotuv_lancer_vorbis_sys/src/8khz_500ms_mono_400hz_sine_wave.ogg"
			)[..]
		)
		.unwrap(),
	];

	// Ensure all decoder state is initialized, which includes pointers in C structs
	heap_decoder_buf[0].decode_audio_block().unwrap();

	// Now that we have a fully initialized decoder on a heap-backed `Vec`, move it out of such `Vec`.
	// Per Rust's move semantics, this effectively `memcpy`s the decoder to a memory address
	// somewhere on the stack, rendering the previous heap address it was located at free for use by
	// any other `Vec` elements, or released back to the OS if the backing buffer is freed or
	// reallocated. Because we don't do the later by this point, the `Vec` buffer has capacity
	// allocated and likely storing a copy of the single element we had in it (this would not
	// necessarily be the case with a `Box`), so we can alter the memory representation of that copy
	// through spare `Vec` capacity accessors (the `unsafe` block that follows is a backport to
	// stable Rust of nightly Rust's MaybeUninit::as_bytes_mut`) to garbage bytes. In turn, the
	// garbage bytes chosen practically guarantee that libvorbis attempts to reference invalid
	// memory when any of its internal structs still refer to the freed, stale copy of the struct.
	// TODO: replace the `unsafe` below with `MaybeUninit::as_bytes_mut` once it gets stabilized
	let mut stack_decoder = heap_decoder_buf.remove(0);
	for byte in unsafe {
		slice::from_raw_parts_mut(
			heap_decoder_buf.spare_capacity_mut()[0]
				.as_mut_ptr()
				.cast::<MaybeUninit<u8>>(),
			mem::size_of::<VorbisDecoder<&[u8]>>()
		)
	} {
		byte.write(0xCA); // Neither 0 nor 0xFF to avoid interpretation as sentinel values
	}

	// After the setup above, when affected by the use-after-free due to a move bug this will reliably
	// cause an invalid memory address to be accessed, aborting the process with SIGSEGV on
	// Unix-like systems or he STATUS_ACCESS_VIOLATION structured exception on Windows, and fail the
	// test
	stack_decoder.decode_audio_block().ok();
}
