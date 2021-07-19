#[allow(unreachable_pub)] // false positive
pub(crate) mod prelude {
    pub(crate) mod rust_2015 {
        #[cfg(feature = "alloc")]
        pub use alloc::borrow::ToOwned;
        #[cfg(feature = "alloc")]
        pub use alloc::boxed::Box;
        #[cfg(feature = "alloc")]
        pub use alloc::string::{String, ToString};
        #[cfg(feature = "alloc")] pub use alloc::vec::Vec;
        pub use core::prelude::v1::*;
    }
    pub(crate) mod rust_2018 {
        pub use super::rust_2015::*;
    }
    pub(crate) mod rust_2021 {
        pub use core::convert::{TryFrom, TryInto};
        pub use core::iter::FromIterator;

        pub use super::rust_2015::*;
    }
}
