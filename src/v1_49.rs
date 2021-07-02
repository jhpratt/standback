mod sort;

use core::cmp::Ordering;

use easy_ext::ext;

use crate::traits::Sealed;

#[ext(Slice_v1_49)]
pub impl<T> [T]
where
    Self: Sealed<[T]>,
{
    fn select_nth_unstable(&mut self, index: usize) -> (&mut [T], &mut T, &mut [T])
    where
        T: Ord,
    {
        let mut f = |a: &T, b: &T| a.lt(b);
        sort::partition_at_index(self, index, &mut f)
    }

    fn select_nth_unstable_by<F>(
        &mut self,
        index: usize,
        mut compare: F,
    ) -> (&mut [T], &mut T, &mut [T])
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut f = |a: &T, b: &T| compare(a, b) == Ordering::Less;
        sort::partition_at_index(self, index, &mut f)
    }

    fn select_nth_unstable_by_key<K, F>(
        &mut self,
        index: usize,
        mut f: F,
    ) -> (&mut [T], &mut T, &mut [T])
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let mut g = |a: &T, b: &T| f(a).lt(&f(b));
        sort::partition_at_index(self, index, &mut g)
    }
}
