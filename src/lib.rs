#![cfg_attr(not(feature = "std"), no_std)]
#![deny(rust_2018_idioms, unused_qualifications)]
#![allow(
    non_camel_case_types,
    unstable_name_collisions,
    clippy::missing_safety_doc
)]

/*!
Standback backports a number of methods, structs, and macros that have been stabilized in the Rust
standard library since 1.36.0. This allows crate authors to depend on Standback rather than forcing
downstream users to upgrade their compiler (or not use the new feature at all).

Due to a variety of restrictions in the Rust, it is not possible to implement everything that has
been stabilized.

# Usage

For most cases, importing the prelude should suffice.

```rust,no_run
use standback::prelude::*;
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

An allocator is not required for any backported item. If any require an allocator in the future, it
will be gated under an `alloc` feature.

# Inherent and trait methods, associated constants

The following methods and constants are available via the prelude. For brevity, `i*` is `i8`, `i16`,
`i32`, `i64`, `i128`, and `isize`; `u*` is `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`.

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
task::Wake // requires rustc 1.33
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

# Free functions and constants

```text
future::pending // 1.48
future::ready // 1.48
char::UNICODE_VERSION // 1.45
{f32, f64}::consts::LOG10_2 // 1.43
{f32, f64}::consts::LOG2_10 // 1.43
iter::once_with // 1.43
mem::take // 1.40
```

# Macros

```text
matches! // 1.42
todo! // 1.39
```
*/

// A few traits to make sealing other traits simpler.
mod traits {
    pub trait Sealed<T: ?Sized> {}
    impl<T: ?Sized> Sealed<T> for T {}

    macro_rules! impl_trait_for_all {
        ($trait:ident => $($type:ty)+) => {$(
            impl $trait for $type {}
        )+};
    }

    pub trait Integer: Sized {}
    impl_trait_for_all!(Integer => i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

    pub trait SignedInteger {}
    impl_trait_for_all!(SignedInteger => i8 i16 i32 i64 i128 isize);

    pub trait UnsignedInteger {}
    impl_trait_for_all!(UnsignedInteger => u8 u16 u32 u64 u128 usize);

    pub trait Float {}
    impl_trait_for_all!(Float => f32 f64);
}

#[cfg(__standback_before_1_37)]
mod v1_37;
#[cfg(__standback_before_1_38)]
mod v1_38;
#[cfg(__standback_before_1_40)]
mod v1_40;
#[cfg(__standback_before_1_41)]
mod v1_41;
#[cfg(__standback_before_1_42)]
mod v1_42;
#[cfg(__standback_before_1_43)]
mod v1_43;
#[cfg(__standback_before_1_44)]
mod v1_44;
#[cfg(__standback_before_1_45)]
mod v1_45;
#[cfg(__standback_before_1_46)]
mod v1_46;
#[cfg(__standback_before_1_47)]
mod v1_47;
#[cfg(__standback_before_1_48)]
mod v1_48;
#[cfg(__standback_before_1_49)]
mod v1_49;
#[cfg(__standback_before_1_50)]
mod v1_50;
#[cfg(__standback_before_1_51)]
mod v1_51;

#[doc(hidden)]
pub mod prelude {
    #[cfg(__standback_before_1_39)]
    pub use core::unimplemented as todo;

    #[cfg(__standback_before_1_42)]
    pub use crate::matches;
    #[cfg(__standback_before_1_37)]
    pub use crate::v1_37::{
        Cell_v1_37, Cell_v1_37_, DoubleEndedIterator_v1_37, Option_v1_37, Slice_v1_37,
    };
    #[cfg(__standback_before_1_38)]
    pub use crate::v1_38::{
        ConstPtr_v1_38, Duration_v1_38, EuclidFloat_v1_38, Euclid_v1_38, MutPtr_v1_38,
    };
    #[cfg(all(feature = "std", __standback_before_1_40))]
    pub use crate::v1_40::slice_v1_40;
    #[cfg(__standback_before_1_40)]
    pub use crate::v1_40::{f32_v1_40, f64_v1_40, Option_v1_40, Option_v1_40_};
    #[cfg(__standback_before_1_41)]
    pub use crate::v1_41::Result_v1_41;
    #[cfg(all(__standback_before_1_42, feature = "std"))]
    pub use crate::v1_42::Condvar_v1_42;
    #[cfg(__standback_before_1_42)]
    pub use crate::v1_42::ManuallyDrop_v1_42;
    #[cfg(__standback_before_1_43)]
    pub use crate::v1_43::{float_v1_43, int_v1_43};
    #[cfg(__standback_before_1_44)]
    pub use crate::v1_44::Layout_v1_44;
    #[cfg(all(__standback_before_1_44, feature = "std"))]
    pub use crate::v1_44::PathBuf_v1_44;
    #[cfg(__standback_before_1_45)]
    pub use crate::v1_45::int_v1_45;
    #[cfg(__standback_before_1_46)]
    pub use crate::v1_46::{int_v1_46, Option_v1_46};
    #[cfg(all(feature = "std", __standback_before_1_47))]
    pub use crate::v1_47::Vec_v1_47;
    #[cfg(__standback_before_1_47)]
    pub use crate::v1_47::{Range_v1_47, Result_v1_47};
    #[cfg(__standback_before_1_48)]
    pub use crate::v1_48::Slice_v1_48;
    #[cfg(__standback_before_1_49)]
    pub use crate::v1_49::Slice_v1_49;
    #[cfg(all(__standback_before_1_50, feature = "std"))]
    pub use crate::v1_50::{BTreeMapEntry_v1_50, HashMapEntry_v1_50};
    #[cfg(__standback_before_1_50)]
    pub use crate::v1_50::{
        Bool_v1_50, Float_v1_50, Ord_v1_50, RefCell_v1_50, Slice_v1_50, UnsafeCell_v1_50,
    };
    #[cfg(all(__standback_before_1_51, feature = "std"))]
    pub use crate::v1_51::{Arc_v1_51, Seek_v1_51};
    #[cfg(__standback_before_1_51)]
    pub use crate::v1_51::{Integer_v1_51, Peekable_v1_51, Poll_v1_51, Slice_v1_51};
}
#[doc(hidden)]
pub mod mem {
    #[cfg(__standback_since_1_40)]
    pub use core::mem::take;

    #[cfg(__standback_before_1_40)]
    pub use crate::v1_40::take;
}
#[doc(hidden)]
pub mod iter {
    #[cfg(__standback_since_1_43)]
    pub use core::iter::{once_with, OnceWith};

    #[cfg(__standback_before_1_43)]
    pub use crate::v1_43::{once_with, OnceWith};
}
#[doc(hidden)]
pub mod task {
    #[cfg(all(__standback_since_1_51, feature = "std"))]
    pub use std::task::Wake;

    #[cfg(all(__standback_before_1_51, feature = "std"))]
    pub use crate::v1_51::Wake;
}
#[doc(hidden)]
pub mod f32 {
    pub mod consts {
        #[cfg(__standback_since_1_47)]
        pub use core::f32::consts::TAU;
        #[cfg(__standback_since_1_43)]
        pub use core::f32::consts::{LOG10_2, LOG2_10};

        #[cfg(__standback_before_1_43)]
        pub use crate::v1_43::f32::{LOG10_2, LOG2_10};
        #[cfg(__standback_before_1_47)]
        pub use crate::v1_47::f32::TAU;
    }
}
#[doc(hidden)]
pub mod f64 {
    pub mod consts {
        #[cfg(__standback_since_1_47)]
        pub use core::f64::consts::TAU;
        #[cfg(__standback_since_1_43)]
        pub use core::f64::consts::{LOG10_2, LOG2_10};

        #[cfg(__standback_before_1_43)]
        pub use crate::v1_43::f64::{LOG10_2, LOG2_10};
        #[cfg(__standback_before_1_47)]
        pub use crate::v1_47::f64::TAU;
    }
}
#[doc(hidden)]
pub mod char {
    #[cfg(__standback_before_1_38)]
    pub const UNICODE_VERSION: (u8, u8, u8) = (11, 0, 0);
    #[cfg(all(__standback_since_1_38, __standback_before_1_44))]
    pub const UNICODE_VERSION: (u8, u8, u8) = (12, 1, 0);
    #[cfg(all(__standback_since_1_44, __standback_before_1_45))]
    pub const UNICODE_VERSION: (u8, u8, u8) = (13, 0, 0);
    #[cfg(__standback_since_1_45)]
    pub use core::char::UNICODE_VERSION;
}
#[doc(hidden)]
pub mod future {
    #[cfg(__standback_since_1_48)]
    pub use core::future::{pending, ready, Pending, Ready};

    #[cfg(__standback_before_1_48)]
    pub use crate::v1_48::future::{pending, ready, Pending, Ready};
}
