use std::mem::MaybeUninit;

#[macro_use]
mod error;
pub use error::VorbisError;

mod ogg;
pub use ogg::{OggPacket, OggStream};

mod vorbis;
// self:: must be used in order to clear up ambiguity
pub use self::vorbis::{VorbisComments, VorbisInfo};

/// Converts to `Box<T>`.
///
/// # Safety
/// As with [`MaybeUninit::assume_init`], it is up to the caller to guarantee that the value
/// really is in an initialized state. Calling this when the content is not yet fully
/// initialized causes immediate undefined behavior.
///
/// [`MaybeUninit::assume_init`]: mem::MaybeUninit::assume_init
///
/// # Highlights
/// This function is a backport of the unstable `Box::assume_init` method to stable Rust,
/// tracked by [issue 63291](https://github.com/rust-lang/rust/issues/63291).
// FIXME refactor this out once Box::assume_init is made stable, if ever
pub unsafe fn assume_init_box<T>(boxed: Box<MaybeUninit<T>>) -> Box<T> {
	Box::from_raw(Box::into_raw(boxed) as *mut T)
}

use vorbis_sys::{
	OV_EBADHEADER, OV_EBADLINK, OV_EBADPACKET, OV_EFAULT, OV_EIMPL, OV_EINVAL, OV_ENOSEEK,
	OV_ENOTAUDIO, OV_ENOTVORBIS, OV_EOF, OV_EREAD, OV_EVERSION, OV_FALSE, OV_HOLE
};

/// Converts a status code returned by a bound C function to a more user-friendly string representation.
pub(crate) const fn vorbis_error_code_to_string(error_code: i32) -> &'static str {
	// List from
	// https://github.com/xiph/vorbis/blob/84c023699cdf023a32fa4ded32019f194afcdad0/include/vorbis/codec.h#L221-L235
	match error_code {
		OV_FALSE => "OV_FALSE",
		OV_EOF => "OV_EOF (stream at end of file)",
		OV_HOLE => "OV_HOLE (data interruption. Try repairing the file)",
		OV_EREAD => "OV_EREAD (I/O error)",
		OV_EFAULT => "OV_EFAULT (internal error)",
		OV_EIMPL => "OV_EIMPL (not implemented. Maybe try other parameters?)",
		OV_EINVAL => "OV_EINVAL (invalid parameter)",
		OV_ENOTVORBIS => "OV_ENOTVORBIS (not Vorbis data)",
		OV_EBADHEADER => "OV_EBADHEADER (invalid Vorbis stream header)",
		OV_EVERSION => "OV_EVERSION (Vorbis version mismatch)",
		OV_ENOTAUDIO => "OV_ENOTAUDIO (not audio data)",
		OV_EBADPACKET => "OV_EBADPACKET (invalid packet)",
		OV_EBADLINK => "OV_EBADLINK (invalid stream)",
		OV_ENOSEEK => "OV_ENOSEEK (not seekable)",
		_ => "unknown"
	}
}
