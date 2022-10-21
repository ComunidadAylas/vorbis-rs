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

/// Converts a status code returned by a bound C function to a more user-friendly string representation.
pub(crate) const fn vorbis_error_code_to_string(error_code: i32) -> &'static str {
	// List from
	// https://github.com/xiph/vorbis/blob/84c023699cdf023a32fa4ded32019f194afcdad0/include/vorbis/codec.h#L221-L235
	match error_code {
		crate::OV_FALSE => "OV_FALSE",
		crate::OV_EOF => "OV_EOF (stream at end of file)",
		crate::OV_HOLE => "OV_HOLE (data interruption. Try repairing the file)",
		crate::OV_EREAD => "OV_EREAD (I/O error)",
		crate::OV_EFAULT => "OV_EFAULT (internal error)",
		crate::OV_EIMPL => "OV_EIMPL (not implemented. Maybe try other parameters?)",
		crate::OV_EINVAL => "OV_EINVAL (invalid parameter)",
		crate::OV_ENOTVORBIS => "OV_ENOTVORBIS (not Vorbis data)",
		crate::OV_EBADHEADER => "OV_EBADHEADER (invalid Vorbis stream header)",
		crate::OV_EVERSION => "OV_EVERSION (Vorbis version mismatch)",
		crate::OV_ENOTAUDIO => "OV_ENOTAUDIO (not audio data)",
		crate::OV_EBADPACKET => "OV_EBADPACKET (invalid packet)",
		crate::OV_EBADLINK => "OV_EBADLINK (invalid stream)",
		crate::OV_ENOSEEK => "OV_ENOSEEK (not seekable)",
		_ => "unknown"
	}
}
