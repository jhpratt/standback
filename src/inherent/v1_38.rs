use core::time::Duration;

use easy_ext::ext;

use crate::inherent::Sealed;

#[ext]
pub impl<T> *const T
where Self: Sealed<*const T>
{
    fn cast<U>(self) -> *const U {
        self as _
    }
}

#[ext]
pub impl<T> *mut T
where Self: Sealed<*mut T>
{
    fn cast<U>(self) -> *mut U {
        self as _
    }
}

#[ext]
pub impl Duration
where Self: Sealed<Duration>
{
    fn as_secs_f32(&self) -> f32 {
        (self.as_secs() as f32) + (self.subsec_nanos() as f32) / 1_000_000_000.
    }

    fn as_secs_f64(&self) -> f64 {
        (self.as_secs() as f64) + (self.subsec_nanos() as f64) / 1_000_000_000.
    }

    fn div_f32(&self, rhs: f32) -> Self {
        Self::from_secs_f32(self.as_secs_f32() / rhs)
    }

    fn div_f64(&self, rhs: f64) -> Self {
        Self::from_secs_f64(self.as_secs_f64() / rhs)
    }

    fn from_secs_f32(secs: f32) -> Self {
        const MAX_NANOS_F32: f32 = ((u64::max_value() as u128 + 1) * 1_000_000_000) as f32;
        let nanos = secs * 1_000_000_000.;
        if !nanos.is_finite() {
            panic!("got non-finite value when converting float to duration");
        }
        if nanos >= MAX_NANOS_F32 {
            panic!("overflow when converting float to duration");
        }
        if nanos < 0.0 {
            panic!("underflow when converting float to duration");
        }
        let nanos = nanos as u128;
        Self::new((nanos / 1_000_000_000) as u64, (nanos % 1_000_000_000) as u32)
    }

    fn from_secs_f64(secs: f64) -> Self {
        const MAX_NANOS_F64: f64 = ((u64::max_value() as u128 + 1) * 1_000_000_000) as f64;
        let nanos = secs * 1_000_000_000.;
        if !nanos.is_finite() {
            panic!("got non-finite value when converting float to duration");
        }
        if nanos >= MAX_NANOS_F64 {
            panic!("overflow when converting float to duration");
        }
        if nanos < 0.0 {
            panic!("underflow when converting float to duration");
        }
        let nanos = nanos as u128;
        Self::new((nanos / 1_000_000_000) as u64, (nanos % 1_000_000_000) as u32)
    }

    fn mul_f32(&self, rhs: f32) -> Self {
        Self::from_secs_f32(rhs * self.as_secs_f32())
    }

    fn mul_f64(&self, rhs: f64) -> Self {
        Self::from_secs_f64(rhs * self.as_secs_f64())
    }
}

macro_rules! impl_euclid_for_signed {
    ($($type:ty)+) => {$(
        #[ext]
        pub impl $type where Self: Sealed<$type>, {
            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn rem_euclid(self, rhs: Self) -> Self {
                let r = self % rhs;
                if r < 0 {
                    if rhs < 0 {
                        r - rhs
                    } else {
                        r + rhs
                    }
                } else {
                    r
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn checked_rem_euclid(self, rhs: Self) -> Option<$type> {
                if rhs == 0 || (self == Self::min_value() && rhs == -1) {
                    None
                } else {
                    Some(self.rem_euclid(rhs))
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn wrapping_rem_euclid(self, rhs: Self) -> Self {
                self.overflowing_rem_euclid(rhs).0
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn overflowing_rem_euclid(self, rhs: Self) -> ($type, bool) {
                if self == Self::min_value() && rhs == -1 {
                    (0, true)
                } else {
                    (self.rem_euclid(rhs), false)
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn div_euclid(self, rhs: Self) -> Self {
                let q = self / rhs;
                if self % rhs < 0 {
                    return if rhs > 0 { q - 1 } else { q + 1 };
                }
                q
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn checked_div_euclid(self, rhs: Self) -> Option<$type> {
                if rhs == 0 || (self == Self::min_value() && rhs == -1) {
                    None
                } else {
                    Some(self.div_euclid(rhs))
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn wrapping_div_euclid(self, rhs: Self) -> Self {
                self.overflowing_div_euclid(rhs).0
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn overflowing_div_euclid(self, rhs: Self) -> ($type, bool) {
                if self == Self::min_value() && rhs == -1 {
                    (self, true)
                } else {
                    (self.div_euclid(rhs), false)
                }
            }
        }
    )+};
}

impl_euclid_for_signed![i8 i16 i32 i64 i128 isize];

macro_rules! impl_euclid_for_unsigned {
    ($($type:ty)+) => {$(
        #[ext]
        pub impl $type where Self: Sealed<$type>, {
            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn rem_euclid(self, rhs: Self) -> Self {
                self % rhs
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn checked_rem_euclid(self, rhs: Self) -> Option<$type> {
                if rhs == 0 {
                    None
                } else {
                    Some(self.rem_euclid(rhs))
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn wrapping_rem_euclid(self, rhs: Self) -> Self {
                self % rhs
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn overflowing_rem_euclid(self, rhs: Self) -> ($type, bool) {
                (self % rhs, false)
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn div_euclid(self, rhs: Self) -> Self {
                self / rhs
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn checked_div_euclid(self, rhs: Self) -> Option<$type> {
                if rhs == 0 {
                    None
                } else {
                    Some(self.div_euclid(rhs))
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn wrapping_div_euclid(self, rhs: Self) -> Self {
                self / rhs
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            fn overflowing_div_euclid(self, rhs: Self) -> ($type, bool) {
                (self / rhs, false)
            }
        }
    )+};
}

impl_euclid_for_unsigned![u8 u16 u32 u64 u128 usize];

macro_rules! euclid_float {
    ($($type:ty)+) => {$(
        #[cfg(feature = "std")]
        #[ext]
        pub impl $type where Self: Sealed<$type>, {
            #[must_use = "method returns a new number and does not mutate the original value"]
            fn rem_euclid(self, rhs: $type) -> $type {
                let r = self % rhs;
                if r < 0.0 { r + rhs.abs() } else { r }
            }

            #[must_use = "method returns a new number and does not mutate the original value"]
            fn div_euclid(self, rhs: $type) -> $type {
                let q = (self / rhs).trunc();
                if self % rhs < 0.0 {
                    return if rhs > 0.0 { q - 1.0 } else { q + 1.0 };
                }
                q
            }
        }
    )+};
}
euclid_float![f32 f64];
