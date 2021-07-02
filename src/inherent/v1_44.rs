#![allow(deprecated)]

use core::alloc::{Layout, LayoutErr};
use core::cmp;
use core::mem::{self, transmute};
#[cfg(feature = "std")]
use std::ffi::OsString;
#[cfg(feature = "std")]
use std::path::PathBuf;

use easy_ext::ext;

use crate::inherent::Sealed;

#[cfg(feature = "std")]
#[ext(PathBuf_v1_44)]
pub impl PathBuf
where
    Self: Sealed<PathBuf>,
{
    fn with_capacity(capacity: usize) -> PathBuf {
        OsString::with_capacity(capacity).into()
    }

    fn capacity(&self) -> usize {
        unsafe { transmute::<_, &OsString>(self) }.capacity()
    }

    fn clear(&mut self) {
        unsafe { transmute::<_, &mut OsString>(self) }.clear()
    }

    fn reserve(&mut self, additional: usize) {
        unsafe { transmute::<_, &mut OsString>(self) }.reserve(additional)
    }

    fn reserve_exact(&mut self, additional: usize) {
        unsafe { transmute::<_, &mut OsString>(self) }.reserve_exact(additional)
    }

    fn shrink_to_fit(&mut self) {
        unsafe { transmute::<_, &mut OsString>(self) }.shrink_to_fit()
    }
}

#[ext(Layout_v1_44)]
pub impl Layout
where
    Self: Sealed<Layout>,
{
    fn align_to(&self, align: usize) -> Result<Layout, LayoutErr> {
        Layout::from_size_align(self.size(), cmp::max(self.align(), align))
    }

    fn pad_to_align(&self) -> Layout {
        let pad = padding_needed_for(self, self.align());
        let new_size = self.size() + pad;
        Layout::from_size_align(new_size, self.align()).unwrap()
    }

    fn array<T>(n: usize) -> Result<Layout, LayoutErr> {
        repeat(&Layout::new::<T>(), n).map(|(k, offs)| {
            debug_assert!(offs == mem::size_of::<T>());
            k
        })
    }

    fn extend(&self, next: Self) -> Result<(Layout, usize), LayoutErr> {
        let new_align = cmp::max(self.align(), next.align());
        let pad = padding_needed_for(self, next.align());

        let offset = self.size().checked_add(pad).ok_or(layout_err())?;
        let new_size = offset.checked_add(next.size()).ok_or(layout_err())?;

        let layout = Layout::from_size_align(new_size, new_align)?;
        Ok((layout, offset))
    }
}

fn padding_needed_for(zelf: &Layout, align: usize) -> usize {
    let len = zelf.size();
    let len_rounded_up = len.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);
    len_rounded_up.wrapping_sub(len)
}

fn repeat(zelf: &Layout, n: usize) -> Result<(Layout, usize), LayoutErr> {
    let padded_size = zelf.size() + padding_needed_for(zelf, zelf.align());
    let alloc_size = padded_size.checked_mul(n).ok_or(layout_err())?;

    unsafe {
        Ok((
            Layout::from_size_align_unchecked(alloc_size, zelf.align()),
            padded_size,
        ))
    }
}

fn layout_err() -> LayoutErr {
    unsafe { transmute(()) }
}

mod sealed {
    #[allow(unreachable_pub)]
    pub trait FloatToInt<Int> {
        unsafe fn to_int_unchecked(self) -> Int;
    }

    macro_rules! impl_float_to_int {
        ($float:ident => $($int:ident)+) => {$(
            impl FloatToInt<$int> for $float {
                unsafe fn to_int_unchecked(self) -> $int {
                    self as $int
                }
            }
        )+}
    }

    impl_float_to_int!(f32 => u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
    impl_float_to_int!(f64 => u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
}

#[ext(f32_v1_44)]
pub impl f32
where
    Self: Sealed<f32>,
{
    unsafe fn to_int_unchecked<Int>(self) -> Int
    where
        f32: sealed::FloatToInt<Int>,
    {
        sealed::FloatToInt::to_int_unchecked(self)
    }
}

#[ext(f64_v1_44)]
pub impl f64
where
    Self: Sealed<f64>,
{
    unsafe fn to_int_unchecked<Int>(self) -> Int
    where
        f64: sealed::FloatToInt<Int>,
    {
        sealed::FloatToInt::to_int_unchecked(self)
    }
}
