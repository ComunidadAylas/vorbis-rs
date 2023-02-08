# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Fixed
- Fixed a memory leak that occurred when `VorbisDecoder::new` returned unsuccessfully

### Changed
- Removed unnecessary reference to Minecraft in a rustdoc comment
- Explicitly document the audio block format expected by the `encode_audio_block` method

## [0.1.0] - 2022-10-21
### Added
- First public release

[Unreleased]: https://github.com/ComunidadAylas/vorbis-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ComunidadAylas/vorbis-rs/releases/tag/v0.1.0
