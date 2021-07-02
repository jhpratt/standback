pub(crate) mod mem {
    pub fn take<T: Default>(dest: &mut T) -> T {
        core::mem::replace(dest, T::default())
    }
}
