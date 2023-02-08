use std::mem::MaybeUninit;

#[macro_use]
mod error;
pub use error::{VorbisError, VorbisLibrary, VorbisLibraryError, VorbisLibraryErrorKind};

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
pub(crate) unsafe fn assume_init_box<T>(boxed: Box<MaybeUninit<T>>) -> Box<T> {
	Box::from_raw(Box::into_raw(boxed) as *mut T)
}
