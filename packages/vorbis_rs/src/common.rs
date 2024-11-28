#[macro_use]
mod error;
pub use error::{VorbisError, VorbisLibrary, VorbisLibraryError, VorbisLibraryErrorKind};

mod ogg;
pub use ogg::{OggPacket, OggStream};

mod vorbis;
// self:: must be used in order to clear up ambiguity
pub use self::vorbis::{VorbisComments, VorbisInfo};
