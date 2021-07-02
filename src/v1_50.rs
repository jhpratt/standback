#[cfg(feature = "alloc")]
use alloc::collections::btree_map;
use core::cell::{RefCell, UnsafeCell};
#[cfg(feature = "std")]
use std::collections::hash_map;

use easy_ext::ext;

use crate::traits::Sealed;

#[ext(Bool_v1_50)]
pub impl bool
where
    Self: Sealed<bool>,
{
    fn then<T, F: FnOnce() -> T>(self, f: F) -> Option<T> {
        if self { Some(f()) } else { None }
    }
}

#[cfg(feature = "alloc")]
#[ext(BTreeMapEntry_v1_50)]
pub impl<'a, K: Ord + 'a, V: 'a> btree_map::Entry<'a, K, V>
where
    Self: Sealed<btree_map::Entry<'a, K, V>>,
{
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
#[ext(HashMapEntry_v1_50)]
pub impl<'a, K: 'a, V: 'a> hash_map::Entry<'a, K, V>
where
    Self: Sealed<hash_map::Entry<'a, K, V>>,
{
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

#[ext(f32_v1_50)]
pub impl f32
where
    Self: Sealed<f32>,
{
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

#[ext(f64_v1_50)]
pub impl f64
where
    Self: Sealed<f64>,
{
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

#[ext(Ord_v1_50)]
pub impl<T: Ord> T
where
    Self: Sealed<T>,
{
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

#[ext(RefCell_v1_50)]
pub impl<T: Default> RefCell<T>
where
    Self: Sealed<RefCell<T>>,
{
    fn take(&self) -> T {
        self.replace(Default::default())
    }
}

#[ext(Slice_v1_50)]
pub impl<T> [T]
where
    Self: Sealed<[T]>,
{
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

#[ext(UnsafeCell_v1_50)]
pub impl<T> UnsafeCell<T>
where
    Self: Sealed<UnsafeCell<T>>,
{
    fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.get() }
    }
}
