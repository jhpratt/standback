#[cfg(feature = "alloc")] use alloc::boxed::Box;
#[cfg(feature = "alloc")] use alloc::vec::Vec;
use core::ops::{DerefMut, Range};

use easy_ext::ext;

use crate::inherent::Sealed;

#[ext]
pub impl<Idx: PartialOrd<Idx>> Range<Idx>
where Self: Sealed<Range<Idx>>
{
    fn is_empty(&self) -> bool {
        !(self.start < self.end)
    }
}

#[ext]
pub impl<T: DerefMut, E> Result<T, E>
where Self: Sealed<Result<T, E>>
{
    fn as_deref(&self) -> Result<&T::Target, &E> {
        self.as_ref().map(|t| t.deref())
    }

    fn as_deref_mut(&mut self) -> Result<&mut T::Target, &mut E> {
        self.as_mut().map(|t| t.deref_mut())
    }
}

#[cfg(feature = "alloc")]
#[ext]
pub impl<T> Vec<T>
where Self: Sealed<Vec<T>>
{
    fn leak<'a>(self) -> &'a mut [T]
    where T: 'a {
        Box::leak(self.into_boxed_slice())
    }
}
