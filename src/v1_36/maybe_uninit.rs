use core::mem::{self, ManuallyDrop};

#[repr(transparent)]
#[derive(Copy)]
pub struct MaybeUninit<T> {
    value: ManuallyDrop<T>,
}

impl<T: Copy> Clone for MaybeUninit<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> MaybeUninit<T> {
    #[inline(always)]
    pub fn new(val: T) -> MaybeUninit<T> {
        MaybeUninit {
            value: ManuallyDrop::new(val),
        }
    }

    #[inline(always)]
    pub fn uninit() -> MaybeUninit<T> {
        MaybeUninit {
            value: unsafe { mem::uninitialized() },
        }
    }

    #[inline]
    pub fn zeroed() -> MaybeUninit<T> {
        let mut u = MaybeUninit::<T>::uninit();
        unsafe {
            u.as_mut_ptr().write_bytes(0u8, 1);
        }
        u
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        &*self.value as *const T
    }

    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        &mut *self.value as *mut T
    }

    #[inline(always)]
    pub unsafe fn assume_init(self) -> T {
        ManuallyDrop::into_inner(self.value)
    }
}
