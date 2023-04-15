use core::slice;
use std::{
	ffi::c_void,
	io::Read,
	marker::PhantomData,
	mem::MaybeUninit,
	num::{NonZeroU32, NonZeroU8},
	os::raw::c_int,
	ptr
};

use aotuv_lancer_vorbis_sys::{
	ov_callbacks, ov_clear, ov_open_callbacks, ov_read_float, OggVorbis_File
};
use errno::{set_errno, Errno};

use crate::{common::VorbisError, decoder::VorbisAudioSamples};

/// A decoder that turns a perceptually-encoded, non-chained Ogg Vorbis stream into
/// blocks of planar, single-precision float audio samples.
pub struct VorbisDecoder<R: Read> {
	ogg_vorbis_file: OggVorbis_File,
	source: PhantomData<R>,
	last_audio_block: Option<VorbisAudioSamples>
}

impl<R: Read> VorbisDecoder<R> {
	/// Creates a new Vorbis decoder that will read an Ogg Vorbis stream from the
	/// specified source.
	///
	/// This method reads Vorbis header data from the source, validating it as an
	/// Ogg Vorbis stream and setting up internal data structures for audio decode.
	/// I/O errors that might happen during that operation will be returned to the
	/// caller.
	pub fn new<S: Into<Box<R>>>(source: S) -> Result<Self, VorbisError> {
		let mut ogg_vorbis_file = MaybeUninit::uninit();

		// The source read needs to be allocated in the heap (i.e., boxed) to have a
		// constant memory address. Then leak it to a raw pointer to hand its ownership
		// over to C code. Related, interesting read about trait objects and FFI:
		// https://adventures.michaelfbryan.com/posts/ffi-safe-polymorphism-in-rust/
		let source = Box::into_raw(source.into());

		// SAFETY: we assume ov_open_callbacks follows its documented contract
		unsafe {
			match vorbisfile_return_value_to_result!(ov_open_callbacks(
				source as *mut c_void,
				ogg_vorbis_file.as_mut_ptr(),
				ptr::null(),
				0,
				ov_callbacks {
					read_func: {
						// This read callback should match the stdio fread behavior.
						// See: https://man7.org/linux/man-pages/man3/fread.3p.html
						unsafe extern "C" fn read_func<R: Read>(
							ptr: *mut c_void,
							size: usize,
							count: usize,
							datasource: *mut c_void
						) -> usize {
							let source = &mut *(datasource as *mut R);
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
						Some(read_func::<R>)
					},
					seek_func: None,
					close_func: {
						unsafe extern "C" fn close_func<R: Read>(datasource: *mut c_void) -> c_int {
							// Drop the Read when it's no longer needed by vorbisfile.
							// This is called by ov_clear
							drop(Box::from_raw(datasource as *mut R));

							0
						}
						Some(close_func::<R>)
					},
					tell_func: None
				}
			)) {
				Ok(_) => Ok(Self {
					ogg_vorbis_file: ogg_vorbis_file.assume_init(),
					source: PhantomData,
					last_audio_block: None
				}),
				Err(err) => {
					// According to the documented contract for ov_open_callbacks, the
					// application is responsible for cleaning up the data source on
					// failure. This is reiterated in the docs for OggVorbis_File
					drop(Box::from_raw(source as *mut R));
					Err(err)
				}
			}
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
				&mut self.ogg_vorbis_file,
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

				Ok(self.last_audio_block.as_ref())
			} else {
				Ok(None)
			}
		}
	}

	/// Returns the number of channels of the audio signal decoded by this decoder.
	pub fn channels(&self) -> NonZeroU8 {
		// SAFETY: as per the Vorbis I specification, the channel count must be greater than
		// zero and at most 255
		unsafe { NonZeroU8::new_unchecked((*self.ogg_vorbis_file.vi).channels as u8) }
	}

	/// Returns the sampling frequency of the audio signal decoded by this decoder.
	pub fn sampling_frequency(&self) -> NonZeroU32 {
		// SAFETY: as per the Vorbis I specification, the sampling frequency must be greater
		// than zero and fit in an unsigned 32-bit integer
		unsafe { NonZeroU32::new_unchecked((*self.ogg_vorbis_file.vi).rate as u32) }
	}
}

impl<R: Read> Drop for VorbisDecoder<R> {
	fn drop(&mut self) {
		unsafe { ov_clear(&mut self.ogg_vorbis_file) };
	}
}

#[cfg(test)]
mod test {
	use std::io::{self, ErrorKind, Read};

	use super::VorbisDecoder;
	use crate::{VorbisError, VorbisLibraryError, VorbisLibraryErrorKind};

	#[test]
	fn decoder_handles_io_failures() {
		struct ErrorRead;

		impl Read for ErrorRead {
			fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
				Err(ErrorKind::Other.into())
			}
		}

		let result = VorbisDecoder::new(ErrorRead);

		assert!(
			matches!(
				result,
				Err(VorbisError::LibraryError(VorbisLibraryError {
					// libvorbis may return a slightly different error kind depending on the
					// platform: Unix-like systems yield an I/O error, while Windows yields
					// OV_ENOTVORBIS. Both error kinds make sense, and we're not trying to
					// fix upstream code here, so accept both
					kind: VorbisLibraryErrorKind::Io | VorbisLibraryErrorKind::NotVorbis,
					..
				}))
			),
			"Expected I/O library error, got: {:?}",
			result.err()
		);
	}
}
