#[macro_use]
mod error;
pub use error::{VorbisError, VorbisLibrary, VorbisLibraryError, VorbisLibraryErrorKind};

mod ogg;
pub(crate) use ogg::{OggPacket, OggStream};

mod vorbis;
// self:: must be used in order to clear up ambiguity
pub(crate) use self::vorbis::{VorbisComments, VorbisInfo};
