#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::ops::{DerefMut, Range};

use easy_ext::ext;

use crate::traits::Sealed;

#[ext(Range_v1_47)]
pub impl<Idx: PartialOrd<Idx>> Range<Idx>
where
    Self: Sealed<Range<Idx>>,
{
    fn is_empty(&self) -> bool {
        !(self.start < self.end)
    }
}

#[ext(Result_v1_47)]
pub impl<T: DerefMut, E> Result<T, E>
where
    Self: Sealed<Result<T, E>>,
{
    fn as_deref(&self) -> Result<&T::Target, &E> {
        self.as_ref().map(|t| t.deref())
    }

    fn as_deref_mut(&mut self) -> Result<&mut T::Target, &mut E> {
        self.as_mut().map(|t| t.deref_mut())
    }
}

#[cfg(feature = "alloc")]
#[ext(Vec_v1_47)]
pub impl<T> Vec<T>
where
    Self: Sealed<Vec<T>>,
{
    fn leak<'a>(self) -> &'a mut [T]
    where
        T: 'a,
    {
        Box::leak(self.into_boxed_slice())
    }
}

pub(crate) mod f32 {
    pub const TAU: f32 = 6.28318530717958647692528676655900577_f32;
}

pub(crate) mod f64 {
    pub const TAU: f64 = 6.28318530717958647692528676655900577_f64;
}
