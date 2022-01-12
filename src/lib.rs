#![cfg_attr(not(feature = "std"), no_std)]
#![deny(rust_2018_idioms, unused_qualifications)]
#![warn(unreachable_pub)]
#![allow(non_camel_case_types, unstable_name_collisions, clippy::all)]

/*!
Standback backports a number of methods, structs, and macros that have been stabilized in the Rust
standard library since 1.36.0. This allows crate authors to depend on Standback rather than forcing
downstream users to upgrade their compiler (or not use the new feature at all).

Due to a variety of restrictions in the Rust, it is not possible to implement everything that has
been stabilized.

# Usage

For most cases, importing the shims should suffice.

```rust,no_run
use standback::shim::*;
```

If you are using anything that would normally have to be imported, just use the `standback` crate
instead of `core`, `alloc`, or `std`.

```rust,no_run
use standback::mem::take;
```

It is _highly_ recommended to use `#![allow(unstable_name_collisions)]`, as that's the whole point
of this crate. Just be extra-careful to not do it for anything that _can't_ be backported.

# `#![no_std]` support

By default, there standard library is used where necessary. If support for `#![no_std]` is required,
use `default-features = false`.

Items that require an allocator are gated under the `alloc` feature, which is enabled by default via
the `std` feature.

# Minimum supported Rust version

By default, this crate has a minimum supported Rust version of 1.36. If you do not require a MSRV
this low, you can raise it by using `default-features = false` (be sure to re-enable `std` or
`alloc` if you need it) and enabling a `msrv-1.XX` feature flag, substituting the appropriate
version; the standback crate is not guaranteed to work (let alone produce a reasonable result) if no
MSRV is declared when `default-features = false`. All versions up to the most recent stable release
of a compiler are supported.

Note that items stabilized prior to the declared MSRV _will not_ be re-exported.

# Inherent items

The following methods and constants are available via the prelude. For brevity, `i*` is `i8`, `i16`,
`i32`, `i64`, `i128`, and `isize`; `u*` is `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`.

## 1.58

```text
Metadata::is_symlink
Path::is_symlink
{i*, u*}::saturating_div
Option::unwrap_unchecked
Result::unwrap_unchecked
Result::unwrap_err_unchecked
NonZeroU*::is_power_of_two
```

## 1.57

```text
Iterator::map_while
```

## 1.56

None :(

## 1.55

```text
prelude::rust_2015
prelude::rust_2018
prelude::rust_2021
Bounded::cloned
MaybeUninit::assume_init_mut
MaybeUninit::assume_init_ref
MaybeUninit::write
```

## 1.54

```text
BTreeMap::into_keys
BTreeMap::into_values
HashMap::into_keys
HashMap::into_values
VecDeque::binary_search
VecDeque::binary_search_by
VecDeque::binary_search_by_key
VecDeque::partition_point
```

## 1.53

```text
Ordering::is_eq
Duration::ZERO
Duration::is_zero
Duration::saturating_add
Duration::saturating_mul
Duration::saturating_sub
Option::insert
Ordering::is_eq
Ordering::is_ge
Ordering::is_gt
Ordering::is_le
Ordering::is_lt
Ordering::is_ne
OsStr::make_ascii_lowercase
OsStr::make_ascii_uppercase
OsStr::to_ascii_lowercase
OsStr::to_ascii_uppercase
OsStr::is_ascii
OsStr::eq_ignore_ascii_case
Rc::decrement_strong_count
Rc::increment_strong_count
Vec::extend_from_within
{f32, f64}::is_subnormal
{i*, u*}::BITS
```

## 1.52

```text
char::MAX
char::REPLACEMENT_CHARACTER
char::UNICODE_VERSION
char::decode_utf16
char::from_digit
char::from_u32_unchecked
char::from_u32
slice::partition_point
str::rsplit_once
str::split_once
```

## 1.51

```text
Arc::decrement_strong_count
Arc::increment_strong_count
Peekable::next_if_eq
Peekable::next_if
Seek::stream_position
slice::fill_with
slice::split_inclusive_mut
slice::split_inclusive
slice::strip_prefix
slice::strip_suffix
str::split_inclusive
i*::unsigned_abs
Poll::map_ok
Poll::map_err
```

## 1.50

```text
bool::then
btree_map::Entry::or_insert_with_key
hash_map::Entry::or_insert_with_key
{f32, f64}::clamp
Ord::clamp
RefCell::take
slice::fill
UnsafeCell::get_mut
```

## 1.49

```text
slice::select_nth_unstable
slice::select_nth_unstable_by
slice::select_nth_unstable_by_key
```

## 1.48

```text
slice::as_ptr_range
slice::as_mut_ptr_range
```

## 1.47

```text
Range::is_empty
Result::as_deref
Result::as_deref_mut
Vec::leak
f32::TAU
f64::TAU
```

## 1.46

```text
{i*, u*}::leading_ones
{i*, u*}::trailing_ones
Option::zip
```

## 1.45

```text
i*::saturating_abs
i*::saturating_neg
str::strip_prefix
str::strip_suffix
```

## 1.44

```text
PathBuf::with_capacity
PathBuf::capacity
PathBuf::clear
PathBuf::reserve
PathBuf::reserve_exact
PathBuf::shrink_to_fit
Layout::align_to
Layout::pad_to_align
Layout::array
Layout::extend
{f32, f64}::to_int_unchecked
```

## 1.43

```text
{f32, f64}::RADIX
{f32, f64}::MANTISSA_DIGITS
{f32, f64}::DIGITS
{f32, f64}::EPSILON
{f32, f64}::MIN
{f32, f64}::MIN_POSITIVE
{f32, f64}::MAX
{f32, f64}::MIN_EXP
{f32, f64}::MAX_EXP
{f32, f64}::MIN_10_EXP
{f32, f64}::MAX_10_EXP
{f32, f64}::NAN
{f32, f64}::INFINITY
{f32, f64}::NEG_INFINITY
{i*, u*}::MIN
{i*, u*}::MAX
```

## 1.42

```text
CondVar::wait_while
CondVar::wait_timeout_while
ManuallyDrop::take
```

## 1.41

```text
Result::map_or
Result::map_or_else
```

## 1.40

```text
Option::as_deref
Option::as_deref_mut
{f32, f64}::to_be_bytes
{f32, f64}::to_le_bytes
{f32, f64}::to_ne_bytes
{f32, f64}::from_be_bytes
{f32, f64}::from_le_bytes
{f32, f64}::from_ne_bytes
slice::repeat
```

## 1.39

None :(

## 1.38

```text
<*const T>::cast
<*mut T>::cast
Duration::as_secs_f32
Duration::as_secs_f64
Duration::div_f32
Duration::div_f64
Duration::from_secs_f32
Duration::from_secs_f64
Duration::mul_f32
Duration::mul_f64
{i*, u*}::rem_euclid
{i*, u*}::checked_rem_euclid
{i*, u*}::wrapping_rem_euclid
{i*, u*}::overflowing_rem_euclid
{i*, u*}::div_euclid
{i*, u*}::checked_div_euclid
{i*, u*}::wrapping_div_euclid
{i*, u*}::overflowing_div_euclid
{f32, f64}::rem_euclid
{f32, f64}::div_euclid
```

## 1.37

```text
Cell::from_mut
Cell<[T]>::as_slice_of_cells
DoubleEndedIterator::nth_back
Option::xor
slice::copy_within
```

# Free items

```text
array::from_ref // 1.53
array::from_mut // 1.53
cmp::min_by // 1.53
cmp::max_by // 1.53
cmp::min_by_key // 1.53
cmp::max_by_key // 1.53
task::Wake // 1.51
future::pending // 1.48
future::ready // 1.48
{f32, f64}::consts::TAU // 1.47
char::UNICODE_VERSION // 1.45
{f32, f64}::consts::LOG10_2 // 1.43
{f32, f64}::consts::LOG2_10 // 1.43
iter::once_with // 1.43
mem::take // 1.40
```

# Prelude macros (located in `standback::shim`)

```text
matches! // 1.42
todo! // 1.39
```
*/

#[allow(unused_extern_crates)]
#[cfg(feature = "alloc")]
extern crate alloc;

mod inherent;
mod pattern;

mod free {
    #[cfg(shim = "1.40")] pub(crate) mod v1_40;
    #[cfg(shim = "1.43")] pub(crate) mod v1_43;
    #[cfg(shim = "1.47")] pub(crate) mod v1_47;
    #[cfg(shim = "1.48")] pub(crate) mod v1_48;
    #[cfg(shim = "1.51")] pub(crate) mod v1_51;
    #[cfg(shim = "1.53")] pub(crate) mod v1_53;
}

#[doc(hidden)]
pub mod shim {
    #[cfg(shim = "1.39")]
    pub use core::unimplemented as todo;

    pub use crate::inherent::*;
    #[cfg(shim = "1.42")] pub use crate::matches;
}
#[doc(hidden)]
pub mod prelude {
    #[cfg(not(feature = "std"))]
    pub use core::prelude::v1 as rust_2015;
    #[cfg(feature = "std")]
    pub use std::prelude::v1 as rust_2015;

    pub use rust_2015 as rust_2018;

    pub mod rust_2021 {
        pub use core::convert::{TryFrom, TryInto};
        pub use core::iter::FromIterator;

        pub use crate::prelude::rust_2015::*;
    }
}
#[doc(hidden)]
pub mod mem {
    #[cfg(reexport = "1.40")] pub use core::mem::take;

    #[cfg(shim = "1.40")]
    pub use crate::free::v1_40::mem::*;
}
#[doc(hidden)]
pub mod iter {
    #[cfg(reexport = "1.57")]
    pub use core::iter::MapWhile;
    #[cfg(reexport = "1.43")]
    pub use core::iter::{once_with, OnceWith};

    #[cfg(shim = "1.43")]
    pub use crate::free::v1_43::iter::*;
    #[cfg(shim = "1.57")]
    pub use crate::inherent::MapWhile;
}
#[doc(hidden)]
pub mod task {
    #[cfg(all(reexport = "1.51", feature = "alloc"))]
    pub use alloc::task::Wake;

    #[cfg(all(shim = "1.51", feature = "alloc"))]
    pub use crate::free::v1_51::task::*;
}
#[doc(hidden)]
pub mod f32 {
    pub mod consts {
        #[cfg(reexport = "1.47")]
        pub use core::f32::consts::TAU;
        #[cfg(reexport = "1.43")]
        pub use core::f32::consts::{LOG10_2, LOG2_10};

        #[cfg(shim = "1.43")]
        pub use crate::free::v1_43::f32::consts::*;
        #[cfg(shim = "1.47")]
        pub use crate::free::v1_47::f32::consts::*;
    }
}
#[doc(hidden)]
pub mod f64 {
    pub mod consts {
        #[cfg(reexport = "1.47")]
        pub use core::f64::consts::TAU;
        #[cfg(reexport = "1.43")]
        pub use core::f64::consts::{LOG10_2, LOG2_10};

        #[cfg(shim = "1.43")]
        pub use crate::free::v1_43::f64::consts::*;
        #[cfg(shim = "1.47")]
        pub use crate::free::v1_47::f64::consts::*;
    }
}
#[doc(hidden)]
pub mod char {
    #[cfg(shim = "1.38")]
    pub const UNICODE_VERSION: (u8, u8, u8) = (11, 0, 0);
    #[cfg(all(reexport = "1.38", shim = "1.44"))]
    pub const UNICODE_VERSION: (u8, u8, u8) = (12, 1, 0);
    #[cfg(all(reexport = "1.44", shim = "1.45"))]
    pub const UNICODE_VERSION: (u8, u8, u8) = (13, 0, 0);
    #[cfg(reexport = "1.45")]
    pub use core::char::UNICODE_VERSION;
}
#[doc(hidden)]
pub mod future {
    #[cfg(reexport = "1.48")]
    pub use core::future::{pending, ready, Pending, Ready};

    #[cfg(shim = "1.48")]
    pub use crate::free::v1_48::future::*;
}
#[doc(hidden)]
pub mod array {
    #[cfg(reexport = "1.53")]
    pub use core::array::{from_mut, from_ref};

    #[cfg(shim = "1.53")]
    pub use crate::free::v1_53::array::*;
}
#[doc(hidden)]
pub mod cmp {
    #[cfg(reexport = "1.53")]
    pub use core::cmp::{max_by, max_by_key, min_by, min_by_key};

    #[cfg(shim = "1.53")]
    pub use crate::free::v1_53::cmp::*;
}
