#[cfg(feature = "std")] mod available_parallelism;
mod zip;

#[cfg(feature = "std")]
#[allow(unreachable_pub)] // false positive
pub use available_parallelism::*;
#[allow(unreachable_pub)] // false positive
pub use zip::*;
