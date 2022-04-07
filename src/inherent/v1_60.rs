#[cfg(feature = "alloc")] use alloc::vec::Vec;
use core::mem::MaybeUninit;
use core::{ascii, mem, ptr, slice};

use easy_ext::ext;

use crate::inherent::sealed::Sealed;

#[ext]
pub impl u8
where Self: Sealed<u8>
{
    fn escape_ascii(self) -> ascii::EscapeDefault {
        ascii::escape_default(self)
    }
}

#[cfg(feature = "alloc")]
#[ext]
pub impl<T> Vec<T>
where Self: Sealed<Vec<T>>
{
    fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        unsafe {
            slice::from_raw_parts_mut(
                self.as_mut_ptr().add(self.len()) as *mut MaybeUninit<T>,
                self.capacity() - self.len(),
            )
        }
    }
}

#[ext]
pub impl<T> MaybeUninit<T>
where Self: Sealed<MaybeUninit<T>>
{
    unsafe fn assume_init_drop(&mut self) {
        ptr::drop_in_place(self.as_mut_ptr())
    }

    unsafe fn assume_init_read(&self) -> T {
        self.as_ptr().read()
    }
}

macro_rules! impl_abs_diff {
    ($($unsigned:ident $signed:ident)*) => {$(
        #[ext]
        pub impl $unsigned
        where Self: Sealed<$unsigned>
        {
            fn abs_diff(self, other: $unsigned) -> $unsigned {
                if mem::size_of::<Self>() == 1 {
                    (self as i32).wrapping_sub(other as i32).abs() as Self
                } else if self < other {
                    other - self
                } else {
                    self - other
                }
            }
        }

        #[ext]
        pub impl $signed
        where Self: Sealed<$signed> {
            fn abs_diff(self, other: $signed) -> $unsigned {
                if self < other {
                    (other as $unsigned).wrapping_sub(self as $unsigned)
                } else {
                    (self as $unsigned).wrapping_sub(other as $unsigned)
                }
            }
        }
    )*};
}

impl_abs_diff![
    u8 i8
    u16 i16
    u32 i32
    u64 i64
    u128 i128
    usize isize
];
