mod pin;

pub use self::pin::Pin;
use core::time::Duration;
#[cfg(std)]
use std::collections::VecDeque;
#[cfg(all(std, target_family = "unix"))]
use std::{io, os::unix};

#[inline]
pub const fn identity<T>(x: T) -> T {
    x
}

pub trait Unpin {}
impl<'a, T: ?Sized + 'a> Unpin for &'a T {}
impl<'a, T: ?Sized + 'a> Unpin for &'a mut T {}

#[cfg(all(std, target_family = "unix"))]
pub trait UnixFileExt_v1_33: unix::fs::FileExt {
    fn read_exact_at(&self, mut buf: &mut [u8], mut offset: u64) -> io::Result<()> {
        while !buf.is_empty() {
            match self.read_at(buf, offset) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buf;
                    buf = &mut tmp[n..];
                    offset += n as u64;
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        if !buf.is_empty() {
            Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "failed to fill whole buffer",
            ))
        } else {
            Ok(())
        }
    }

    fn write_all_at(&self, mut buf: &[u8], mut offset: u64) -> io::Result<()> {
        while !buf.is_empty() {
            match self.write_at(buf, offset) {
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "failed to write whole buffer",
                    ));
                }
                Ok(n) => {
                    buf = &buf[n..];
                    offset += n as u64
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

#[cfg(all(std, target_family = "unix"))]
impl<F: unix::fs::FileExt> UnixFileExt_v1_33 for F {}

mod private_transpose {
    pub trait Sealed {}
    impl<T, E> Sealed for Option<Result<T, E>> {}
    impl<T, E> Sealed for Result<Option<T>, E> {}
}

pub trait Option_v1_33<T, E>: private_transpose::Sealed {
    fn transpose(self) -> Result<Option<T>, E>;
}

impl<T, E> Option_v1_33<T, E> for Option<Result<T, E>> {
    #[inline]
    fn transpose(self) -> Result<Option<T>, E> {
        match self {
            Some(Ok(x)) => Ok(Some(x)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }
}

pub trait Result_v1_33<T, E>: private_transpose::Sealed {
    fn transpose(self) -> Option<Result<T, E>>;
}

impl<T, E> Result_v1_33<T, E> for Result<Option<T>, E> {
    #[inline]
    fn transpose(self) -> Option<Result<T, E>> {
        match self {
            Ok(Some(x)) => Some(Ok(x)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(std)]
mod private_vec_deque {
    pub trait Sealed {}
    impl<T> Sealed for super::VecDeque<T> {}
}

#[cfg(std)]
pub trait VecDeque_v1_33<T>: private_vec_deque::Sealed {
    fn resize_with(&mut self, new_len: usize, generator: impl FnMut() -> T);
}

#[cfg(std)]
impl<T> VecDeque_v1_33<T> for VecDeque<T> {
    fn resize_with(&mut self, new_len: usize, generator: impl FnMut() -> T) {
        let len = self.len();

        if new_len > len {
            self.extend(core::iter::repeat_with(generator).take(new_len - len))
        } else {
            self.truncate(new_len);
        }
    }
}

mod private_duration {
    pub trait Sealed {}
    impl Sealed for super::Duration {}
}

pub trait Duration_v1_33: private_duration::Sealed {
    fn as_millis(&self) -> u128;
    fn as_micros(&self) -> u128;
    fn as_nanos(&self) -> u128;
}

impl Duration_v1_33 for Duration {
    #[inline]
    fn as_millis(&self) -> u128 {
        self.as_secs() as u128 * 1_000 + (self.subsec_nanos() / 1_000_000) as u128
    }

    #[inline]
    fn as_micros(&self) -> u128 {
        self.as_secs() as u128 * 1_000_000 + (self.subsec_nanos() / 1_000) as u128
    }

    #[inline]
    fn as_nanos(&self) -> u128 {
        self.as_secs() as u128 * 1_000_000_000 + self.subsec_nanos() as u128
    }
}
