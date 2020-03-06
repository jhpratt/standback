//! Newtype wrappers to provide a consistent interface for types that cannot simply be copied from
//! the standard library.

#[cfg(since_1_36)]
pub mod maybe_uninit;
