#[cfg(feature = "alloc")]
use alloc::collections::btree_map;
#[cfg(feature = "alloc")]
use alloc::collections::VecDeque;
#[cfg(feature = "alloc")] use core::cmp::Ordering;
#[cfg(feature = "alloc")]
use core::iter::FusedIterator;
#[cfg(feature = "std")]
use std::collections::hash_map;

#[cfg(feature = "alloc")] use easy_ext::ext;

#[cfg(feature = "alloc")]
use crate::inherent::Sealed;

#[cfg(feature = "alloc")]
#[ext]
pub impl<K, V> btree_map::BTreeMap<K, V>
where Self: Sealed<btree_map::BTreeMap<K, V>>
{
    fn into_keys(self) -> BTreeMapIntoKeys<K, V> {
        BTreeMapIntoKeys { inner: self.into_iter() }
    }
    fn into_values(self) -> BTreeMapIntoValues<K, V> {
        BTreeMapIntoValues { inner: self.into_iter() }
    }
}

#[cfg(feature = "alloc")]
#[derive(Debug)]
pub struct BTreeMapIntoKeys<K, V> {
    inner: btree_map::IntoIter<K, V>,
}

#[cfg(feature = "alloc")]
#[derive(Debug)]
pub struct BTreeMapIntoValues<K, V> {
    inner: btree_map::IntoIter<K, V>,
}

#[cfg(feature = "alloc")]
impl<K, V> Iterator for BTreeMapIntoKeys<K, V> {
    type Item = K;

    fn next(&mut self) -> Option<K> {
        self.inner.next().map(|(k, _)| k)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    fn last(mut self) -> Option<K> {
        self.next_back()
    }
    fn min(mut self) -> Option<K> {
        self.next()
    }
    fn max(mut self) -> Option<K> {
        self.next_back()
    }
}

#[cfg(feature = "alloc")]
impl<K, V> DoubleEndedIterator for BTreeMapIntoKeys<K, V> {
    fn next_back(&mut self) -> Option<K> {
        self.inner.next_back().map(|(k, _)| k)
    }
}

#[cfg(feature = "alloc")]
impl<K, V> ExactSizeIterator for BTreeMapIntoKeys<K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(feature = "alloc")]
impl<K, V> FusedIterator for BTreeMapIntoKeys<K, V> {}

#[cfg(feature = "alloc")]
impl<K, V> Iterator for BTreeMapIntoValues<K, V> {
    type Item = V;

    fn next(&mut self) -> Option<V> {
        self.inner.next().map(|(_, v)| v)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    fn last(mut self) -> Option<V> {
        self.next_back()
    }
}

#[cfg(feature = "alloc")]
impl<K, V> DoubleEndedIterator for BTreeMapIntoValues<K, V> {
    fn next_back(&mut self) -> Option<V> {
        self.inner.next_back().map(|(_, v)| v)
    }
}

#[cfg(feature = "alloc")]
impl<K, V> ExactSizeIterator for BTreeMapIntoValues<K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(feature = "alloc")]
impl<K, V> FusedIterator for BTreeMapIntoValues<K, V> {}

#[cfg(feature = "std")]
#[ext]
pub impl<K, V> hash_map::HashMap<K, V>
where Self: Sealed<hash_map::HashMap<K, V>>
{
    fn into_keys(self) -> HashMapIntoKeys<K, V> {
        HashMapIntoKeys { inner: self.into_iter() }
    }
    fn into_values(self) -> HashMapIntoValues<K, V> {
        HashMapIntoValues { inner: self.into_iter() }
    }
}

#[cfg(feature = "std")]
#[derive(Debug)]
pub struct HashMapIntoKeys<K, V> {
    inner: hash_map::IntoIter<K, V>,
}

#[cfg(feature = "std")]
#[derive(Debug)]
pub struct HashMapIntoValues<K, V> {
    inner: hash_map::IntoIter<K, V>,
}

#[cfg(feature = "std")]
impl<K, V> Iterator for HashMapIntoKeys<K, V> {
    type Item = K;

    fn next(&mut self) -> Option<K> {
        self.inner.next().map(|(k, _)| k)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
#[cfg(feature = "std")]
impl<K, V> ExactSizeIterator for HashMapIntoKeys<K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}
#[cfg(feature = "std")]
impl<K, V> FusedIterator for HashMapIntoKeys<K, V> {}
#[cfg(feature = "std")]
impl<K, V> Iterator for HashMapIntoValues<K, V> {
    type Item = V;

    fn next(&mut self) -> Option<V> {
        self.inner.next().map(|(_, v)| v)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
#[cfg(feature = "std")]
impl<K, V> ExactSizeIterator for HashMapIntoValues<K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}
#[cfg(feature = "std")]
impl<K, V> FusedIterator for HashMapIntoValues<K, V> {}

#[cfg(feature = "alloc")]
#[ext]
pub impl<T> VecDeque<T>
where Self: Sealed<VecDeque<T>>
{
    fn binary_search(&self, x: &T) -> Result<usize, usize>
    where T: Ord {
        self.binary_search_by(|e| e.cmp(x))
    }
    fn binary_search_by<'a, F: FnMut(&'a T) -> Ordering>(
        &'a self,
        mut f: F,
    ) -> Result<usize, usize>
    where
        T: 'a,
    {
        let (front, back) = self.as_slices();
        let cmp_back = back.first().map(|elem| f(elem));

        if let Some(Ordering::Equal) = cmp_back {
            Ok(front.len())
        } else if let Some(Ordering::Less) = cmp_back {
            back.binary_search_by(f).map(|idx| idx + front.len()).map_err(|idx| idx + front.len())
        } else {
            front.binary_search_by(f)
        }
    }
    fn binary_search_by_key<'a, B: Ord, F: FnMut(&'a T) -> B>(
        &'a self,
        b: &B,
        mut f: F,
    ) -> Result<usize, usize>
    where
        T: 'a,
    {
        self.binary_search_by(|k| f(k).cmp(b))
    }
    fn partition_point<P: FnMut(&T) -> bool>(&self, mut pred: P) -> usize {
        let (front, back) = self.as_slices();

        if let Some(true) = back.first().map(|v| pred(v)) {
            partition_point(back, pred) + front.len()
        } else {
            partition_point(front, pred)
        }
    }
}

#[cfg(feature = "alloc")]
fn partition_point<T, P: FnMut(&T) -> bool>(this: &[T], mut pred: P) -> usize {
    let mut left = 0;
    let mut right = this.len();

    while left != right {
        let mid = left + (right - left) / 2;
        let value = unsafe { this.get_unchecked(mid) };
        if pred(value) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}
