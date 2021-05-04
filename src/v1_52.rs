use core::char::DecodeUtf16;

use crate::traits::Sealed;

pub trait char_v1_52: Sealed<char> {
    const MAX: char;
    const REPLACEMENT_CHARACTER: char;
    const UNICODE_VERSION: (u8, u8, u8);
    fn decode_utf16<I: IntoIterator<Item = u16>>(iter: I) -> DecodeUtf16<I::IntoIter>;
    fn from_digit(num: u32, radix: u32) -> Option<char>;
    unsafe fn from_u32_unchecked(i: u32) -> char;
    fn from_u32(i: u32) -> Option<char>;
}

impl char_v1_52 for char {
    const MAX: char = '\u{10ffff}';
    const REPLACEMENT_CHARACTER: char = '\u{FFFD}';
    const UNICODE_VERSION: (u8, u8, u8) = {
        #[cfg(shim = "1.38")]
        {
            (11, 0, 0)
        }
        #[cfg(all(reexport = "1.38", shim = "1.44"))]
        {
            (12, 1, 0)
        }
        #[cfg(all(reexport = "1.44", shim = "1.45"))]
        {
            (13, 0, 0)
        }
        #[cfg(not(shim = "1.45"))]
        {
            core::char::UNICODE_VERSION
        }
    };

    #[inline]
    fn decode_utf16<I: IntoIterator<Item = u16>>(iter: I) -> DecodeUtf16<I::IntoIter> {
        core::char::decode_utf16(iter)
    }

    #[inline]
    fn from_digit(num: u32, radix: u32) -> Option<char> {
        core::char::from_digit(num, radix)
    }

    #[inline]
    unsafe fn from_u32_unchecked(i: u32) -> char {
        core::char::from_u32_unchecked(i)
    }

    #[inline]
    fn from_u32(i: u32) -> Option<char> {
        core::char::from_u32(i)
    }
}

pub trait Slice_v1_52<T>: Sealed<[T]> {
    fn partition_point<P: FnMut(&T) -> bool>(&self, pred: P) -> usize;
}

impl<T> Slice_v1_52<T> for [T] {
    fn partition_point<P: FnMut(&T) -> bool>(&self, mut pred: P) -> usize {
        let mut left = 0;
        let mut right = self.len();

        while left != right {
            let mid = left + (right - left) / 2;
            let value = unsafe { self.get_unchecked(mid) };
            if pred(value) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}
