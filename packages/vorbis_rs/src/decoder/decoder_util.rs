use std::slice;

use tinyvec::{TinyVec, tiny_vec};

#[allow(unused_imports)]
use crate::VorbisDecoder;

/// Contains a block of audio samples. This struct is returned by a [`VorbisDecoder`].
pub struct VorbisAudioSamples {
	// This static lifetime is not really accurate. See safety comments below for more details
	audio_samples: TinyVec<[&'static [f32]; 8]>
}

impl VorbisAudioSamples {
	/// Creates a new block of audio samples.
	///
	/// # Safety
	/// This function assumes that it is called by trusted code on [`VorbisDecoder`] only.
	/// For more information, please read the safety comment in its source code.
	pub(in crate::decoder) fn new(
		buf: *mut *mut f32,
		channels: usize,
		samples_read: usize
	) -> Self {
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
		let mut audio_samples = tiny_vec!([&[f32]; 8]);
		unsafe {
			audio_samples.extend(
				slice::from_raw_parts(buf, channels)
					.iter()
					.map(|channel_samples| slice::from_raw_parts(*channel_samples, samples_read))
			);
		}

		Self { audio_samples }
	}

	/// Returns a reference to the buffer of planar, single-precision float samples held
	/// by this audio block.
	pub fn samples(&self) -> &[&[f32]] {
		&self.audio_samples
	}
}
