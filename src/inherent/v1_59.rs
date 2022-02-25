use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

use easy_ext::ext;

use crate::inherent::Sealed;

#[ext]
pub impl<'a, T: 'a, E> Result<&'a T, E>
where Self: Sealed<Result<&'a T, E>>
{
    fn copied(self) -> Result<T, E>
    where T: Copy {
        self.map(|&t| t)
    }

    fn cloned(self) -> Result<T, E>
    where T: Clone {
        self.map(|t| t.clone())
    }
}

macro_rules! impl_nonzero {
    ($($t:ty)+) => {$(
        #[allow(unreachable_pub)] // false positive
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

impl_nonzero![NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize];
