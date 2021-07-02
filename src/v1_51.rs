mod slice;
mod str;

#[cfg(feature = "alloc")]
use alloc::sync::Arc;
use core::iter::Peekable;
#[cfg(feature = "alloc")]
use core::mem;
use core::task::Poll;
#[cfg(feature = "std")]
use std::io::{Seek, SeekFrom};

use easy_ext::ext;

use crate::pattern::Pattern;
use crate::traits::Sealed;

#[cfg(feature = "alloc")]
#[ext(Arc_v1_51)]
pub impl<T> Arc<T>
where
    Self: Sealed<Arc<T>>,
{
    unsafe fn decrement_strong_count(ptr: *const T) {
        drop(Arc::from_raw(ptr));
    }

    unsafe fn increment_strong_count(ptr: *const T) {
        let arc = mem::ManuallyDrop::new(Arc::<T>::from_raw(ptr));
        let _arc_clone: mem::ManuallyDrop<_> = arc.clone();
    }
}

#[ext(Peekable_v1_51)]
pub impl<I: Iterator> Peekable<I>
where
    Self: Sealed<Peekable<I>>,
{
    fn next_if(&mut self, func: impl FnOnce(&I::Item) -> bool) -> Option<I::Item> {
        if func(self.peek()?) {
            self.next()
        } else {
            None
        }
    }

    fn next_if_eq<T>(&mut self, expected: &T) -> Option<I::Item>
    where
        T: ?Sized,
        I::Item: PartialEq<T>,
    {
        self.next_if(|next| next == expected)
    }
}

#[cfg(feature = "std")]
#[ext(Seek_v1_51)]
pub impl<T: Seek> T
where
    Self: Sealed<T>,
{
    fn stream_position(&mut self) -> std::io::Result<u64> {
        self.seek(SeekFrom::Current(0))
    }
}

#[ext(Slice_v1_51)]
pub impl<T> [T]
where
    Self: Sealed<[T]>,
{
    fn fill_with<F>(&mut self, mut f: F)
    where
        F: FnMut() -> T,
    {
        for el in self {
            *el = f();
        }
    }

    fn split_inclusive_mut<F>(&mut self, pred: F) -> slice::SplitInclusiveMut<'_, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        slice::SplitInclusiveMut::new(self, pred)
    }

    fn split_inclusive<F>(&self, pred: F) -> slice::SplitInclusive<'_, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        slice::SplitInclusive::new(self, pred)
    }

    #[must_use = "returns the subslice without modifying the original"]
    fn strip_prefix(&self, prefix: &[T]) -> Option<&[T]>
    where
        T: PartialEq,
    {
        let n = prefix.len();
        if n <= self.len() {
            let (head, tail) = self.split_at(n);
            if head == prefix {
                return Some(tail);
            }
        }
        None
    }

    #[must_use = "returns the subslice without modifying the original"]
    fn strip_suffix(&self, suffix: &[T]) -> Option<&[T]>
    where
        T: PartialEq,
    {
        let (len, n) = (self.len(), suffix.len());
        if n <= len {
            let (head, tail) = self.split_at(len - n);
            if tail == suffix {
                return Some(head);
            }
        }
        None
    }
}

#[cfg(feature = "alloc")]
pub trait Wake {
    fn wake(self: Arc<Self>);
    #[cfg(since = "1.41")]
    fn wake_by_ref(self: &Arc<Self>) {
        self.clone().wake();
    }
}

macro_rules! impl_integer {
    ($(($trait_name:ident $int:ty => $uint:ty))+) => {$(
        #[ext($trait_name)]
        pub impl $int where Self: Sealed<$int>, {
            fn unsigned_abs(self) -> $uint {
                 self.wrapping_abs() as $uint
            }
        }
    )+};
}

impl_integer! {
    (i8_v1_51 i8 => u8)
    (i16_v1_51 i16 => u16)
    (i32_v1_51 i32 => u32)
    (i64_v1_51 i64 => u64)
    (i128_v1_51 i128 => u128)
}

#[ext(Poll_v1_51)]
pub impl<T, E> Poll<Result<T, E>>
where
    Self: Sealed<Poll<Result<T, E>>>,
{
    fn map_ok<U, F>(self, f: F) -> Poll<Result<U, E>>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Poll::Ready(Ok(t)) => Poll::Ready(Ok(f(t))),
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }

    fn map_err<U, F>(self, f: F) -> Poll<Result<T, U>>
    where
        F: FnOnce(E) -> U,
    {
        match self {
            Poll::Ready(Ok(t)) => Poll::Ready(Ok(t)),
            Poll::Ready(Err(e)) => Poll::Ready(Err(f(e))),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[ext(str_v1_51)]
pub impl str
where
    Self: Sealed<str>,
{
    fn split_inclusive<'a, P: Pattern<'a>>(&'a self, pat: P) -> str::SplitInclusive<'a, P> {
        str::SplitInclusive(str::SplitInternal {
            start: 0,
            end: self.len(),
            matcher: pat.into_searcher(self),
            allow_trailing_empty: false,
            finished: false,
        })
    }
}
