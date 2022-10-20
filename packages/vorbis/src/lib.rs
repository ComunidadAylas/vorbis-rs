//! Safe, ergonomic, and high-quality Rust bindings to the `vorbisfile`, `libvorbisenc`, and
//! `libvorbis`  C libraries, focused on satisfying the audio processing requirements of
//! [PackSquash](https://github.com/ComunidadAylas/PackSquash).
//!
//! These bindings depend on the custom, low-level FFI bindings of the accompanying `ogg_sys` and
//! `vorbis_sys` crates, which link to the latest versions of the upstream `libogg` and patched
//! upstream `libvorbis` codebases, respectively. The upstream `libvorbis` codebase is patched
//! with the latest aoTuV and Lancer patches, which improve coding efficiency and performance.

#[cfg(test)]
mod tests;

/// A constant whose numeric value represents a libvorbis `OV_EBADHEADER` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EBADHEADER;
/// A constant whose numeric value represents a libvorbis `OV_EBADLINK` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EBADLINK;
/// A constant whose numeric value represents a libvorbis `OV_EBADPACKET` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EBADPACKET;
/// A constant whose numeric value represents a libvorbis `OV_EFAULT` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EFAULT;
/// A constant whose numeric value represents a libvorbis `OV_EIMPL` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EIMPL;
/// A constant whose numeric value represents a libvorbis `OV_EINVAL` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EINVAL;
/// A constant whose numeric value represents a libvorbis `OV_ENOSEEK` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_ENOSEEK;
/// A constant whose numeric value represents a libvorbis `OV_ENOTAUDIO` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_ENOTAUDIO;
/// A constant whose numeric value represents a libvorbis `OV_ENOTVORBIS` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_ENOTVORBIS;
/// A constant whose numeric value represents a libvorbis `OV_EOF` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EOF;
/// A constant whose numeric value represents a libvorbis `OV_EREAD` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EREAD;
/// A constant whose numeric value represents a libvorbis `OV_EVERSION` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_EVERSION;
/// A constant whose numeric value represents a libvorbis `OV_FALSE` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_FALSE;
/// A constant whose numeric value represents a libvorbis `OV_HOLE` error, which may be
/// returned as a [`LibraryError`](VorbisError::LibraryError) error code.
pub use vorbis_sys::OV_HOLE;

#[macro_use]
mod common;
pub use common::VorbisError;

mod encoder;
pub use encoder::{VorbisBitrateManagementStrategy, VorbisEncoder};

mod decoder;
pub use decoder::{VorbisAudioSamples, VorbisDecoder};
