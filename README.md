# vorbis-rs

High and low-level Rust bindings for API-compatible C libraries of the reference
implementation of the Vorbis audio codec and Ogg container encapsulation.

This project provides bindings for the [upstream
`libogg`](https://github.com/xiph/ogg) library. The Vorbis encoder is a modified
version of the latest reference implementation, available
[here](https://github.com/enzo1982/vorbis-aotuv-lancer), with the
[aoTuV](https://github.com/AO-Yumi/vorbis_aotuv) and
[Lancer](https://web.archive.org/web/20160408055651/http://homepage3.nifty.com/blacksword/index.htm)
patchsets applied to it, which are considered to implement significant encoding
quality and performance improvements by the community.

## ❓ Motivation

The Rust ecosystem already has bindings for these libraries (see
[`vorbis-sys`](https://crates.io/crates/vorbis-sys) and
[`vorbis`](https://crates.io/crates/vorbis)), but the quality and maintenance
status of the available crates is problematic in an entangled way that does not
seem reasonable to fix via PRs or patching:

- The high-level `vorbis` crate was not updated in 6 years, has arguably low
  code quality (lots of `panic!` with messages containing e-mail addresses,
  etc.), and depends on an old version of `vorbis-sys`. In turn `vorbis-sys`
  depends on an old version of the `libvorbis` C library with known security
  vulnerabilities. It also lacks APIs to do some operations needed by sensible
  audio processing applications that are offered by the C libraries.
- `vorbis-sys` only contains bindings for `libvorbis`, but updated bindings for
  `libvorbisenc` and `vorbisfile` are necessary to do meaningful Vorbis stream
  operations in a sane way. These are not available either.
- There are several random crates in crates.io depending on several of these
  unsatisfactory binding projects, and it seems unlikely that their maintainers
  will promptly accept breaking changes on them, generating ecosystem
  fragmentation.
- Like most software libraries, frameworks and even Linux distributions, the
  existing crates seem to be oblivious to the existence of the aoTuV and Lancer
  patchsets, even though they are meant to be drop-in replacements. Thus, users
  may reasonably expect any patches to be mentioned, which is a good reason to
  do a different set of binding crates anyway.

Given these issues and the need for a better solution for Ogg Vorbis audio
processing applications in Rust, it was decided to spend development effort on
making new bindings: it was estimated that the upfront cost of fixing the
technical debt of the ecosystem was higher than starting bindings from scratch
and periodically updating the library bindings from upstream. Rewriting the
patched Vorbis encoder in Rust was deemed unfeasible.

## ✨ Contributing

This repository started as an ad-hoc solution to address the needs of a Rust
application, but it has grown into a project of its own - PRs are welcome!

### Cloning & updating

The C libraries sources are managed with
[submodules](https://git-scm.com/book/en/v2/Git-Tools-Submodules), so updating
the binding crates with the latest upstream changes should be easy:

1. Update the submodules.
2. Run the `generate-bindings.sh` script on a Unix-like system (Linux, macOS,
   BSD) with
   [`rust-bindgen`](https://rust-lang.github.io/rust-bindgen/command-line-usage.html),
   or build the project with `cargo build --features build-time-bindgen`. This
   will regenerate the low-level FFI bindings according to the latest source
   code.
3. Run `cargo test`. This will execute some basic sanity checks, including
   encoding and decoding example files, to check that the bindings still work.
4. Review the changes and/or fix failing tests until there is reasonable
   confidence that the upgrade was completed successfully.
5. Commit the changes.

When cloning the repository, remember to also check out the submodules with the
vendor code. You can do this by running `git submodule update --init --recursive`.
