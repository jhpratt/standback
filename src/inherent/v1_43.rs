use easy_ext::ext;

use crate::inherent::Sealed;

#[ext]
pub impl f32
where Self: Sealed<f32>
{
    const DIGITS: u32 = 6;
    const EPSILON: f32 = 1.19209290e-07_f32;
    const INFINITY: f32 = 1.0_f32 / 0.0_f32;
    const MANTISSA_DIGITS: u32 = 24;
    const MAX: f32 = 3.40282347e+38_f32;
    const MAX_10_EXP: i32 = 38;
    const MAX_EXP: i32 = 128;
    const MIN: f32 = -3.40282347e+38_f32;
    const MIN_10_EXP: i32 = -37;
    const MIN_EXP: i32 = -125;
    const MIN_POSITIVE: f32 = 1.17549435e-38_f32;
    const NAN: f32 = 0.0_f32 / 0.0_f32;
    const NEG_INFINITY: f32 = -1.0_f32 / 0.0_f32;
    const RADIX: u32 = 2;
}

#[ext]
pub impl f64
where Self: Sealed<f64>
{
    const DIGITS: u32 = 15;
    const EPSILON: f64 = 2.2204460492503131e-16_f64;
    const INFINITY: f64 = 1.0_f64 / 0.0_f64;
    const MANTISSA_DIGITS: u32 = 53;
    const MAX: f64 = 1.7976931348623157e+308_f64;
    const MAX_10_EXP: i32 = 308;
    const MAX_EXP: i32 = 1024;
    const MIN: f64 = -1.7976931348623157e+308_f64;
    const MIN_10_EXP: i32 = -307;
    const MIN_EXP: i32 = -1021;
    const MIN_POSITIVE: f64 = 2.2250738585072014e-308_f64;
    const NAN: f64 = 0.0_f64 / 0.0_f64;
    const NEG_INFINITY: f64 = -1.0_f64 / 0.0_f64;
    const RADIX: u32 = 2;
}

macro_rules! impl_int_v1_43 {
    ($($signed_type:ty, $unsigned_type:ty),+) => {$(
        #[ext]
        impl $signed_type where Self: Sealed<$signed_type>, {
            const MIN: Self = !0 ^ ((!0 as $unsigned_type) >> 1) as Self;
            const MAX: Self = !Self::MIN;
        }

        #[ext]
        impl $unsigned_type where Self: Sealed<$unsigned_type>, {
            const MIN: Self = 0;
            const MAX: Self = !0;
        }
    )+}
}

impl_int_v1_43![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];
