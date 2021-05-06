#[cfg(feature = "alloc")]
use alloc::collections::btree_map;
use core::cell::{RefCell, UnsafeCell};
#[cfg(feature = "std")]
use std::collections::hash_map;

use crate::traits::{Float, Sealed};

pub trait Bool_v1_50: Sealed<bool> {
    fn then<T, F: FnOnce() -> T>(self, f: F) -> Option<T>;
}

impl Bool_v1_50 for bool {
    fn then<T, F: FnOnce() -> T>(self, f: F) -> Option<T> {
        if self { Some(f()) } else { None }
    }
}

#[cfg(feature = "alloc")]
pub trait BTreeMapEntry_v1_50<'a, K: 'a, V: 'a>: Sealed<btree_map::Entry<'a, K, V>> {
    fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V;
}

#[cfg(feature = "alloc")]
impl<'a, K: Ord + 'a, V: 'a> BTreeMapEntry_v1_50<'a, K, V> for btree_map::Entry<'a, K, V> {
    fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V {
        match self {
            btree_map::Entry::Occupied(entry) => entry.into_mut(),
            btree_map::Entry::Vacant(entry) => {
                let value = default(entry.key());
                entry.insert(value)
            }
        }
    }
}

#[cfg(feature = "std")]
pub trait HashMapEntry_v1_50<'a, K: 'a, V: 'a>: Sealed<hash_map::Entry<'a, K, V>> {
    fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V;
}

#[cfg(feature = "std")]
impl<'a, K: 'a, V: 'a> HashMapEntry_v1_50<'a, K, V> for hash_map::Entry<'a, K, V> {
    fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V {
        match self {
            hash_map::Entry::Occupied(entry) => entry.into_mut(),
            hash_map::Entry::Vacant(entry) => {
                let value = default(entry.key());
                entry.insert(value)
            }
        }
    }
}

pub trait Float_v1_50: Float {
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl Float_v1_50 for f32 {
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn clamp(self, min: f32, max: f32) -> f32 {
        assert!(min <= max);
        let mut x = self;
        if x < min {
            x = min;
        }
        if x > max {
            x = max;
        }
        x
    }
}

impl Float_v1_50 for f64 {
    #[must_use = "method returns a new number and does not mutate the original value"]
    fn clamp(self, min: f64, max: f64) -> f64 {
        assert!(min <= max);
        let mut x = self;
        if x < min {
            x = min;
        }
        if x > max {
            x = max;
        }
        x
    }
}

pub trait Ord_v1_50<T: Ord>: Sealed<T> {
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl<T: Ord> Ord_v1_50<T> for T {
    #[must_use]
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

pub trait RefCell_v1_50<T>: Sealed<RefCell<T>> {
    fn take(&self) -> T;
}

impl<T: Default> RefCell_v1_50<T> for RefCell<T> {
    fn take(&self) -> T {
        self.replace(Default::default())
    }
}

pub trait Slice_v1_50<T>: Sealed<[T]> {
    fn fill(&mut self, value: T)
    where
        T: Clone;
}

impl<T> Slice_v1_50<T> for [T] {
    fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        if let Some((last, elems)) = self.split_last_mut() {
            for el in elems {
                el.clone_from(&value);
            }

            *last = value
        }
    }
}

pub trait UnsafeCell_v1_50<T>: Sealed<UnsafeCell<T>> {
    fn get_mut(&mut self) -> &mut T;
}

impl<T> UnsafeCell_v1_50<T> for UnsafeCell<T> {
    fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.get() }
    }
}
