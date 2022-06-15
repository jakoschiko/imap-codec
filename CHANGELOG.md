# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - YYYY-MM-DD

## [Version 0.6.0] - 2022-06-14

### Added

- Introduce "starttls" feature.
  - Cleanup and document existing features.
- Measure code coverage in CI.
  - Upload coverage report to Coveralls.io.
  - Add/Update code coverage badge in README.md.
- Compile fuzzers in CI.
- Implement benchmarks (Criterion.rs).
  - Compile benchmarks in CI.
- Implement `Command::into_static()` and `Response::into_static()`.
  - Use `bounded-static` (thanks, @jakoschiko)
- Add types to fix misuses
  - Introduce `AtomExt` (1*ASTRING-CHAR) to fix misuse.
  - Introduce `CapabilityEnable` to increase misuse-resistance.
- Split imap-codec into imap-codec and imap-types.
  - Implement non-nom parsing in imap-types.
  - Add README.md to imap-types.

### Changed

- Split crate into imap-codec and imap-types.
  - Make imap-codec the primary workspace member.
  - Re-export `imap-types`.
- Make fuzz targets members of workspace to simplify workflow.
- Rename "serdex"/"nomx" features to "serde"/"nom".
- Reduce allocations during parsing.
  - Use `Cow` to abstract over owned and borrowed slices.
- Do not check slices twice.
  - Introduce `new_unchecked()` functions.
  - Check `new_unchecked()` during debug builds.
- Cleanup API for `AuthMechanism`.
- Update to nom 7 and abnf-core 0.5.

### Removed

- Remove `impl Display` for types in imap-types.
- Remove `nom` feature in imap-types.
- Remove/cleanup (unused) dependencies in imap-codec.
- Remove/cleanup (unused) dependencies in imap-types.

### Fixed

- Fix fuzz targets.
- Fix benchmarks (thanks, @franziskuskiefer).
- Fix misuses, e.g., `AtomExt` (1*ASTRING-CHAR).

[Unreleased]: https://github.com/duesee/imap-codec/compare/468e220a3b7e4faafc23a4c1da8a05b05cd91c35...HEAD
[Version 0.6.0]: https://github.com/duesee/imap-codec/compare/fcb400e508f74a8d88bbcbfd777bdca7cb75bdeb...468e220a3b7e4faafc23a4c1da8a05b05cd91c35