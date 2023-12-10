# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added a logo for `vorbis-rs` in the project README.

### Changed

- Added a section to the `VorbisEncoder::encode_audio_block` documentation
  explaining important details about sample block size and encoding performance.
- Added a remark to the `VorbisEncoder::encode_audio_block` documentation about
  the usual numeric range of the input sample values.
- Bumped MSRV to 1.64 due to the new usage of workspace property inheritance
  features introduced in that Rust version.

### Fixed

- Corrected another errata in the `libvorbis` patch for the stack overflow
  fix of [0.5.1].

## [0.5.2] - 2023-11-25

### Fixed

- Corrected an errata in the `libvorbis` patch for the stack overflow fix of
  [0.5.1].

## [0.5.1] - 2023-11-25

### Fixed

- `VorbisEncoder::encode_audio_block` no longer causes a stack overflow in
  practical scenarios when encoding a large sample buffer. This overflow
  occurred when the available stack space was too small to handle the buffer,
  and usually caused segmentation faults.
  ([#17](https://github.com/ComunidadAylas/vorbis-rs/issues/17), thanks @emoon)

### Changed

- Corrected some minor pedantic Clippy lints.
- Slightly reduced heap allocations when decoding signals with more than two
  audio channels.
- Unified some minor cosmetic details of this changelog.

## [0.5.0] - 2023-08-06

### Changed

- The `VorbisEncoderBuilder::build` method now has a mutable reference receiver,
  making its usage more ergonomic and enabling more use cases.
  ([#11](https://github.com/ComunidadAylas/vorbis-rs/issues/11))
- Due to the above change, `VorbisEncoderBuilder` now automatically marks the
  stream serials it uses to build encoders for renewal, triggering their
  automatic replacement with different ones when `VorbisEncoderBuilder::build`
  is called. This behavior is meant to provide a reasonable default when using
  the same builder to build multiple encoders, but it can be customized by
  calling `VorbisEncoderBuilder::stream_serial` method before
  `VorbisEncoderBuilder::build`.

## [0.4.0] - 2023-07-29

### Added

- The `VorbisEncoderBuilder` struct was added for more ergonomic `VorbisEncoder`
  instantiation. The now obsolete `VorbisEncoder::new` method was removed.
- When the new default feature `stream-serial-rng` is enabled,
  `VorbisEncoderBuilder` can now automatically generate a random Ogg stream
  serial.
- `VorbisBitrateManagementStrategy` now has a `Default` implementation that
  targets a medium, usually transparent audio quality.

### Changed

- Removed unnecessary internal allocation in `VorbisDecoder`.

## [0.3.0] - 2023-04-14

### Changed

- `VorbisEncoder::finish` now returns the sink it was created with.
  ([#8](https://github.com/ComunidadAylas/vorbis-rs/pull/8), thanks @algesten)
- Updated author contact metadata for every crate in the repository.

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

- Fixed a memory leak that occurred when `VorbisDecoder::new` returned unsuccessfully.

### Changed

- More idiomatic Rust representation of C library errors by using enums. ([#7](https://github.com/ComunidadAylas/vorbis-rs/issues/7))
- Optimized `VorbisDecoder` implementation to get rid of double indirection for the Vorbis stream source across the FFI boundary.
- Removed unnecessary reference to Minecraft in a rustdoc comment.
- Explicitly document the audio block format expected by the `encode_audio_block` method.
- Document I/O side effects of `Vorbis{Decoder,Encoder}::new`.

## [0.1.0] - 2022-10-21

### Added

- First public release.

[Unreleased]: https://github.com/ComunidadAylas/vorbis-rs/compare/v0.5.2...HEAD
[0.5.2]: https://github.com/ComunidadAylas/vorbis-rs/releases/tag/v0.5.2
[0.5.1]: https://github.com/ComunidadAylas/vorbis-rs/releases/tag/v0.5.1
[0.5.0]: https://github.com/ComunidadAylas/vorbis-rs/releases/tag/v0.5.0
[0.4.0]: https://github.com/ComunidadAylas/vorbis-rs/releases/tag/v0.4.0
[0.3.0]: https://github.com/ComunidadAylas/vorbis-rs/releases/tag/v0.3.0
[0.2.0]: https://github.com/ComunidadAylas/vorbis-rs/releases/tag/v0.2.0
[0.1.0]: https://github.com/ComunidadAylas/vorbis-rs/releases/tag/v0.1.0
