<div align="center">
<img src="https://github.com/ComunidadAylas/vorbis-rs/assets/7822554/c12d850c-8924-41fa-b2f3-9d00e18bd7bc" alt="vorbis-rs logo" width="300" height="300">
<h1>vorbis-rs</h1>

<i>Rust bindings for the best-in-breed C libraries of the Vorbis audio codec and
Ogg container encapsulation.</i>

<a href="https://github.com/ComunidadAylas/vorbis-rs/actions?query=workflow%3ACI"><img alt="CI workflow status"
src="https://github.com/ComunidadAylas/vorbis-rs/actions/workflows/ci.yml/badge.svg"></a>
<a href="https://crates.io/crates/vorbis_rs"><img alt="crates.io latest version" src="https://img.shields.io/crates/v/vorbis_rs"></a>
<a href="https://docs.rs/vorbis_rs"><img alt="docs.rs status" src="https://img.shields.io/docsrs/vorbis_rs?label=docs.rs"></a>

<img alt="crates.io downloads" src="https://img.shields.io/crates/d/vorbis_rs?label=crates.io%20downloads">
</div>

The `vorbis_rs` package in this repository provides updated, well-documented and
ergonomic bindings for a modified version of the latest reference Vorbis
encoder, available [here](https://github.com/enzo1982/vorbis-aotuv-lancer), with
the [aoTuV](https://github.com/AO-Yumi/vorbis_aotuv) and
[Lancer](https://web.archive.org/web/20160408055651/http://homepage3.nifty.com/blacksword/index.htm)
patchsets applied to it. These patches are considered to implement significant
encoding quality and performance improvements by the community.

The supporting `aotuv_lancer_vorbis_sys` and `ogg_next_sys` packages provide
automatically-generated low-level bindings used by `vorbis_rs`.

The minimum supported Rust version (MSRV) for every package in this repository
is 1.64. Bumping this version is not considered a breaking change for semantic
versioning purposes. We will try to do it only when we estimate that such a bump
would not cause widespread inconvenience or breakage.

# ❓ Motivation

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

# ⚖️ License

The bindings in this repository are licensed under the BSD 3-Clause "New" or
"Revised" License (`LICENSE` or https://opensource.org/licenses/BSD-3-Clause),
which is the same permissive license used by the upstream projects.

## Contribution license

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in vorbis_rs by you shall be licensed as above, without any
additional terms or conditions.

# ✨ Contributing

This repository started as an ad-hoc solution to address the needs of a Rust
application, but it has grown into a project of its own - PRs are welcome!

## Cloning & updating

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

Each time vendor code is updated, it will be necessary to release a new version
of `aotuv_lancer_vorbis_sys` and/or `ogg_next_sys`, so that users of `vorbis_rs`
can download binding crates with the updated vendor code.

# 🤝 Contact

We welcome friendly talk about the project, including questions,
congratulations, and suggestions. Head to the [GitHub Discussions
page](https://github.com/ComunidadAylas/vorbis-rs/discussions) to interact with
fellow users, contributors and developers.

# 🧑‍🤝‍🧑 Contributors

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/AlexTMjugador"><img src="https://avatars.githubusercontent.com/u/7822554?v=4?s=100" width="100px;" alt="Alejandro González"/><br /><sub><b>Alejandro González</b></sub></a><br /><a href="https://github.com/ComunidadAylas/vorbis-rs/commits?author=AlexTMjugador" title="Code">💻</a> <a href="https://github.com/ComunidadAylas/vorbis-rs/commits?author=AlexTMjugador" title="Documentation">📖</a> <a href="#example-AlexTMjugador" title="Examples">💡</a> <a href="#maintenance-AlexTMjugador" title="Maintenance">🚧</a> <a href="#projectManagement-AlexTMjugador" title="Project Management">📆</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/vivyir"><img src="https://avatars.githubusercontent.com/u/67435287?v=4?s=100" width="100px;" alt="Vivian"/><br /><sub><b>Vivian</b></sub></a><br /><a href="https://github.com/ComunidadAylas/vorbis-rs/commits?author=vivyir" title="Code">💻</a> <a href="#ideas-vivyir" title="Ideas, Planning, & Feedback">🤔</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://dulc.es/@martin"><img src="https://avatars.githubusercontent.com/u/227204?v=4?s=100" width="100px;" alt="Martin Algesten"/><br /><sub><b>Martin Algesten</b></sub></a><br /><a href="https://github.com/ComunidadAylas/vorbis-rs/commits?author=algesten" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/astral4"><img src="https://avatars.githubusercontent.com/u/88992929?v=4?s=100" width="100px;" alt="Kyle Chen"/><br /><sub><b>Kyle Chen</b></sub></a><br /><a href="#ideas-astral4" title="Ideas, Planning, & Feedback">🤔</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://mastodon.gamedev.place/@daniel_collin"><img src="https://avatars.githubusercontent.com/u/350822?v=4?s=100" width="100px;" alt="Daniel Collin"/><br /><sub><b>Daniel Collin</b></sub></a><br /><a href="https://github.com/ComunidadAylas/vorbis-rs/issues?q=author%3Aemoon" title="Bug reports">🐛</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the
[all-contributors](https://github.com/all-contributors/all-contributors)
specification. Contributions of any kind welcome!
