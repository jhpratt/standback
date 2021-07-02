use core::mem::ManuallyDrop;
use core::ptr;
#[cfg(feature = "std")]
use std::{
    sync::{Condvar, LockResult, MutexGuard, WaitTimeoutResult},
    time::{Duration, Instant},
};

use easy_ext::ext;

use crate::traits::Sealed;

#[cfg(feature = "std")]
fn new_wait_timeout_result(value: bool) -> WaitTimeoutResult {
    unsafe { core::mem::transmute(value) }
}

#[cfg(feature = "std")]
#[ext(Condvar_v1_42)]
pub impl Condvar
where
    Self: Sealed<Condvar>,
{
    fn wait_while<'a, T, F>(
        &self,
        mut guard: MutexGuard<'a, T>,
        mut condition: F,
    ) -> LockResult<MutexGuard<'a, T>>
    where
        F: FnMut(&mut T) -> bool,
    {
        while condition(&mut *guard) {
            guard = self.wait(guard)?;
        }
        Ok(guard)
    }

    fn wait_timeout_while<'a, T, F>(
        &self,
        mut guard: MutexGuard<'a, T>,
        dur: Duration,
        mut condition: F,
    ) -> LockResult<(MutexGuard<'a, T>, WaitTimeoutResult)>
    where
        F: FnMut(&mut T) -> bool,
    {
        let start = Instant::now();
        loop {
            if !condition(&mut *guard) {
                return Ok((guard, new_wait_timeout_result(false)));
            }
            let timeout = match dur.checked_sub(start.elapsed()) {
                Some(timeout) => timeout,
                None => return Ok((guard, new_wait_timeout_result(true))),
            };
            guard = self.wait_timeout(guard, timeout)?.0;
        }
    }
}

#[ext(ManuallyDrop_v1_42)]
pub impl<T> ManuallyDrop<T>
where
    Self: Sealed<ManuallyDrop<T>>,
{
    #[must_use = "if you don't need the value, you can use `ManuallyDrop::drop` instead"]
    unsafe fn take(slot: &mut ManuallyDrop<T>) -> T {
        ptr::read(slot as *mut _ as *const _)
    }
}

#[macro_export]
macro_rules! matches {
    ($expression:expr, $( $pattern:pat )|+ $(if $guard:expr)? $(,)?) => {
        match $expression {
            $( $pattern )|+ $(if $guard)? => true,
            _ => false
        }
    };
}
