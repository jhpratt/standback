use core::mem::MaybeUninit;
use core::ops::Bound;

use easy_ext::ext;

use crate::inherent::Sealed;

#[ext]
pub impl<T: Clone> Bound<T>
where Self: Sealed<Bound<T>>
{
    fn cloned(self) -> Self {
        match self {
            Bound::Unbounded => Bound::Unbounded,
            Bound::Included(x) => Bound::Included(x.clone()),
            Bound::Excluded(x) => Bound::Excluded(x.clone()),
        }
    }
}

#[ext]
pub impl<T> MaybeUninit<T>
where Self: Sealed<MaybeUninit<T>>
{
    unsafe fn assume_init_mut(&mut self) -> &mut T {
        &mut *self.as_mut_ptr()
    }

    unsafe fn assume_init_ref(&self) -> &T {
        &*self.as_ptr()
    }

    fn write(&mut self, value: T) -> &mut T {
        *self = MaybeUninit::new(value);
        unsafe { self.assume_init_mut() }
    }
}
