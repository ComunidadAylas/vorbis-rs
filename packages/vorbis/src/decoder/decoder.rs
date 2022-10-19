use core::slice;
use errno::{set_errno, Errno};
use std::ffi::c_void;
use std::io::Read;
use std::mem::MaybeUninit;
use std::num::{NonZeroU32, NonZeroU8};
use std::os::raw::c_int;
use std::ptr;

use vorbis_sys::{ov_callbacks, ov_clear, ov_open_callbacks, ov_read_float, OggVorbis_File};

use crate::common::{assume_init_box, VorbisError};
use crate::decoder::VorbisAudioSamples;
use crate::vorbis_error_code_to_string;

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
		// NOTE: stable-friendly version of Box::new_uninit
		let mut ogg_vorbis_file = Box::new(MaybeUninit::uninit());
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
				ogg_vorbis_file: assume_init_box(ogg_vorbis_file),
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
