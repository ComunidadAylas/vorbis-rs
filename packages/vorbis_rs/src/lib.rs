//! Safe, ergonomic, and high-quality Rust bindings to the `vorbisfile`, `libvorbisenc`, and
//! `libvorbis` C libraries.
//!
//! These bindings depend on the accompanying low-level FFI bindings at the `ogg_next_sys` and
//! `aotuv_lancer_vorbis_sys` crates, which link to the latest versions of the upstream `libogg` and patched
//! upstream `libvorbis` codebases, respectively. The upstream `libvorbis` codebase is patched
//! with the latest [aoTuV] and [Lancer] patches, which improve coding efficiency and performance.
//!
//! [aoTuV]: https://github.com/AO-Yumi/vorbis_aotuv
//! [Lancer]: https://web.archive.org/web/20160408055651/http://homepage3.nifty.com/blacksword/index.htm
//!
//! # Known limitations
//!
//! Decoding chained Ogg Vorbis streams (i.e., with several consecutive logical bitstreams) is not
//! supported for now. These are seldom found, however. Feel free to file an issue or open a pull
//! request if you are interested in chained stream support.
//!
//! Seeking is also not supported, although it'd be a welcome addition.
//!
//! # Examples
//!
//! The following example transcodes an Ogg Vorbis file to another in-memory Ogg Vorbis stream,
//! showing how the API offered by this crate works together to achieve this task.
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use std::fs::File;
//! # use std::io::Cursor;
//! # use vorbis_rs::{VorbisBitrateManagementStrategy, VorbisDecoder, VorbisEncoder};
//! #
//! # let mut source_ogg = &include_bytes!("../../aotuv_lancer_vorbis_sys/src/8khz_500ms_mono_400hz_sine_wave.ogg")[..];
//! # #[cfg(__example_only)]
//! let mut source_ogg = File::open("audio.ogg")?;
//! let mut transcoded_ogg = vec![];
//!
//! let mut decoder = VorbisDecoder::new(&mut source_ogg)?;
//! let mut encoder = VorbisEncoder::new(
//!     0, // Stream serial. Please consider generating a random one for production use
//!     [("", ""); 0], // No comments
//!     decoder.sampling_frequency(),
//!     decoder.channels(),
//!     VorbisBitrateManagementStrategy::QualityVbr {
//!         // This is a very low quality factor, meant to generate the smallest files and
//!         // showcase aoTuV features. You maybe not want to use it
//!         target_quality: -0.2
//!     },
//!     None, // Let the libraries choose a minimum Ogg page size
//!     &mut transcoded_ogg
//! )?;
//!
//! while let Some(decoded_block) = decoder.decode_audio_block()? {
//!     encoder.encode_audio_block(decoded_block.samples())?;
//! }
//!
//! // The encoder is automatically finished on drop, so calling finish explicitly is
//! // only needed if you want to handle any errors
//! encoder.finish()?;
//!
//! // Do whatever you want with transcoded_ogg now
//!
//! # // Decode the generated output as a sanity check
//! # let mut decoder = VorbisDecoder::new(Cursor::new(transcoded_ogg))?;
//! # while decoder.decode_audio_block()?.is_some() {}
//! # Ok(())
//! # }
//! ```

#![warn(
	let_underscore_drop,
	missing_abi,
	missing_docs,
	unused_crate_dependencies,
	unused_import_braces
)]

#[macro_use]
mod common;
pub use common::{VorbisError, VorbisLibrary, VorbisLibraryError, VorbisLibraryErrorKind};

mod encoder;
pub use encoder::{VorbisBitrateManagementStrategy, VorbisEncoder};

mod decoder;
pub use decoder::{VorbisAudioSamples, VorbisDecoder};
