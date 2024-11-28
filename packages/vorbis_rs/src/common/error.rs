use std::{
	convert::Infallible,
	ffi::NulError,
	fmt::{self, Display, Formatter},
	hint::unreachable_unchecked,
	io,
	num::TryFromIntError
};

use thiserror::Error;

/// An error condition that may happen during an encoding or decoding operation.
///
/// New error condition types may be added over time.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum VorbisError {
	/// A Vorbis C library function informed about an error condition.
	#[error("Library error: {0}")]
	LibraryError(#[from] VorbisLibraryError),
	/// The expected and actual count of channels in an audio block did not match.
	#[error("Expected {expected} channels in audio block, got {actual}")]
	InvalidAudioBlockChannelCount {
		/// The expected channel count.
		expected: usize,
		/// The actual channel count of the provided audio samples block.
		actual: usize
	},
	/// The expected and actual count of samples in an audio block channel did not match.
	#[error("Expected {expected} samples in audio block channel, got {actual}")]
	InvalidAudioBlockSampleCount {
		/// The expected sample count.
		expected: usize,
		/// The actual sample count of the provided audio samples block.
		actual: usize
	},
	/// The input stream is a chained Ogg Vorbis stream, which is not supported by these bindings.
	#[error("Chained Ogg Vorbis streams are not supported. Please combine them into a single logical stream")]
	UnsupportedStreamChaining,
	/// A requested Vorbis user comment string contains a NUL character, which is not allowed.
	#[error("Invalid comment string: {0}")]
	InvalidCommentString(#[from] NulError),
	/// A number had a value that exceeded platform-dependent numerical limits.
	#[error("Integer outside of expected range: {0}")]
	RangeExceeded(#[from] TryFromIntError),
	/// An I/O error occurred.
	#[error("I/O error: {0}")]
	Io(#[from] io::Error),
	/// A necessary random number could not be generated due to an error. Currently, this error
	/// may only happen during automatic Ogg stream serial generation, but this is subject to
	/// change in the future.
	#[cfg(feature = "stream-serial-rng")]
	#[error("RNG error: {0}")]
	Rng(#[from] getrandom::Error),
	/// The output sink was already consumed by a previous `VorbisEncoderBuilder::build`
	/// operation. A new one must be set up to keep using the builder to create new encoders.
	#[error("The output sink was already consumed by a previous build operation")]
	ConsumedEncoderBuilderSink
}

#[doc(hidden)] // Implementation detail to allow for ergonomic usage of ?
impl From<Infallible> for VorbisError {
	fn from(_: Infallible) -> Self {
		// SAFETY: Infallible can't be instantiated, so this conversion will never happen
		unsafe { unreachable_unchecked() }
	}
}

/// An error condition reported by one of the `libogg`, `libvorbis`,
/// `libvorbisenc` or `vorbisfile` C libraries during encoding or
/// decoding operations.
#[derive(Error, Debug)]
#[error("{library} error calling {function}: {kind}")]
pub struct VorbisLibraryError {
	pub(crate) library: VorbisLibrary,
	pub(crate) function: &'static str,
	pub(crate) kind: VorbisLibraryErrorKind
}

impl VorbisLibraryError {
	/// Returns the C library that yielded this error.
	pub fn library(&self) -> VorbisLibrary {
		self.library
	}

	/// Returns the name of the C function that yielded this error.
	pub fn function(&self) -> &'static str {
		self.function
	}

	/// Returns the corresponding [`VorbisLibraryErrorKind`] for this error.
	pub fn kind(&self) -> VorbisLibraryErrorKind {
		self.kind
	}
}

/// Represents an Ogg Vorbis C library that participates in audio encoding
/// or decoding operations, and may be responsible for a [`VorbisLibraryError`].
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum VorbisLibrary {
	/// The `libogg` C library.
	Ogg,
	/// The `libvorbis` C library-
	Vorbis,
	/// The `libvorbisenc` C library.
	VorbisEnc,
	/// The `vorbisfile` C library.
	VorbisFile
}

impl Display for VorbisLibrary {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Ogg => f.write_str("libogg"),
			Self::Vorbis => f.write_str("libvorbis"),
			Self::VorbisEnc => f.write_str("libvorbisenc"),
			Self::VorbisFile => f.write_str("vorbisfile")
		}
	}
}

/// A category of [`VorbisLibraryError`].
///
/// This list of categories may vary over time, depending on the underlying C libraries and
/// how their return codes to error categories are mapped.
// List derived from:
// https://github.com/xiph/vorbis/blob/84c023699cdf023a32fa4ded32019f194afcdad0/include/vorbis/codec.h#L221-L235
// These error categories are returned by libvorbis, libvorbisenc and vorbisfile.
// libogg either repurposes return values or uses a different set of error codes. Therefore, libogg
// error codes should get the Other variant
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
#[non_exhaustive]
pub enum VorbisLibraryErrorKind {
	/// Unexpected, generic false condition.
	///
	/// Corresponds to the low-level [`OV_FALSE`](aotuv_lancer_vorbis_sys::OV_FALSE) result code.
	False,
	/// Unexpected end of file reached in input stream.
	///
	/// Corresponds to the low-level [`OV_EOF`](aotuv_lancer_vorbis_sys::OV_EOF) result code.
	Eof,
	/// Unexpected input stream data interruption.
	///
	/// Recovery from this error condition is automatically done by `libvorbis`, so most
	/// applications can ignore it and keep decoding the stream as usual.
	///
	/// Corresponds to the low-level [`OV_HOLE`](aotuv_lancer_vorbis_sys::OV_HOLE) result code.
	Hole,
	/// Unexpected input stream I/O error.
	///
	/// Corresponds to the low-level [`OV_EREAD`](aotuv_lancer_vorbis_sys::OV_EREAD) result code.
	Io,
	/// Internal library error.
	///
	/// Corresponds to the low-level [`OV_EFAULT`](aotuv_lancer_vorbis_sys::OV_EFAULT) result code.
	InternalFault,
	/// The requested operation is not implemented for the specified parameters.
	///
	/// Corresponds to the low-level [`OV_EIMPL`](aotuv_lancer_vorbis_sys::OV_EIMPL) result code.
	NotImplemented,
	/// Invalid parameter.
	///
	/// Corresponds to the low-level [`OV_EINVAL`](aotuv_lancer_vorbis_sys::OV_EINVAL) result code.
	InvalidValue,
	/// The input stream does not contain Ogg Vorbis data as expected.
	///
	/// Corresponds to the low-level [`OV_ENOTVORBIS`](aotuv_lancer_vorbis_sys::OV_ENOTVORBIS) result code.
	NotVorbis,
	/// The input stream headers are malformed.
	///
	/// Corresponds to the low-level [`OV_EBADHEADER`](aotuv_lancer_vorbis_sys::OV_EBADHEADER) result code.
	BadHeader,
	/// Vorbis version mismatch, caused by the input stream declaring an unknown or incompatible Vorbis
	/// version.
	///
	/// Corresponds to the low-level [`OV_EVERSION`](aotuv_lancer_vorbis_sys::OV_EVERSION) result code.
	BadVorbisVersion,
	/// The input stream cannot contain audio data.
	///
	/// Corresponds to the low-level [`OV_ENOTAUDIO`](aotuv_lancer_vorbis_sys::OV_ENOTAUDIO) result code.
	NotAudio,
	/// The input stream contains an invalid packet.
	///
	/// Corresponds to the low-level [`OV_EBADPACKET`](aotuv_lancer_vorbis_sys::OV_EBADPACKET) result code.
	BadPacket,
	/// An Ogg logical stream in the input stream is corrupted.
	///
	/// Corresponds to the low-level [`OV_EBADLINK`](aotuv_lancer_vorbis_sys::OV_EBADLINK) result code.
	BadLink,
	/// A seek operation was attempted in an unseekable input stream.
	///
	/// Corresponds to the low-level [`OV_ENOSEEK`](aotuv_lancer_vorbis_sys::OV_ENOSEEK) result code.
	NotSeekable,
	/// An error condition that does not fall under any other error kind.
	///
	/// Currently, this is the expected kind for errors returned by `libogg`, but this may change in
	/// the future.
	Other {
		/// The raw result code returned by the library function whose execution failed.
		result_code: i32
	}
}

impl Display for VorbisLibraryErrorKind {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::False => f.write_str("OV_FALSE"),
			Self::Eof => f.write_str("OV_EOF: stream at end of file"),
			Self::Hole => {
				f.write_str("OV_HOLE: stream data interruption. Try repairing the stream")
			}
			Self::Io => f.write_str("OV_EREAD: I/O error"),
			Self::InternalFault => f.write_str("OV_EFAULT: internal error"),
			Self::NotImplemented => {
				f.write_str("OV_EIMPL: not implemented. Maybe try other parameters?")
			}
			Self::InvalidValue => f.write_str("OV_EINVAL: invalid parameter"),
			Self::NotVorbis => f.write_str("OV_ENOTVORBIS: not Vorbis data"),
			Self::BadHeader => f.write_str("OV_EBADHEADER: invalid Vorbis stream header"),
			Self::BadVorbisVersion => f.write_str("OV_EVERSION: Vorbis version mismatch"),
			Self::NotAudio => f.write_str("OV_ENOTAUDIO: not audio data"),
			Self::BadPacket => f.write_str("OV_EBADPACKET: invalid packet"),
			Self::BadLink => f.write_str("OV_EBADLINK: invalid stream"),
			Self::NotSeekable => f.write_str("OV_ENOSEEK: not seekable"),
			Self::Other { result_code } => f.write_fmt(format_args!("Other: {result_code}"))
		}
	}
}

impl From<i32> for VorbisLibraryErrorKind {
	fn from(result_code: i32) -> Self {
		match result_code {
			aotuv_lancer_vorbis_sys::OV_FALSE => Self::False,
			aotuv_lancer_vorbis_sys::OV_EOF => Self::Eof,
			aotuv_lancer_vorbis_sys::OV_HOLE => Self::Hole,
			aotuv_lancer_vorbis_sys::OV_EREAD => Self::Io,
			aotuv_lancer_vorbis_sys::OV_EFAULT => Self::InternalFault,
			aotuv_lancer_vorbis_sys::OV_EIMPL => Self::NotImplemented,
			aotuv_lancer_vorbis_sys::OV_EINVAL => Self::InvalidValue,
			aotuv_lancer_vorbis_sys::OV_ENOTVORBIS => Self::NotVorbis,
			aotuv_lancer_vorbis_sys::OV_EBADHEADER => Self::BadHeader,
			aotuv_lancer_vorbis_sys::OV_EVERSION => Self::BadVorbisVersion,
			aotuv_lancer_vorbis_sys::OV_ENOTAUDIO => Self::NotAudio,
			aotuv_lancer_vorbis_sys::OV_EBADPACKET => Self::BadPacket,
			aotuv_lancer_vorbis_sys::OV_EBADLINK => Self::BadLink,
			aotuv_lancer_vorbis_sys::OV_ENOSEEK => Self::NotSeekable,
			result_code => Self::Other { result_code }
		}
	}
}

/// Converts the status code returned by a bound C function to an idiomatic [`VorbisError`].
macro_rules! return_value_to_result {
	( $func:ident ( $($arg:expr),* ), $lib:ident ) => {{
		#[allow(clippy::unnecessary_fallible_conversions)]
		let return_value = $func($($arg),*) as i32;
		if return_value >= 0 {
			Ok(return_value)
		} else {
			Err($crate::VorbisError::from($crate::VorbisLibraryError {
				library: $crate::VorbisLibrary::$lib,
				function: stringify!($func),
				kind: if matches!($crate::VorbisLibrary::$lib, $crate::VorbisLibrary::Ogg) {
					$crate::VorbisLibraryErrorKind::Other { result_code: return_value }
				} else {
					return_value.into()
				}
			}))
		}
	}};
}

/// Converts the status code returned by a bound `libogg` C function to an idiomatic [`VorbisError`].
macro_rules! libogg_return_value_to_result {
	( $func:ident ( $($arg:expr),* ) ) => {
		return_value_to_result!($func($($arg),*), Ogg)
	};
}

/// Converts the status code returned by a bound `libvorbis` C function to an idiomatic [`VorbisError`].
macro_rules! libvorbis_return_value_to_result {
	( $func:ident ( $($arg:expr),* ) ) => {
		return_value_to_result!($func($($arg),*), Vorbis)
	};
}

macro_rules! libvorbisenc_return_value_to_result {
	( $func:ident ( $($arg:expr),* ) ) => {
		return_value_to_result!($func($($arg),*), VorbisEnc)
	};
}

/// Converts the status code returned by a bound `vorbisfile` C function to an idiomatic [`VorbisError`].
macro_rules! vorbisfile_return_value_to_result {
	( $func:ident ( $($arg:expr),* ) ) => {
		return_value_to_result!($func($($arg),*), VorbisFile)
	};
}
