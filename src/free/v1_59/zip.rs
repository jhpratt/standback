use core::cmp;
use core::fmt::{self, Debug};
use core::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator, Iterator};

pub fn zip<A: IntoIterator, B: IntoIterator>(a: A, b: B) -> Zip<A::IntoIter, B::IntoIter> {
    Zip { a: a.into_iter(), b: b.into_iter() }
}

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Zip<A, B> {
    a: A,
    b: B,
}

impl<A: Debug, B: Debug> Debug for Zip<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Zip").finish()
    }
}

impl<A: Iterator, B: Iterator> Iterator for Zip<A, B> {
    type Item = (A::Item, B::Item);

    fn next(&mut self) -> Option<(A::Item, B::Item)> {
        let x = self.a.next()?;
        let y = self.b.next()?;
        Some((x, y))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (a_lower, a_upper) = self.a.size_hint();
        let (b_lower, b_upper) = self.b.size_hint();

        let lower = cmp::min(a_lower, b_lower);

        let upper = match (a_upper, b_upper) {
            (Some(x), Some(y)) => Some(cmp::min(x, y)),
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None,
        };

        (lower, upper)
    }
}

impl<A: DoubleEndedIterator + ExactSizeIterator, B: DoubleEndedIterator + ExactSizeIterator>
    DoubleEndedIterator for Zip<A, B>
{
    fn next_back(&mut self) -> Option<(A::Item, B::Item)> {
        let a_sz = self.a.len();
        let b_sz = self.b.len();
        if a_sz != b_sz {
            if a_sz > b_sz {
                for _ in 0..a_sz - b_sz {
                    self.a.next_back();
                }
            } else {
                for _ in 0..b_sz - a_sz {
                    self.b.next_back();
                }
            }
        }
        match (self.a.next_back(), self.b.next_back()) {
            (Some(x), Some(y)) => Some((x, y)),
            (None, None) => None,
            _ => unreachable!(),
        }
    }
}

impl<A: ExactSizeIterator, B: ExactSizeIterator> ExactSizeIterator for Zip<A, B> {}
impl<A: FusedIterator, B: FusedIterator> FusedIterator for Zip<A, B> {}
