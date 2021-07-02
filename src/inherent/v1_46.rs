use easy_ext::ext;

use crate::inherent::Sealed;

macro_rules! impl_int_v1_46 {
    ($($signed_type:ty, $unsigned_type:ty),+) => {$(
        #[ext]
        pub impl $signed_type where Self: Sealed<$signed_type>, {
            fn leading_ones(self) -> u32 {
                (self as $unsigned_type).leading_ones()
            }

            fn trailing_ones(self) -> u32 {
                (self as $unsigned_type).trailing_ones()
            }
        }

        #[ext]
        pub impl $unsigned_type where Self: Sealed<$unsigned_type>, {
            fn leading_ones(self) -> u32 {
                (!self).leading_zeros()
            }

            fn trailing_ones(self) -> u32 {
                (!self).trailing_zeros()
            }
        }
    )*};
}

impl_int_v1_46![
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
];

#[ext]
pub impl<T> Option<T>
where
    Self: Sealed<Option<T>>,
{
    fn zip<U>(self, other: Option<U>) -> Option<(T, U)> {
        match (self, other) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
}
