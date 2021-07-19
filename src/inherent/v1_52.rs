use core::char::DecodeUtf16;

use easy_ext::ext;

use crate::inherent::Sealed;
use crate::pattern::{Pattern, ReverseSearcher, Searcher};

#[ext]
pub impl char
where Self: Sealed<char>
{
    const MAX: char = '\u{10ffff}';
    const REPLACEMENT_CHARACTER: char = '\u{FFFD}';
    const UNICODE_VERSION: (u8, u8, u8) = {
        #[cfg(before = "1.38")]
        {
            (11, 0, 0)
        }
        #[cfg(all(since = "1.38", before = "1.44"))]
        {
            (12, 1, 0)
        }
        #[cfg(all(since = "1.44", before = "1.45"))]
        {
            (13, 0, 0)
        }
        #[cfg(since = "1.45")]
        {
            core::char::UNICODE_VERSION
        }
    };

    fn decode_utf16<I: IntoIterator<Item = u16>>(iter: I) -> DecodeUtf16<I::IntoIter> {
        core::char::decode_utf16(iter)
    }

    fn from_digit(num: u32, radix: u32) -> Option<char> {
        core::char::from_digit(num, radix)
    }

    unsafe fn from_u32_unchecked(i: u32) -> char {
        core::char::from_u32_unchecked(i)
    }

    fn from_u32(i: u32) -> Option<char> {
        core::char::from_u32(i)
    }
}

#[ext]
pub impl<T> [T]
where Self: Sealed<[T]>
{
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

#[ext]
pub impl str
where Self: Sealed<str>
{
    fn rsplit_once<'a, P>(&'a self, delimiter: P) -> Option<(&'a str, &'a str)>
    where
        P: Pattern<'a>,
        <P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,
    {
        let (start, end) = delimiter.into_searcher(self).next_match_back()?;
        Some((&self[..start], &self[end..]))
    }

    fn split_once<'a, P: Pattern<'a>>(&'a self, delimiter: P) -> Option<(&'a str, &'a str)> {
        let (start, end) = delimiter.into_searcher(self).next_match()?;
        Some((&self[..start], &self[end..]))
    }
}
