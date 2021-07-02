#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::ops::DerefMut;
#[cfg(feature = "alloc")]
use core::ptr;

use easy_ext::ext;

use crate::traits::Sealed;

#[ext(Option_v1_40)]
pub impl<T: DerefMut> Option<T>
where
    Self: Sealed<Option<T>>,
{
    fn as_deref_mut(&mut self) -> Option<&mut T::Target> {
        self.as_mut().map(|t| t.deref_mut())
    }

    fn as_deref(&self) -> Option<&T::Target> {
        self.as_ref().map(|t| t.deref())
    }
}

#[ext(Option_v1_40_)]
pub impl<T> Option<Option<T>>
where
    Self: Sealed<Option<Option<T>>>,
{
    fn flatten(self) -> Option<T> {
        self.and_then(core::convert::identity)
    }
}

#[ext(f32_v1_40)]
pub impl f32
where
    Self: Sealed<f32>,
{
    fn to_be_bytes(self) -> [u8; 4] {
        self.to_bits().to_be_bytes()
    }

    fn to_le_bytes(self) -> [u8; 4] {
        self.to_bits().to_le_bytes()
    }

    fn to_ne_bytes(self) -> [u8; 4] {
        self.to_bits().to_ne_bytes()
    }

    fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self::from_bits(u32::from_be_bytes(bytes))
    }

    fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self::from_bits(u32::from_le_bytes(bytes))
    }

    fn from_ne_bytes(bytes: [u8; 4]) -> Self {
        Self::from_bits(u32::from_ne_bytes(bytes))
    }
}

#[ext(f64_v1_40)]
pub impl f64
where
    Self: Sealed<f64>,
{
    fn to_be_bytes(self) -> [u8; 8] {
        self.to_bits().to_be_bytes()
    }

    fn to_le_bytes(self) -> [u8; 8] {
        self.to_bits().to_le_bytes()
    }

    fn to_ne_bytes(self) -> [u8; 8] {
        self.to_bits().to_ne_bytes()
    }

    fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self::from_bits(u64::from_be_bytes(bytes))
    }

    fn from_le_bytes(bytes: [u8; 8]) -> Self {
        Self::from_bits(u64::from_le_bytes(bytes))
    }

    fn from_ne_bytes(bytes: [u8; 8]) -> Self {
        Self::from_bits(u64::from_ne_bytes(bytes))
    }
}

pub fn take<T: Default>(dest: &mut T) -> T {
    core::mem::replace(dest, T::default())
}

#[cfg(feature = "alloc")]
#[ext(slice_v1_40)]
pub impl<T: Copy> [T]
where
    Self: Sealed<[T]>,
{
    fn repeat(&self, n: usize) -> Vec<T> {
        if n == 0 {
            return Vec::new();
        }

        let mut buf = Vec::with_capacity(self.len().checked_mul(n).expect("capacity overflow"));

        buf.extend(self);
        {
            let mut m = n >> 1;
            while m > 0 {
                unsafe {
                    ptr::copy_nonoverlapping(
                        buf.as_ptr(),
                        (buf.as_mut_ptr() as *mut T).add(buf.len()),
                        buf.len(),
                    );
                    let buf_len = buf.len();
                    buf.set_len(buf_len * 2);
                }

                m >>= 1;
            }
        }

        let rem_len = self.len() * n - buf.len();
        if rem_len > 0 {
            unsafe {
                ptr::copy_nonoverlapping(
                    buf.as_ptr(),
                    (buf.as_mut_ptr() as *mut T).add(buf.len()),
                    rem_len,
                );
                let buf_cap = buf.capacity();
                buf.set_len(buf_cap);
            }
        }
        buf
    }
}
