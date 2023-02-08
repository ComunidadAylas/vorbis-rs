# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Prevented `VorbisDecoder` from accepting `Read` implementations that borrow
  data that may be dropped while `VorbisDecoder` is referenced. Now
  `VorbisDecoder` behaves as if it owned the `Read` object. Previously, code
  that could cause undefined behavior was accepted, such as:

  ```rs
	let vorbis_data = /* Create some Vec<u8> */;

	let mut decoder = VorbisDecoder::new(&*vorbis_data).unwrap();

	// Free the Vorbis data buffer used as a source for decoding...
	drop(vorbis_data);

	// ...but VorbisDecoder still can read from it!
	while let Ok(Some(_)) = decoder.decode_audio_block() {
		eprintln!("Undefined behavior!");
	}
  ```

- Fixed a memory leak that occurred when `VorbisDecoder::new` returned unsuccessfully

### Changed

- Optimized `VorbisDecoder` implementation to get rid of double indirection for the Vorbis stream source accross the FFI boundary
- Removed unnecessary reference to Minecraft in a rustdoc comment
- Explicitly document the audio block format expected by the `encode_audio_block` method

## [0.1.0] - 2022-10-21

### Added

- First public release

[Unreleased]: https://github.com/ComunidadAylas/vorbis-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ComunidadAylas/vorbis-rs/releases/tag/v0.1.0
