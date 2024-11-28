#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// This `extern crate` is required for the linker to honor the linker flags `ogg_next_sys` sets on
// its build script, and therefore make the Ogg library symbols available at runtime. See:
// https://doc.rust-lang.org/nightly/reference/items/extern-crates.html#r-items.extern-crate.lookup
extern crate ogg_next_sys;

use ogg_next_sys::*;

include!("bindings.rs");

#[cfg(test)]
mod test {
	use std::{
		ffi::CStr,
		io::{Cursor, Read, Write},
		mem::MaybeUninit,
		os::raw::{c_int, c_long, c_void},
		ptr, slice
	};

	use ogg_next_sys::{ogg_stream_init, ogg_stream_packetin};

	use super::*;

	#[test]
	fn roundtrip_decode_and_encode() {
		let mut input_data =
			Cursor::new(&include_bytes!("8khz_500ms_mono_400hz_sine_wave.ogg")[..]);
		let mut output_buffer = Cursor::new(vec![]);

		// This example is hacked together by translating the encoder_example.c libvorbis
		// example to Rust. It does not do proper error handling, and is not intended to be
		// a proper usage example. We just want to make sure that the bindings work
		unsafe {
			// Tear-up decoding

			let mut ogg_vorbis_file = MaybeUninit::uninit();
			assert_eq!(
				ov_open_callbacks(
					&mut input_data as *mut Cursor<&[u8]> as *mut c_void,
					ogg_vorbis_file.as_mut_ptr(),
					ptr::null(),
					0,
					ov_callbacks {
						read_func: {
							// This read callback should match the stdio fread behavior.
							// See: https://man7.org/linux/man-pages/man3/fread.3.html
							unsafe extern "C" fn read_func(
								ptr: *mut std::ffi::c_void,
								size: usize,
								count: usize,
								datasource: *mut std::ffi::c_void
							) -> usize {
								let data = &mut *(datasource as *mut Cursor<&[u8]>);
								let buf = slice::from_raw_parts_mut(ptr as *mut u8, size * count);
								match data.read(buf) {
									Ok(n) => n / size,
									Err(_) => 0
								}
							}
							Some(read_func)
						},
						seek_func: None,
						close_func: None,
						tell_func: None
					}
				),
				0
			);
			let ogg_vorbis_file = ogg_vorbis_file.assume_init_mut();

			let ogg_vorbis_info = &*ov_info(ogg_vorbis_file, -1);
			eprintln!(
				"Encoder version string: {}",
				CStr::from_ptr(vorbis_version_string())
					.to_str()
					.unwrap_or_default()
			);
			eprintln!("Bitrate: {}", ov_bitrate(ogg_vorbis_file, -1));
			eprintln!("Channels: {}", ogg_vorbis_info.channels);
			eprintln!("Sampling frequency: {}", ogg_vorbis_info.rate);

			// Tear-up encode with libvorbisenc and libvorbis

			let mut vorbis_info = MaybeUninit::uninit();
			vorbis_info_init(vorbis_info.as_mut_ptr());
			let vorbis_info = vorbis_info.assume_init_mut();

			assert_eq!(
				// Pure VBR chosen by quality factor, no bitrate management engine
				vorbis_encode_init_vbr(
					vorbis_info,
					ogg_vorbis_info.channels as c_long,
					ogg_vorbis_info.rate,
					-0.2 // The worst possible quality for aoTuV for the smallest size
				),
				0
			);

			let mut vorbis_dsp_state = MaybeUninit::uninit();
			assert_eq!(
				vorbis_analysis_init(vorbis_dsp_state.as_mut_ptr(), vorbis_info),
				0
			);
			let vorbis_dsp_state = vorbis_dsp_state.assume_init_mut();

			let mut vorbis_block = MaybeUninit::uninit();
			assert_eq!(
				vorbis_block_init(vorbis_dsp_state, vorbis_block.as_mut_ptr()),
				0
			);
			let vorbis_block = vorbis_block.assume_init_mut();

			let mut vorbis_comment = MaybeUninit::uninit();
			vorbis_comment_init(vorbis_comment.as_mut_ptr());
			let vorbis_comment = vorbis_comment.assume_init_mut();

			// Generate header packets

			let mut vorbis_identification_packet = MaybeUninit::uninit();
			let mut vorbis_comment_packet = MaybeUninit::uninit();
			let mut vorbis_setup_packet = MaybeUninit::uninit();
			assert_eq!(
				vorbis_analysis_headerout(
					vorbis_dsp_state,
					vorbis_comment,
					vorbis_identification_packet.as_mut_ptr(),
					vorbis_comment_packet.as_mut_ptr(),
					vorbis_setup_packet.as_mut_ptr()
				),
				0
			);
			let vorbis_identification_packet = vorbis_identification_packet.assume_init_mut();
			let vorbis_comment_packet = vorbis_comment_packet.assume_init_mut();
			let vorbis_setup_packet = vorbis_setup_packet.assume_init_mut();

			// Tear-up Ogg stream and write header packets

			let mut ogg_stream = MaybeUninit::uninit();
			assert_eq!(ogg_stream_init(ogg_stream.as_mut_ptr(), 0), 0);
			let ogg_stream = ogg_stream.assume_init_mut();

			assert_eq!(
				ogg_stream_packetin(ogg_stream, vorbis_identification_packet),
				0
			);
			assert_eq!(ogg_stream_packetin(ogg_stream, vorbis_comment_packet), 0);
			assert_eq!(ogg_stream_packetin(ogg_stream, vorbis_setup_packet), 0);

			let mut ogg_page = MaybeUninit::uninit();
			loop {
				if ogg_stream_flush(ogg_stream, ogg_page.as_mut_ptr()) == 0 {
					break;
				}
				let ogg_page = ogg_page.assume_init_mut();

				// Write final page to the output
				output_buffer
					.write_all(slice::from_raw_parts(
						ogg_page.header,
						ogg_page.header_len as usize
					))
					.unwrap();
				output_buffer
					.write_all(slice::from_raw_parts(
						ogg_page.body,
						ogg_page.body_len as usize
					))
					.unwrap();
			}
			let ogg_page = ogg_page.assume_init_mut();

			// Encode loop

			let mut sample_source_buf = MaybeUninit::uninit();
			let mut bitstream_id = MaybeUninit::uninit();
			let mut previous_bitstream_id = None;

			loop {
				let samples_read = ov_read_float(
					ogg_vorbis_file,
					sample_source_buf.as_mut_ptr(),
					1024,
					bitstream_id.as_mut_ptr()
				);
				assert!(samples_read >= 0);
				let sample_source_buf = *sample_source_buf.assume_init_mut();

				let bitstream_id = *bitstream_id.assume_init_mut();
				assert!(
					previous_bitstream_id.is_none() || previous_bitstream_id == Some(bitstream_id),
					"Chained Ogg Vorbis files are not supported"
				);
				previous_bitstream_id = Some(bitstream_id);

				if samples_read == 0 {
					// Signal EOS
					assert_eq!(vorbis_analysis_wrote(vorbis_dsp_state, 0), 0);
				} else {
					let sample_sink_buf =
						vorbis_analysis_buffer(vorbis_dsp_state, samples_read as c_int);

					// Copy the samples for each input channel to each output channel
					for i in 0..ogg_vorbis_info.channels {
						(*sample_source_buf.offset(i as isize)).copy_to_nonoverlapping(
							*sample_sink_buf.offset(i as isize),
							samples_read as usize
						);
					}

					assert_eq!(
						vorbis_analysis_wrote(vorbis_dsp_state, samples_read as c_int),
						0
					);
				}

				// Poll for Vorbis audio blocks in packets. Write those packets to Ogg pages
				loop {
					let blockout_result = vorbis_analysis_blockout(vorbis_dsp_state, vorbis_block);
					assert!(blockout_result >= 0);
					if blockout_result != 1 {
						break;
					}

					assert_eq!(vorbis_analysis(vorbis_block, ptr::null_mut()), 0);
					assert_eq!(vorbis_bitrate_addblock(vorbis_block), 0);

					loop {
						let mut ogg_packet = MaybeUninit::uninit();
						let flushpacket_result =
							vorbis_bitrate_flushpacket(vorbis_dsp_state, ogg_packet.as_mut_ptr());
						assert!(flushpacket_result >= 0);
						if flushpacket_result != 1 {
							break;
						}

						ogg_stream_packetin(ogg_stream, ogg_packet.assume_init_mut());
						if ogg_stream_pageout(ogg_stream, ogg_page) != 0 {
							output_buffer
								.write_all(slice::from_raw_parts(
									ogg_page.header,
									ogg_page.header_len as usize
								))
								.unwrap();
							output_buffer
								.write_all(slice::from_raw_parts(
									ogg_page.body,
									ogg_page.body_len as usize
								))
								.unwrap();
						}
					}
				}

				if samples_read == 0 {
					break;
				}
			}

			// libogg cleanup
			ogg_stream_clear(ogg_stream);

			// libvorbis and libvorbisenc cleanup
			vorbis_dsp_clear(vorbis_dsp_state);
			vorbis_block_clear(vorbis_block);
			vorbis_comment_clear(vorbis_comment);
			vorbis_info_clear(vorbis_info);

			// libvorbisfile cleanup
			ov_clear(ogg_vorbis_file);
		}

		// Weak sanity check for the output buffer to make sense
		assert_eq!(&output_buffer.into_inner()[..4], b"OggS");
	}
}
