use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
#[cfg(feature = "std")] use std::fs::Metadata;
#[cfg(feature = "std")] use std::path::Path;

use easy_ext::ext;

use crate::inherent::Sealed;

#[cfg(feature = "std")]
#[ext]
pub impl Metadata
where Self: Sealed<Metadata>
{
    fn is_symlink(&self) -> bool {
        self.file_type().is_symlink()
    }
}

#[cfg(feature = "std")]
#[ext]
pub impl Path
where Self: Sealed<Path>
{
    fn is_symlink(&self) -> bool {
        std::fs::symlink_metadata(self).map(|m| m.is_symlink()).unwrap_or(false)
    }
}

macro_rules! impl_int_v1_58 {
    ($($signed_type:ty, $unsigned_type:ty),+) => {$(
        #[ext]
        pub impl $signed_type where Self: Sealed<$signed_type> {
            fn saturating_div(self, rhs: Self) -> Self {
                match self.overflowing_div(rhs) {
                    (result, false) => result,
                    (_, true) => Self::max_value(),
                }
            }
        }

        #[ext]
        pub impl $unsigned_type where Self: Sealed<$unsigned_type> {
            fn saturating_div(self, rhs: Self) -> Self {
                self.wrapping_div(rhs)
            }
        }
    )*};
}

impl_int_v1_58![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];

#[ext]
pub impl<T> Option<T>
where Self: Sealed<Option<T>>
{
    unsafe fn unwrap_unchecked(self) -> T {
        debug_assert!(self.is_some());
        match self {
            Some(val) => val,
            None => core::hint::unreachable_unchecked(),
        }
    }
}

#[ext]
pub impl<T, E> Result<T, E>
where Self: Sealed<Result<T, E>>
{
    unsafe fn unwrap_unchecked(self) -> T {
        debug_assert!(self.is_ok());
        match self {
            Ok(val) => val,
            Err(_) => core::hint::unreachable_unchecked(),
        }
    }

    unsafe fn unwrap_err_unchecked(self) -> E {
        debug_assert!(self.is_err());
        match self {
            Ok(_) => core::hint::unreachable_unchecked(),
            Err(err) => err,
        }
    }
}

macro_rules! impl_nonzero_v1_58 {
    ($($t:ty)*) => {$(
        #[ext]
        pub impl $t
        where Self: Sealed<$t>
        {
            fn is_power_of_two(self) -> bool {
                self.get().is_power_of_two()
            }
        }
    )*};
}

impl_nonzero_v1_58![NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize];
