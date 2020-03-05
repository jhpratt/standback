mod private {
    pub trait Sealed {}
    impl<T, E> Sealed for Result<T, E> {}
}

pub trait Result_v1_41<T, E>: private::Sealed {
    fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U;
    fn map_or_else<U, D: FnOnce(E) -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U;
}

impl<T, E> Result_v1_41<T, E> for Result<T, E> {
    #[inline]
    fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        match self {
            Ok(t) => f(t),
            Err(_) => default,
        }
    }

    #[inline]
    fn map_or_else<U, D: FnOnce(E) -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
        match self {
            Ok(t) => f(t),
            Err(e) => default(e),
        }
    }
}
