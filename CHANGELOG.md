# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

No changes yet.

## [0.2.0] - 2023-02-08

### Fixed

- Prevented `VorbisDecoder` from accepting `Read` implementations that borrow
  data that may be dropped while `VorbisDecoder` is referenced. Now
  `VorbisDecoder` behaves as if it owned the `Read` object. Previously, code
  that could cause undefined behavior was accepted, such as:

  ```rs
	let vorbis_data = /* Create some Vec<u8> */;

	// VorbisDecoder::new borrows vorbis_data for the duration of the function invocation
	let mut decoder = VorbisDecoder::new::<&[u8], _>(&*vorbis_data).unwrap();

	// The Vorbis data buffer used as a source for decoding may be freed later...
	drop(vorbis_data);

	// ... but safe code still can read from a deallocated buffer!
	while let Ok(Some(_)) = decoder.decode_audio_block() {
		eprintln!("Undefined behavior!");
	}
  ```

- Fixed a memory leak that occurred when `VorbisDecoder::new` returned unsuccessfully

### Changed

- More idiomatic Rust representation of C library errors by using enums ([#7](https://github.com/ComunidadAylas/vorbis-rs/issues/7))
- Optimized `VorbisDecoder` implementation to get rid of double indirection for the Vorbis stream source across the FFI boundary
- Removed unnecessary reference to Minecraft in a rustdoc comment
- Explicitly document the audio block format expected by the `encode_audio_block` method
- Document I/O side effects of `Vorbis{Decoder,Encoder}::new`

## [0.1.0] - 2022-10-21

### Added

- First public release

[Unreleased]: https://github.com/ComunidadAylas/vorbis-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/ComunidadAylas/vorbis-rs/releases/tag/v0.2.0
[0.1.0]: https://github.com/ComunidadAylas/vorbis-rs/releases/tag/v0.1.0
