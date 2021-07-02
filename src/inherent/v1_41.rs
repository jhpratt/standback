use easy_ext::ext;

use crate::inherent::Sealed;

#[ext]
pub impl<T, E> Result<T, E>
where
    Self: Sealed<Result<T, E>>,
{
    fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        match self {
            Ok(t) => f(t),
            Err(_) => default,
        }
    }

    fn map_or_else<U, D: FnOnce(E) -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
        match self {
            Ok(t) => f(t),
            Err(e) => default(e),
        }
    }
}
