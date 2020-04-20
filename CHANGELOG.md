# Changelog

All notable changes to the standback project will be documented in this file.

The format is based on [Keep a Changelog]. This project adheres to [Semantic
Versioning].

---

## Unreleased

Nothing yet.

## 0.2.4 [2020-04-20]

- `TryFrom` identity is implemented for some primitives.
- `TryFrom`, `TryInto` and correctly re-exported.
- `TryFromIntError` has been moved to `mod num`.

## 0.2.3 [2020-04-18]

Embedded and `#![no_std]` now have full support and are checked in CI.

## 0.2.2 [2020-04-02]

The version of rustc being used will now respect the $RUSTC environment
variable.

## 0.2.1 [2020-03-13]

Support for Rust 1.42.0 APIs has been added.

## 0.2.0 [2020-03-10]

- `MaybeUninit` is restricted to `Copy` types, eliminating undefined behavior.
- `todo!` has been moved to the prelude.

## 0.1.0 [2020-03-05]

Initial release.
