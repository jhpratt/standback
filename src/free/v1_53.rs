pub(crate) mod array {
    pub fn from_ref<T>(s: &T) -> &[T; 1] {
        unsafe { &*(s as *const T as *const [T; 1]) }
    }
    pub fn from_mut<T>(s: &mut T) -> &mut [T; 1] {
        unsafe { &mut *(s as *mut T as *mut [T; 1]) }
    }
}

pub(crate) mod cmp {
    use core::cmp::Ordering;

    #[must_use]
    pub fn min_by<T, F: FnOnce(&T, &T) -> Ordering>(v1: T, v2: T, compare: F) -> T {
        match compare(&v1, &v2) {
            Ordering::Less | Ordering::Equal => v1,
            Ordering::Greater => v2,
        }
    }
    #[must_use]
    pub fn min_by_key<T, F: FnMut(&T) -> K, K: Ord>(v1: T, v2: T, mut f: F) -> T {
        min_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))
    }
    #[must_use]
    pub fn max_by<T, F: FnOnce(&T, &T) -> Ordering>(v1: T, v2: T, compare: F) -> T {
        match compare(&v1, &v2) {
            Ordering::Less | Ordering::Equal => v2,
            Ordering::Greater => v1,
        }
    }
    #[must_use]
    pub fn max_by_key<T, F: FnMut(&T) -> K, K: Ord>(v1: T, v2: T, mut f: F) -> T {
        max_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))
    }
}
