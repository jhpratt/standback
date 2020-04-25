use core::iter::FusedIterator;

pub mod f32 {
    pub const LOG10_2: f32 = 0.301029995663981195213738894724493027_f32;
    pub const LOG2_10: f32 = 3.32192809488736234787031942948939018_f32;
}

pub mod f64 {
    pub const LOG10_2: f64 = 0.301029995663981195213738894724493027_f64;
    pub const LOG2_10: f64 = 3.32192809488736234787031942948939018_f64;
}

#[inline]
pub fn once_with<A, F: FnOnce() -> A>(gen: F) -> OnceWith<F> {
    OnceWith { gen: Some(gen) }
}

#[derive(Copy, Clone, Debug)]
pub struct OnceWith<F> {
    gen: Option<F>,
}

impl<A, F: FnOnce() -> A> Iterator for OnceWith<F> {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<A> {
        let f = self.gen.take()?;
        Some(f())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.gen.iter().size_hint()
    }
}

impl<A, F: FnOnce() -> A> DoubleEndedIterator for OnceWith<F> {
    fn next_back(&mut self) -> Option<A> {
        self.next()
    }
}

impl<A, F: FnOnce() -> A> ExactSizeIterator for OnceWith<F> {
    fn len(&self) -> usize {
        self.gen.iter().len()
    }
}

impl<A, F: FnOnce() -> A> FusedIterator for OnceWith<F> {}

#[cfg(since_1_32)]
pub mod primitive {
    pub use {
        u8,
        u16,
        u32,
        u64,
        u128,
        usize,
        i8,
        i16,
        i32,
        i64,
        i128,
        isize,
        f32,
        f64,
        bool,
        char,
        str,
    };
}

mod private_f32 {
    pub trait Sealed {}
    impl Sealed for f32 {}
}

pub trait f32_v1_43: private_f32::Sealed {
    const RADIX: u32;
    const MANTISSA_DIGITS: u32;
    const DIGITS: u32;
    const EPSILON: f32;
    const MIN: f32;
    const MIN_POSITIVE: f32;
    const MAX: f32;
    const MIN_EXP: i32;
    const MAX_EXP: i32;
    const MIN_10_EXP: i32;
    const MAX_10_EXP: i32;
    const NAN: f32;
    const INFINITY: f32;
    const NEG_INFINITY: f32;
}

impl f32_v1_43 for f32 {
    const RADIX: u32 = 2;
    const MANTISSA_DIGITS: u32 = 24;
    const DIGITS: u32 = 6;
    const EPSILON: f32 = 1.19209290e-07_f32;
    const MIN: f32 = -3.40282347e+38_f32;
    const MIN_POSITIVE: f32 = 1.17549435e-38_f32;
    const MAX: f32 = 3.40282347e+38_f32;
    const MIN_EXP: i32 = -125;
    const MAX_EXP: i32 = 128;
    const MIN_10_EXP: i32 = -37;
    const MAX_10_EXP: i32 = 38;
    const NAN: f32 = 0.0_f32 / 0.0_f32;
    const INFINITY: f32 = 1.0_f32 / 0.0_f32;
    const NEG_INFINITY: f32 = -1.0_f32 / 0.0_f32;
}

mod private_f64 {
    pub trait Sealed {}
    impl Sealed for f64 {}
}

pub trait f64_v1_43: private_f64::Sealed {
    const RADIX: u32;
    const MANTISSA_DIGITS: u32;
    const DIGITS: u32;
    const EPSILON: f64;
    const MIN: f64;
    const MIN_POSITIVE: f64;
    const MAX: f64;
    const MIN_EXP: i32;
    const MAX_EXP: i32;
    const MIN_10_EXP: i32;
    const MAX_10_EXP: i32;
    const NAN: f64;
    const INFINITY: f64;
    const NEG_INFINITY: f64;
}

impl f64_v1_43 for f64 {
    const RADIX: u32 = 2;
    const MANTISSA_DIGITS: u32 = 53;
    const DIGITS: u32 = 15;
    const EPSILON: f64 =2.2204460492503131e-16_f64;
    const MIN: f64 = -1.7976931348623157e+308_f64;
    const MIN_POSITIVE: f64 = 2.2250738585072014e-308_f64;
    const MAX: f64 = 1.7976931348623157e+308_f64;
    const MIN_EXP: i32 = -1021;
    const MAX_EXP: i32 = 1024;
    const MIN_10_EXP: i32 = -307;
    const MAX_10_EXP: i32 = 308;
    const NAN: f64 = 0.0_f64 / 0.0_f64;
    const INFINITY: f64 = 1.0_f64 / 0.0_f64;
    const NEG_INFINITY: f64 = -1.0_f64 / 0.0_f64;
}

mod private_int {
    pub trait Sealed {}
    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for u128 {}
    impl Sealed for usize {}
    impl Sealed for i8 {}
    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for i128 {}
    impl Sealed for isize {}
}

pub trait int_v1_43: private_int::Sealed {
    const MIN: Self;
    const MAX: Self;
}

macro_rules! impl_int_v1_43 {
    ($($signed_type:ty, $unsigned_type:ty),*) => {$(
        impl int_v1_43 for $signed_type {
            const MIN: Self = !0 ^ ((!0 as $unsigned_type) >> 1) as Self;
            const MAX: Self = !Self::MIN;
        }

        impl int_v1_43 for $unsigned_type {
            const MIN: Self = 0;
            const MAX: Self = !0;
        }
    )*}
}

impl_int_v1_43![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];
