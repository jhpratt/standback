use easy_ext::ext;

use crate::traits::Sealed;

macro_rules! impl_int_v1_46 {
    ($(($signed_trait_name:ident $signed_type:ty) ($unsigned_trait_name:ident $unsigned_type:ty))+) => {$(
        #[ext($signed_trait_name)]
        pub impl $signed_type where Self: Sealed<$signed_type>, {
            fn leading_ones(self) -> u32 {
                (self as $unsigned_type).leading_ones()
            }

            fn trailing_ones(self) -> u32 {
                (self as $unsigned_type).trailing_ones()
            }
        }

        #[ext($unsigned_trait_name)]
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
    (i8_v1_46 i8) (u8_v1_46 u8)
    (i16_v1_46 i16) (u16_v1_46 u16)
    (i32_v1_46 i32) (u32_v1_46 u32)
    (i64_v1_46 i64) (u64_v1_46 u64)
    (i128_v1_46 i128) (u128_v1_46 u128)
    (isize_v1_46 isize) (usize_v1_46 usize)
];

#[ext(Option_v1_46)]
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
