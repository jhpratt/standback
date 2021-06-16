# Changelog

All notable changes to the standback project will be documented in this file.

The format is based on [Keep a Changelog]. This project adheres to [Semantic Versioning].

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html

---

## 0.3.3 [2021-06-16]

- Support for Rust 1.53 APIs has been added.

## 0.3.2 [2021-05-05]

- Support for Rust 1.52 APIs has been added.
- Some older APIs have been added due to the internal addition of the unstable `Pattern`.

## 0.3.1 [2021-04-06]

- Support for `#![no_std]` with an allocator has been added
- The ability to declare a higher MSRV has been added.
- The crate's MSRV has been raised to 1.36 due to the `alloc` support.

## 0.3.0 [2021-04-06] [YANKED]

This version provides no functionality whatsoever and has been yanked.

## 0.2.17 [2021-03-21]

Support for Rust 1.51 APIs has been added.

## 0.2.16 [2021-03-19] [YANKED]

This version does not build on Rust 1.51+, and has been yanked.

## 0.2.15 [2021-02-04]

Support for Rust 1.50 APIs has been added.

## 0.2.14 [2020-12-30]

Support for Rust 1.49 APIs has been added.

## 0.2.13 [2020-11-16]

Support for Rust 1.48 APIs have been added.

## 0.2.12 [2020-11-16] [YANKED]

This version does not build, and has been yanked.

## 0.2.11 [2020-10-07]

Support for Rust 1.47 APIs has been added.

## 0.2.10 [2020-08-23]

- Support for Rust 1.45 and 1.46 APIs has been added.
- `float::to_int_unchecked` has been added (stabilized in Rust 1.44.0).

## 0.2.9 [2020-06-04]

- Support for Rust 1.44 APIs has been added.
- Non-stable releases are now supported. All releases are treated as equivalent to the stable that
  was out at the time.

## 0.2.8 [2020-04-27]

Removed the `primitive` module, as it caused internal compiler errors in older compilers.

## 0.2.7 [2020-04-25] [YANKED]

Additional modules and constants were added for Rust 1.43.

## 0.2.6 [2020-04-21]

Support for Rust 1.43 APIs has been added.

## 0.2.5 [2020-04-20]

- `TryFrom`, `TryInto` and correctly re-exported.
- `TryFromIntError` has been moved to `mod num`.

## 0.2.4 [2020-04-20]

`TryFrom` identity is implemented for some primitives.

## 0.2.3 [2020-04-18]

Embedded and `#![no_std]` now have full support and are checked in CI.

## 0.2.2 [2020-04-02]

The version of rustc being used will now respect the `RUSTC` environment variable.

## 0.2.1 [2020-03-13]

Support for Rust 1.42 APIs has been added.

## 0.2.0 [2020-03-10]

- `MaybeUninit` is restricted to `Copy` types, eliminating undefined behavior.
- `todo!` has been moved to the prelude.

## 0.1.0 [2020-03-05]

Initial release.
