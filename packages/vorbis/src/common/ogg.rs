use ogg_sys::{
	ogg_packet, ogg_page, ogg_stream_clear, ogg_stream_flush, ogg_stream_init, ogg_stream_packetin,
	ogg_stream_pageout, ogg_stream_pageout_fill, ogg_stream_state
};

use std::io::Write;
use std::mem::MaybeUninit;
use std::slice;

use crate::common::vorbis_error_code_to_string;
use crate::VorbisError;

/// A high-level abstraction for an Ogg stream.
pub struct OggStream {
	ogg_stream: ogg_stream_state
}

impl OggStream {
	/// Creates a new Ogg stream with the specified serial number.
	pub fn new(stream_serial: i32) -> Result<Self, VorbisError> {
		let mut ogg_stream = MaybeUninit::uninit();

		// SAFETY: we assume ogg_stream_init follows its documented contract,
		// and that C code does not assume that the struct stays at the same
		// memory location
		unsafe {
			libogg_return_value_to_result!(ogg_stream_init(
				ogg_stream.as_mut_ptr(),
				stream_serial
			))?;

			Ok(Self {
				ogg_stream: ogg_stream.assume_init()
			})
		}
	}

	/// Pulls the pending pages to write to an Ogg stream, and then writes them
	/// out to the specified sink. Optionally, it waits until there are at least
	/// `minimum_page_data_size` packet bytes to write or the page size limit
	/// is reached before yielding any pages.
	///
	/// This is a facade for the `ogg_stream_pageout` and `ogg_stream_pageout_fill`
	/// functions.
	pub fn write_pending_pages<W: Write>(
		&mut self,
		mut sink: W,
		minimum_page_data_size: Option<u16>
	) -> Result<(), VorbisError> {
		let mut ogg_page = MaybeUninit::uninit();

		loop {
			// SAFETY: we assume ogg_stream_pageout and ogg_stream_pageout_fill follow
			// their documented contract
			unsafe {
				let no_pending_page = if let Some(minimum_page_data_size) = minimum_page_data_size {
					ogg_stream_pageout_fill(
						&mut self.ogg_stream,
						ogg_page.as_mut_ptr(),
						minimum_page_data_size as i32
					)
				} else {
					ogg_stream_pageout(&mut self.ogg_stream, ogg_page.as_mut_ptr())
				} == 0;

				if no_pending_page {
					break;
				}

				OggPage {
					ogg_page: ogg_page.assume_init()
				}
				.write(&mut sink)?;
			}
		}

		Ok(())
	}

	/// Ends the current Ogg page and writes it to the stream immediately, if possible,
	/// even if little or no packet data was submitted since the last page.
	///
	/// This is a facade for the `ogg_stream_flush` function.
	pub fn flush<W: Write>(&mut self, mut sink: W) -> Result<(), VorbisError> {
		let mut ogg_page = MaybeUninit::uninit();

		// SAFETY: we assume ogg_stream_flush follows its documented contract
		unsafe {
			while ogg_stream_flush(&mut self.ogg_stream, ogg_page.as_mut_ptr()) != 0 {
				OggPage {
					ogg_page: ogg_page.assume_init()
				}
				.write(&mut sink)?;
			}
		}

		Ok(())
	}
}

impl Drop for OggStream {
	fn drop(&mut self) {
		// SAFETY: when this struct is dropped we have a valid Ogg stream to clear,
		// and there are no references to it
		unsafe { ogg_stream_clear(&mut self.ogg_stream) };
	}
}

/// A high-level abstraction for an Ogg packet.
pub struct OggPacket {
	ogg_packet: ogg_packet
}

impl OggPacket {
	/// Creates a new Ogg packet.
	pub fn new(ogg_packet: ogg_packet) -> Self {
		OggPacket { ogg_packet }
	}

	/// Submits this Ogg packet to be written to the specified Ogg stream. The
	/// packet won't actually be written anywhere until
	/// [its pending pages are written](OggStream::write_pending_pages) or
	/// [flushed](OggStream::flush).
	///
	/// This is a facade for the `ogg_stream_packetin` function.
	pub fn submit(&mut self, ogg_stream: &mut OggStream) -> Result<(), VorbisError> {
		// SAFETY: we assume ogg_stream_packetin follows its documented contract
		unsafe {
			libogg_return_value_to_result!(ogg_stream_packetin(
				&mut ogg_stream.ogg_stream,
				&mut self.ogg_packet
			))?;
		}

		Ok(())
	}
}

/// A high-level abstraction for an Ogg page.
pub struct OggPage {
	ogg_page: ogg_page
}

impl OggPage {
	/// Writes this Ogg page to the specified sink.
	fn write<W: Write>(&self, mut sink: W) -> Result<(), VorbisError> {
		// SAFETY: we reinterpret an initialized, aligned C pointer that outlives
		// this stack frame with a buffer length to a Rust slice for read-only
		// operations, which is safe. header_len is guaranteed by the Ogg spec
		// to be at most 252, which can be safely casted to a usize. body_len is
		// also guaranteed by spec to fit in an unsigned 16 bits integer, and
		// usize can be assumed to always be at least 16 bits wide
		unsafe {
			sink.write_all(slice::from_raw_parts(
				self.ogg_page.header,
				self.ogg_page.header_len as usize
			))?;
			sink.write_all(slice::from_raw_parts(
				self.ogg_page.body,
				self.ogg_page.body_len as usize
			))?;
		}

		Ok(())
	}
}
