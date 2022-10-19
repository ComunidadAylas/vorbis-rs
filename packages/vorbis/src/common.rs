use std::mem::MaybeUninit;

#[macro_use]
mod error;
pub use error::VorbisError;

mod ogg_data;
pub use ogg_data::{OggPacket, OggPage, OggStream};

mod vorbis_data;
pub use vorbis_data::{VorbisComments, VorbisInfo};

// taken from stdlib's assume_init implementation for Box
pub unsafe fn assume_init_box<T>(boxed: Box<MaybeUninit<T>>) -> Box<T> {
	Box::from_raw(Box::into_raw(boxed) as *mut T)
}
