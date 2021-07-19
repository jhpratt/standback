use core::ops::Range;

use easy_ext::ext;

use crate::inherent::Sealed;
#[ext]
pub impl<T> [T]
where Self: Sealed<[T]>
{
    fn as_ptr_range(&self) -> Range<*const T> {
        let start = self.as_ptr();
        let end = unsafe { start.add(self.len()) };
        start..end
    }

    fn as_mut_ptr_range(&mut self) -> Range<*mut T> {
        let start = self.as_mut_ptr();
        let end = unsafe { start.add(self.len()) };
        start..end
    }
}
