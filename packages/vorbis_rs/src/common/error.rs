use std::borrow::Cow;
use std::convert::Infallible;
use std::ffi::NulError;
use std::hint::unreachable_unchecked;
use std::io;
use std::num::TryFromIntError;
use thiserror::Error;

/// An error condition that may happen during an encoding or decoding operation.
#[derive(Error, Debug)]
pub enum VorbisError {
	/// A Vorbis C library function returned an error code.
	#[error("{__error_message}{__error_reason}")]
	LibraryError {
		/// This field is an implementation detail. DO NOT USE.
		#[doc(hidden)]
		__error_message: Cow<'static, str>,
		/// This field is an implementation detail. DO NOT USE.
		#[doc(hidden)]
		__error_reason: Cow<'static, str>,
		/// The raw error code returned by the library function whose execution failed.
		error_code: i32
	},
	/// The expected and actual count of channels in an audio block did not match.
	#[error("Expected {expected} channels in audio block, got {actual}")]
	InvalidAudioBlockChannelCount { expected: usize, actual: usize },
	/// The expected and actual count of samples in an audio block channel did not match.
	#[error("Expected {expected} samples in audio block channel, got {actual}")]
	InvalidAudioBlockSampleCount { expected: usize, actual: usize },
	/// The input stream is a chained Ogg Vorbis stream, which are not supported by either
	/// Minecraft and these bindings.
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
	Io(#[from] io::Error)
}

#[doc(hidden)] // Implementation detail to allow for ergonomic usage of ?
impl From<Infallible> for VorbisError {
	fn from(_: Infallible) -> Self {
		// SAFETY: infallible can't be instantiated, so this conversion will never happen
		unsafe { unreachable_unchecked() }
	}
}

/// Converts the status code returned by a bound C function to an idiomatic [`VorbisError`].
macro_rules! return_value_to_result {
	( $func:ident ( $($arg:expr),* ), $is_ok:expr, $lib_name:literal, $result_to_error_string:expr ) => {{
		let return_value = $func($($arg),*) as i32;
		if $is_ok(return_value) {
			Ok(return_value)
		} else {
			Err(VorbisError::LibraryError {
				__error_message: concat!($lib_name, " error calling ", stringify!($func), ". Reason: ").into(),
				__error_reason: $result_to_error_string(return_value).into(),
				error_code: return_value
			})
		}
	}};
	( $func:ident ( $($arg:expr),* ), $lib_name:literal ) => {
		return_value_to_result!(
			$func($($arg),*),
			|return_value: i32| return_value >= 0,
			$lib_name,
			|return_value: i32| vorbis_error_code_to_string(return_value)
		)
	};
}

/// Converts the status code returned by a bound `libogg` C function to an idiomatic [`VorbisError`].
macro_rules! libogg_return_value_to_result {
	( $func:ident ( $($arg:expr),* ) ) => {
		return_value_to_result!($func($($arg),*), "libogg")
	};
}

/// Converts the status code returned by a bound `libvorbis` C function to an idiomatic [`VorbisError`].
macro_rules! libvorbis_return_value_to_result {
	( $func:ident ( $($arg:expr),* ) ) => {
		return_value_to_result!($func($($arg),*), "libvorbis")
	};
}

macro_rules! libvorbisenc_return_value_to_result {
	( $func:ident ( $($arg:expr),* ) ) => {
		return_value_to_result!($func($($arg),*), "libvorbisenc")
	};
}

/// Converts the status code returned by a bound `vorbisfile` C function to an idiomatic [`VorbisError`].
macro_rules! vorbisfile_return_value_to_result {
	( $func:ident ( $($arg:expr),* ) ) => {
		return_value_to_result!($func($($arg),*), "vorbisfile")
	};
}
