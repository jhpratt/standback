#[cfg(feature = "alloc")] use alloc::rc::Rc;
#[cfg(feature = "alloc")] use alloc::vec::Vec;
use core::cmp::Ordering;
#[cfg(feature = "std")] use core::mem::transmute;
use core::num::FpCategory;
#[cfg(feature = "alloc")] use core::ops;
use core::time::Duration;
use core::{mem, u64};
#[cfg(feature = "std")]
use std::ffi::{OsStr, OsString};

use easy_ext::ext;

use crate::inherent::Sealed;

#[ext]
pub impl Ordering
where Self: Sealed<Ordering>
{
    #[must_use]
    fn is_eq(self) -> bool {
        self == Ordering::Equal
    }
    #[must_use]
    fn is_ne(self) -> bool {
        self != Ordering::Equal
    }
    #[must_use]
    fn is_lt(self) -> bool {
        self == Ordering::Less
    }
    #[must_use]
    fn is_gt(self) -> bool {
        self == Ordering::Greater
    }
    #[must_use]
    fn is_le(self) -> bool {
        self != Ordering::Greater
    }
    #[must_use]
    fn is_ge(self) -> bool {
        self != Ordering::Less
    }
}

#[ext]
pub impl<T> Option<T>
where Self: Sealed<Option<T>>
{
    fn insert(&mut self, value: T) -> &mut T {
        *self = Some(value);

        match self {
            Some(v) => v,
            None => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}

#[ext]
pub impl f32
where Self: Sealed<f32>
{
    fn is_subnormal(self) -> bool {
        self.classify() == FpCategory::Subnormal
    }
}

#[ext]
pub impl f64
where Self: Sealed<f64>
{
    fn is_subnormal(self) -> bool {
        self.classify() == FpCategory::Subnormal
    }
}

#[ext]
pub impl Duration
where Self: Sealed<Duration>
{
    const ZERO: Self = Self::from_nanos(0);

    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
    fn saturating_add(self, rhs: Self) -> Self {
        match self.checked_add(rhs) {
            Some(res) => res,
            None => Duration::from_secs(u64::MAX) + Duration::from_nanos(999_999_999),
        }
    }
    fn saturating_sub(self, rhs: Self) -> Self {
        match self.checked_sub(rhs) {
            Some(res) => res,
            None => Self::ZERO,
        }
    }
    fn saturating_mul(self, rhs: u32) -> Self {
        match self.checked_mul(rhs) {
            Some(res) => res,
            None => Duration::from_secs(u64::MAX) + Duration::from_nanos(999_999_999),
        }
    }
}

macro_rules! impl_integer {
    ($($t:ty)+) => {$(
        #[ext]
        pub impl $t where Self: Sealed<$t>, {
            const BITS: u32 = mem::size_of::<$t>() as u32 * 8;
        }
    )+};
}

impl_integer![u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize];

#[cfg(feature = "alloc")]
#[ext]
pub impl<T> Rc<T>
where Self: Sealed<Rc<T>>
{
    unsafe fn increment_strong_count(ptr: *const T) {
        let rc = mem::ManuallyDrop::new(Rc::<T>::from_raw(ptr));
        let _rc_clone = rc.clone();
    }
    unsafe fn decrement_strong_count(ptr: *const T) {
        drop(Rc::from_raw(ptr));
    }
}

#[cfg(feature = "std")]
#[ext]
pub impl OsStr
where Self: Sealed<OsStr>
{
    fn make_ascii_lowercase(&mut self) {
        unsafe { transmute::<_, &mut [u8]>(self).make_ascii_lowercase() }
    }
    fn make_ascii_uppercase(&mut self) {
        unsafe { transmute::<_, &mut [u8]>(self).make_ascii_uppercase() }
    }
    fn to_ascii_lowercase(&self) -> OsString {
        unsafe { transmute(transmute::<_, &[u8]>(self).to_ascii_lowercase()) }
    }
    fn to_ascii_uppercase(&self) -> OsString {
        unsafe { transmute(transmute::<_, &[u8]>(self).to_ascii_uppercase()) }
    }
    fn is_ascii(&self) -> bool {
        unsafe { transmute::<_, &[u8]>(self).is_ascii() }
    }
    fn eq_ignore_ascii_case<S: AsRef<OsStr>>(&self, other: S) -> bool {
        unsafe { transmute::<_, &[u8]>(self).eq_ignore_ascii_case(transmute(other.as_ref())) }
    }
}

#[cfg(feature = "alloc")]
#[ext]
pub impl<T: Clone> Vec<T>
where Self: Sealed<Vec<T>>
{
    fn extend_from_within<
        R: ops::RangeBounds<usize> + core::slice::SliceIndex<[T], Output = [T]>,
    >(
        &mut self,
        src: R,
    ) {
        let start = match src.start_bound() {
            ops::Bound::Included(&start) => start,
            ops::Bound::Excluded(start) => {
                start.checked_add(1).expect("attempted to index slice from after maximum usize")
            }
            ops::Bound::Unbounded => 0,
        };
        let end = match src.end_bound() {
            ops::Bound::Included(end) => {
                end.checked_add(1).expect("attempted to index slice up to maximum usize")
            }
            ops::Bound::Excluded(&end) => end,
            ops::Bound::Unbounded => self.len(),
        };
        if start > end {
            panic!("slice index starts at {} but ends at {}", start, end);
        }
        if end > self.len() {
            panic!("range end index {} out of range for slice of length {}", end, self.len());
        }
        self.reserve(end - start);

        let ptr = self.as_mut_ptr();
        let spare_ptr = unsafe { ptr.add(self.len()) } as _;

        let this = unsafe { core::slice::from_raw_parts_mut(ptr, self.len()) };
        let spare =
            unsafe { core::slice::from_raw_parts_mut(spare_ptr, self.capacity() - self.len()) };

        unsafe { this.get_unchecked(src) }
            .iter()
            .cloned()
            .zip(spare.iter_mut())
            .map(|(src, dst)| *dst = core::mem::MaybeUninit::new(src))
            .for_each(|_| unsafe { self.set_len(self.len() + 1) });
    }
}
