# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2024-12-02

[1.1.0]: https://github.com/sunsided/merge-whitespace-rs/releases/tag/v1.1.0

### Added

- Added the `merge-whitespace-utils` crate to expose the `merge_whitespace` and `merge_whitespace_with_quotes`
  functions for operating on strings rather than string literals.

### Internal

- Updated dependencies to latest patch releases.
- Added fuzz testing to ensure panic-safe implementation.

## [1.0.0] - 2024-06-03

[1.0.0]: https://github.com/sunsided/merge-whitespace-rs/releases/tag/v1.0.0

### Added

- Added support for `quote_char` and `escape_char` arguments to have optional quotation and
  escaping support.

### Internal

- The crate is now explicitly labeled as `#![forbid(unsafe_code)]`.

## [0.1.0] - 2024-05-14

[0.1.0]: https://github.com/sunsided/merge-whitespace-rs/releases/tag/v0.1.0

### Added

- Added the `merge_whitespace` macro.
- 🎉 Initial release.
