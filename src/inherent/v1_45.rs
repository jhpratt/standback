use easy_ext::ext;

use crate::inherent::Sealed;
use crate::pattern::{Pattern, ReverseSearcher};

macro_rules! impl_int_v1_45 {
    ($($type:ident)+) => {$(
        #[ext]
        pub impl $type where Self: Sealed<$type>, {
            fn saturating_neg(self) -> Self {
                if self == core::$type::MIN {
                    core::$type::MAX
                } else {
                    -self
                }
            }

            fn saturating_abs(self) -> Self {
                if self.is_negative() {
                    self.saturating_neg()
                } else {
                    self
                }
            }
        }
    )*};
}

impl_int_v1_45![i8 i16 i32 i64 i128 isize];

#[ext]
pub impl str
where
    Self: Sealed<str>,
{
    #[must_use = "this returns the remaining substring as a new slice, without modifying the \
                  original"]
    fn strip_prefix<'a, P: Pattern<'a>>(&'a self, prefix: P) -> Option<&'a str> {
        prefix.strip_prefix_of(self)
    }

    #[must_use = "this returns the remaining substring as a new slice, without modifying the \
                  original"]
    fn strip_suffix<'a, P>(&'a self, suffix: P) -> Option<&'a str>
    where
        P: Pattern<'a>,
        <P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,
    {
        suffix.strip_suffix_of(self)
    }
}
