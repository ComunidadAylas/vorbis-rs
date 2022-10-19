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

/// Converts a status code returned by a bound C function to a more user-friendly string representation.
const fn vorbis_error_code_to_string(error_code: i32) -> &'static str {
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
