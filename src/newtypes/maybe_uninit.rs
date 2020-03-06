use core::mem;

// There is no good way to reimplement `core::mem::MaybeUninit` without untagged unions, which
// cannot be created for non-`Copy` types in 1.31, meaning that the interface to `MaybeUninit` in
// this crate needs to be `unsafe`.
#[repr(transparent)]
#[derive(Copy)]
pub struct MaybeUninit<T> {
    value: mem::MaybeUninit<T>,
}

impl<T: Copy> Clone for MaybeUninit<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        MaybeUninit { value: self.value.clone() }
    }
}

impl<T> MaybeUninit<T> {
    #[inline(always)]
    pub fn new(val: T) -> MaybeUninit<T> {
        MaybeUninit {
            value: mem::MaybeUninit::new(val),
        }
    }

    #[inline(always)]
    pub unsafe fn uninit() -> MaybeUninit<T> {
        MaybeUninit {
            value: mem::MaybeUninit::uninit(),
        }
    }

    #[inline]
    pub unsafe fn zeroed() -> MaybeUninit<T> {
        MaybeUninit {
            value: mem::MaybeUninit::zeroed(),
        }
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        self.value.as_ptr()
    }

    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.value.as_mut_ptr()
    }

    #[inline(always)]
    pub unsafe fn assume_init(self) -> T {
        self.value.assume_init()
    }
}
