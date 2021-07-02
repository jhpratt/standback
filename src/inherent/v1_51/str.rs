use core::fmt;

use crate::pattern::Pattern;

pub struct SplitInclusive<'a, P: Pattern<'a>>(pub(super) SplitInternal<'a, P>);

pub(super) struct SplitInternal<'a, P: Pattern<'a>> {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) matcher: P::Searcher,
    pub(super) allow_trailing_empty: bool,
    pub(super) finished: bool,
}

impl<'a, P> fmt::Debug for SplitInternal<'a, P>
where
    P: Pattern<'a>,
    P::Searcher: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SplitInternal")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("matcher", &self.matcher)
            .field("allow_trailing_empty", &self.allow_trailing_empty)
            .field("finished", &self.finished)
            .finish()
    }
}
