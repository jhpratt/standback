use core::iter::FusedIterator;

pub mod f32 {
    pub const LOG10_2: f32 = 0.301029995663981195213738894724493027_f32;
    pub const LOG2_10: f32 = 3.32192809488736234787031942948939018_f32;
}

pub mod f64 {
    pub const LOG10_2: f64 = 0.301029995663981195213738894724493027_f64;
    pub const LOG2_10: f64 = 3.32192809488736234787031942948939018_f64;
}

#[inline]
pub fn once_with<A, F: FnOnce() -> A>(gen: F) -> OnceWith<F> {
    OnceWith { gen: Some(gen) }
}

#[derive(Copy, Clone, Debug)]
pub struct OnceWith<F> {
    gen: Option<F>,
}

impl<A, F: FnOnce() -> A> Iterator for OnceWith<F> {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<A> {
        let f = self.gen.take()?;
        Some(f())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.gen.iter().size_hint()
    }
}

impl<A, F: FnOnce() -> A> DoubleEndedIterator for OnceWith<F> {
    fn next_back(&mut self) -> Option<A> {
        self.next()
    }
}

impl<A, F: FnOnce() -> A> ExactSizeIterator for OnceWith<F> {
    fn len(&self) -> usize {
        self.gen.iter().len()
    }
}

impl<A, F: FnOnce() -> A> FusedIterator for OnceWith<F> {}
