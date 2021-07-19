mod memchr;

#[allow(unused_imports)]
#[cfg(feature = "alloc")]
use alloc::string::String;
use core::{cmp, fmt, usize};

mod sealed {
    use super::*;

    #[allow(unreachable_pub)]
    pub trait Sealed {}
    impl Sealed for &str {}
    impl Sealed for char {}
    impl Sealed for &[char] {}
    impl<F: FnMut(char) -> bool> Sealed for F {}
    impl Sealed for &&str {}
    #[cfg(feature = "alloc")]
    impl Sealed for &String {}

    impl<C: FnMut(char) + MultiCharEq> Sealed for MultiCharEqSearcher<'_, C> {}
    impl<C: MultiCharEq> Sealed for MultiCharEqPattern<C> {}
}

#[allow(unreachable_pub)]
pub trait Pattern<'a>: Sized + sealed::Sealed {
    type Searcher: Searcher<'a>;

    fn into_searcher(self, haystack: &'a str) -> Self::Searcher;

    fn is_contained_in(self, haystack: &'a str) -> bool {
        self.into_searcher(haystack).next_match().is_some()
    }

    fn is_prefix_of(self, haystack: &'a str) -> bool {
        match self.into_searcher(haystack).next() {
            SearchStep::Match(0, _) => true,
            _ => false,
        }
    }

    fn is_suffix_of(self, haystack: &'a str) -> bool
    where Self::Searcher: ReverseSearcher<'a> {
        match self.into_searcher(haystack).next_back() {
            SearchStep::Match(_, j) if haystack.len() == j => true,
            _ => false,
        }
    }

    fn strip_prefix_of(self, haystack: &'a str) -> Option<&'a str> {
        if let SearchStep::Match(start, len) = self.into_searcher(haystack).next() {
            debug_assert_eq!(
                start, 0,
                "The first search step from Searcher must include the first character"
            );
            unsafe { Some(haystack.get_unchecked(len..)) }
        } else {
            None
        }
    }

    fn strip_suffix_of(self, haystack: &'a str) -> Option<&'a str>
    where Self::Searcher: ReverseSearcher<'a> {
        if let SearchStep::Match(start, end) = self.into_searcher(haystack).next_back() {
            debug_assert_eq!(
                end,
                haystack.len(),
                "The first search step from ReverseSearcher must include the last character"
            );
            unsafe { Some(haystack.get_unchecked(..start)) }
        } else {
            None
        }
    }
}

#[allow(unreachable_pub)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SearchStep {
    Match(usize, usize),
    Reject(usize, usize),
    Done,
}

#[allow(unreachable_pub)]
pub unsafe trait Searcher<'a> {
    fn haystack(&self) -> &'a str;
    fn next(&mut self) -> SearchStep;

    fn next_match(&mut self) -> Option<(usize, usize)> {
        loop {
            match self.next() {
                SearchStep::Match(a, b) => return Some((a, b)),
                SearchStep::Done => return None,
                _ => continue,
            }
        }
    }

    fn next_reject(&mut self) -> Option<(usize, usize)> {
        loop {
            match self.next() {
                SearchStep::Reject(a, b) => return Some((a, b)),
                SearchStep::Done => return None,
                _ => continue,
            }
        }
    }
}

#[allow(unreachable_pub)]
pub unsafe trait ReverseSearcher<'a>: Searcher<'a> {
    fn next_back(&mut self) -> SearchStep;

    fn next_match_back(&mut self) -> Option<(usize, usize)> {
        loop {
            match self.next_back() {
                SearchStep::Match(a, b) => return Some((a, b)),
                SearchStep::Done => return None,
                _ => continue,
            }
        }
    }

    fn next_reject_back(&mut self) -> Option<(usize, usize)> {
        loop {
            match self.next_back() {
                SearchStep::Reject(a, b) => return Some((a, b)),
                SearchStep::Done => return None,
                _ => continue,
            }
        }
    }
}

#[allow(unreachable_pub)]
#[derive(Clone, Debug)]
pub struct CharSearcher<'a> {
    haystack: &'a str,
    finger: usize,
    finger_back: usize,
    needle: char,
    utf8_size: usize,
    utf8_encoded: [u8; 4],
}

unsafe impl<'a> Searcher<'a> for CharSearcher<'a> {
    fn haystack(&self) -> &'a str {
        self.haystack
    }
    fn next(&mut self) -> SearchStep {
        let old_finger = self.finger;
        let slice = unsafe { self.haystack.get_unchecked(old_finger..self.finger_back) };
        let mut iter = slice.chars();
        let old_len = iter.as_str().len();
        if let Some(ch) = iter.next() {
            self.finger += old_len - iter.as_str().len();
            if ch == self.needle {
                SearchStep::Match(old_finger, self.finger)
            } else {
                SearchStep::Reject(old_finger, self.finger)
            }
        } else {
            SearchStep::Done
        }
    }
    fn next_match(&mut self) -> Option<(usize, usize)> {
        loop {
            let bytes = self.haystack.as_bytes().get(self.finger..self.finger_back)?;
            let last_byte = unsafe { *self.utf8_encoded.get_unchecked(self.utf8_size - 1) };
            if let Some(index) = memchr::memchr(last_byte, bytes) {
                self.finger += index + 1;
                if self.finger >= self.utf8_size {
                    let found_char = self.finger - self.utf8_size;
                    if let Some(slice) = self.haystack.as_bytes().get(found_char..self.finger) {
                        if slice == &self.utf8_encoded[0..self.utf8_size] {
                            return Some((found_char, self.finger));
                        }
                    }
                }
            } else {
                self.finger = self.finger_back;
                return None;
            }
        }
    }
}

unsafe impl<'a> ReverseSearcher<'a> for CharSearcher<'a> {
    fn next_back(&mut self) -> SearchStep {
        let old_finger = self.finger_back;
        let slice = unsafe { self.haystack.get_unchecked(self.finger..old_finger) };
        let mut iter = slice.chars();
        let old_len = iter.as_str().len();
        if let Some(ch) = iter.next_back() {
            self.finger_back -= old_len - iter.as_str().len();
            if ch == self.needle {
                SearchStep::Match(self.finger_back, old_finger)
            } else {
                SearchStep::Reject(self.finger_back, old_finger)
            }
        } else {
            SearchStep::Done
        }
    }
    fn next_match_back(&mut self) -> Option<(usize, usize)> {
        let haystack = self.haystack.as_bytes();
        loop {
            let bytes = haystack.get(self.finger..self.finger_back)?;
            let last_byte = unsafe { *self.utf8_encoded.get_unchecked(self.utf8_size - 1) };
            if let Some(index) = memchr::memrchr(last_byte, bytes) {
                let index = self.finger + index;
                let shift = self.utf8_size - 1;
                if index >= shift {
                    let found_char = index - shift;
                    if let Some(slice) = haystack.get(found_char..(found_char + self.utf8_size)) {
                        if slice == &self.utf8_encoded[0..self.utf8_size] {
                            self.finger_back = found_char;
                            return Some((self.finger_back, self.finger_back + self.utf8_size));
                        }
                    }
                }
                self.finger_back = index;
            } else {
                self.finger_back = self.finger;
                return None;
            }
        }
    }
}

impl<'a> Pattern<'a> for char {
    type Searcher = CharSearcher<'a>;

    fn into_searcher(self, haystack: &'a str) -> Self::Searcher {
        let mut utf8_encoded = [0; 4];
        let utf8_size = self.encode_utf8(&mut utf8_encoded).len();
        CharSearcher {
            haystack,
            finger: 0,
            finger_back: haystack.len(),
            needle: self,
            utf8_size,
            utf8_encoded,
        }
    }

    fn is_contained_in(self, haystack: &'a str) -> bool {
        if (self as u32) < 128 {
            haystack.as_bytes().contains(&(self as u8))
        } else {
            let mut buffer = [0u8; 4];
            self.encode_utf8(&mut buffer).is_contained_in(haystack)
        }
    }

    fn is_prefix_of(self, haystack: &'a str) -> bool {
        self.encode_utf8(&mut [0u8; 4]).is_prefix_of(haystack)
    }

    fn strip_prefix_of(self, haystack: &'a str) -> Option<&'a str> {
        self.encode_utf8(&mut [0u8; 4]).strip_prefix_of(haystack)
    }

    fn is_suffix_of(self, haystack: &'a str) -> bool
    where Self::Searcher: ReverseSearcher<'a> {
        self.encode_utf8(&mut [0u8; 4]).is_suffix_of(haystack)
    }

    fn strip_suffix_of(self, haystack: &'a str) -> Option<&'a str>
    where Self::Searcher: ReverseSearcher<'a> {
        self.encode_utf8(&mut [0u8; 4]).strip_suffix_of(haystack)
    }
}

#[doc(hidden)]
trait MultiCharEq {
    fn matches(&mut self, c: char) -> bool;
}

impl<F> MultiCharEq for F
where F: FnMut(char) -> bool
{
    fn matches(&mut self, c: char) -> bool {
        (*self)(c)
    }
}

impl MultiCharEq for &[char] {
    fn matches(&mut self, c: char) -> bool {
        self.iter().any(|&m| m == c)
    }
}

struct MultiCharEqPattern<C: MultiCharEq>(C);

#[derive(Clone, Debug)]
struct MultiCharEqSearcher<'a, C: MultiCharEq> {
    char_eq: C,
    haystack: &'a str,
    char_indices: core::str::CharIndices<'a>,
}

impl<'a, C: MultiCharEq> Pattern<'a> for MultiCharEqPattern<C> {
    type Searcher = MultiCharEqSearcher<'a, C>;

    fn into_searcher(self, haystack: &'a str) -> MultiCharEqSearcher<'a, C> {
        MultiCharEqSearcher { haystack, char_eq: self.0, char_indices: haystack.char_indices() }
    }
}

unsafe impl<'a, C: MultiCharEq> Searcher<'a> for MultiCharEqSearcher<'a, C> {
    fn haystack(&self) -> &'a str {
        self.haystack
    }

    fn next(&mut self) -> SearchStep {
        let s = &mut self.char_indices;
        let pre_len = s.as_str().len();
        if let Some((i, c)) = s.next() {
            let len = s.as_str().len();
            let char_len = pre_len - len;
            if self.char_eq.matches(c) {
                return SearchStep::Match(i, i + char_len);
            } else {
                return SearchStep::Reject(i, i + char_len);
            }
        }
        SearchStep::Done
    }
}

unsafe impl<'a, C: MultiCharEq> ReverseSearcher<'a> for MultiCharEqSearcher<'a, C> {
    fn next_back(&mut self) -> SearchStep {
        let s = &mut self.char_indices;
        let pre_len = s.as_str().len();
        if let Some((i, c)) = s.next_back() {
            let len = s.as_str().len();
            let char_len = pre_len - len;
            if self.char_eq.matches(c) {
                return SearchStep::Match(i, i + char_len);
            } else {
                return SearchStep::Reject(i, i + char_len);
            }
        }
        SearchStep::Done
    }
}

macro_rules! pattern_methods {
    ($t:ty, $pmap:expr, $smap:expr) => {
        type Searcher = $t;

        #[allow(clippy::redundant_closure_call)]
        fn into_searcher(self, haystack: &'a str) -> $t {
            ($smap)(($pmap)(self).into_searcher(haystack))
        }

        #[allow(clippy::redundant_closure_call)]
        fn is_contained_in(self, haystack: &'a str) -> bool {
            ($pmap)(self).is_contained_in(haystack)
        }

        #[allow(clippy::redundant_closure_call)]
        fn is_prefix_of(self, haystack: &'a str) -> bool {
            ($pmap)(self).is_prefix_of(haystack)
        }

        #[allow(clippy::redundant_closure_call)]
        fn strip_prefix_of(self, haystack: &'a str) -> Option<&'a str> {
            ($pmap)(self).strip_prefix_of(haystack)
        }

        #[allow(clippy::redundant_closure_call)]
        fn is_suffix_of(self, haystack: &'a str) -> bool
        where $t: ReverseSearcher<'a> {
            ($pmap)(self).is_suffix_of(haystack)
        }

        #[allow(clippy::redundant_closure_call)]
        fn strip_suffix_of(self, haystack: &'a str) -> Option<&'a str>
        where $t: ReverseSearcher<'a> {
            ($pmap)(self).strip_suffix_of(haystack)
        }
    };
}

macro_rules! searcher_methods {
    (forward) => {
        fn haystack(&self) -> &'a str {
            self.0.haystack()
        }
        fn next(&mut self) -> SearchStep {
            self.0.next()
        }
        fn next_match(&mut self) -> Option<(usize, usize)> {
            self.0.next_match()
        }
        fn next_reject(&mut self) -> Option<(usize, usize)> {
            self.0.next_reject()
        }
    };
    (reverse) => {
        fn next_back(&mut self) -> SearchStep {
            self.0.next_back()
        }
        fn next_match_back(&mut self) -> Option<(usize, usize)> {
            self.0.next_match_back()
        }
        fn next_reject_back(&mut self) -> Option<(usize, usize)> {
            self.0.next_reject_back()
        }
    };
}

#[allow(unreachable_pub)]
#[derive(Clone, Debug)]
pub struct CharSliceSearcher<'a, 'b>(<MultiCharEqPattern<&'b [char]> as Pattern<'a>>::Searcher);

unsafe impl<'a, 'b> Searcher<'a> for CharSliceSearcher<'a, 'b> {
    searcher_methods!(forward);
}

unsafe impl<'a, 'b> ReverseSearcher<'a> for CharSliceSearcher<'a, 'b> {
    searcher_methods!(reverse);
}

impl<'a, 'b> Pattern<'a> for &'b [char] {
    pattern_methods!(CharSliceSearcher<'a, 'b>, MultiCharEqPattern, CharSliceSearcher);
}

#[allow(unreachable_pub)]
#[derive(Clone)]
pub struct CharPredicateSearcher<'a, F>(<MultiCharEqPattern<F> as Pattern<'a>>::Searcher)
where F: FnMut(char) -> bool;

impl<F> fmt::Debug for CharPredicateSearcher<'_, F>
where F: FnMut(char) -> bool
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CharPredicateSearcher")
            .field("haystack", &self.0.haystack)
            .field("char_indices", &self.0.char_indices)
            .finish()
    }
}
unsafe impl<'a, F> Searcher<'a> for CharPredicateSearcher<'a, F>
where F: FnMut(char) -> bool
{
    searcher_methods!(forward);
}

unsafe impl<'a, F> ReverseSearcher<'a> for CharPredicateSearcher<'a, F>
where F: FnMut(char) -> bool
{
    searcher_methods!(reverse);
}

impl<'a, F> Pattern<'a> for F
where F: FnMut(char) -> bool
{
    pattern_methods!(CharPredicateSearcher<'a, F>, MultiCharEqPattern, CharPredicateSearcher);
}

impl<'a, 'b, 'c> Pattern<'a> for &'c &'b str {
    pattern_methods!(StrSearcher<'a, 'b>, |&s| s, |s| s);
}

impl<'a, 'b> Pattern<'a> for &'b str {
    type Searcher = StrSearcher<'a, 'b>;

    fn into_searcher(self, haystack: &'a str) -> StrSearcher<'a, 'b> {
        StrSearcher::new(haystack, self)
    }

    fn is_prefix_of(self, haystack: &'a str) -> bool {
        haystack.as_bytes().starts_with(self.as_bytes())
    }

    fn strip_prefix_of(self, haystack: &'a str) -> Option<&'a str> {
        if self.is_prefix_of(haystack) {
            unsafe { Some(haystack.get_unchecked(self.as_bytes().len()..)) }
        } else {
            None
        }
    }

    fn is_suffix_of(self, haystack: &'a str) -> bool {
        haystack.as_bytes().ends_with(self.as_bytes())
    }

    fn strip_suffix_of(self, haystack: &'a str) -> Option<&'a str> {
        if self.is_suffix_of(haystack) {
            let i = haystack.len() - self.as_bytes().len();
            unsafe { Some(haystack.get_unchecked(..i)) }
        } else {
            None
        }
    }
}

#[allow(unreachable_pub)]
#[derive(Clone, Debug)]
pub struct StrSearcher<'a, 'b> {
    haystack: &'a str,
    needle: &'b str,

    searcher: StrSearcherImpl,
}

#[derive(Clone, Debug)]
enum StrSearcherImpl {
    Empty(EmptyNeedle),
    TwoWay(TwoWaySearcher),
}

#[derive(Clone, Debug)]
struct EmptyNeedle {
    position: usize,
    end: usize,
    is_match_fw: bool,
    is_match_bw: bool,
}

impl<'a, 'b> StrSearcher<'a, 'b> {
    fn new(haystack: &'a str, needle: &'b str) -> StrSearcher<'a, 'b> {
        if needle.is_empty() {
            StrSearcher {
                haystack,
                needle,
                searcher: StrSearcherImpl::Empty(EmptyNeedle {
                    position: 0,
                    end: haystack.len(),
                    is_match_fw: true,
                    is_match_bw: true,
                }),
            }
        } else {
            StrSearcher {
                haystack,
                needle,
                searcher: StrSearcherImpl::TwoWay(TwoWaySearcher::new(
                    needle.as_bytes(),
                    haystack.len(),
                )),
            }
        }
    }
}

unsafe impl<'a, 'b> Searcher<'a> for StrSearcher<'a, 'b> {
    fn haystack(&self) -> &'a str {
        self.haystack
    }

    fn next(&mut self) -> SearchStep {
        match self.searcher {
            StrSearcherImpl::Empty(ref mut searcher) => {
                let is_match = searcher.is_match_fw;
                searcher.is_match_fw = !searcher.is_match_fw;
                let pos = searcher.position;
                match self.haystack[pos..].chars().next() {
                    _ if is_match => SearchStep::Match(pos, pos),
                    None => SearchStep::Done,
                    Some(ch) => {
                        searcher.position += ch.len_utf8();
                        SearchStep::Reject(pos, searcher.position)
                    }
                }
            }
            StrSearcherImpl::TwoWay(ref mut searcher) => {
                if searcher.position == self.haystack.len() {
                    return SearchStep::Done;
                }
                let is_long = searcher.memory == usize::MAX;
                match searcher.next::<RejectAndMatch>(
                    self.haystack.as_bytes(),
                    self.needle.as_bytes(),
                    is_long,
                ) {
                    SearchStep::Reject(a, mut b) => {
                        while !self.haystack.is_char_boundary(b) {
                            b += 1;
                        }
                        searcher.position = cmp::max(b, searcher.position);
                        SearchStep::Reject(a, b)
                    }
                    otherwise => otherwise,
                }
            }
        }
    }

    fn next_match(&mut self) -> Option<(usize, usize)> {
        match self.searcher {
            StrSearcherImpl::Empty(..) => loop {
                match self.next() {
                    SearchStep::Match(a, b) => return Some((a, b)),
                    SearchStep::Done => return None,
                    SearchStep::Reject(..) => {}
                }
            },
            StrSearcherImpl::TwoWay(ref mut searcher) => {
                let is_long = searcher.memory == usize::MAX;
                if is_long {
                    searcher.next::<MatchOnly>(
                        self.haystack.as_bytes(),
                        self.needle.as_bytes(),
                        true,
                    )
                } else {
                    searcher.next::<MatchOnly>(
                        self.haystack.as_bytes(),
                        self.needle.as_bytes(),
                        false,
                    )
                }
            }
        }
    }
}

unsafe impl<'a, 'b> ReverseSearcher<'a> for StrSearcher<'a, 'b> {
    fn next_back(&mut self) -> SearchStep {
        match self.searcher {
            StrSearcherImpl::Empty(ref mut searcher) => {
                let is_match = searcher.is_match_bw;
                searcher.is_match_bw = !searcher.is_match_bw;
                let end = searcher.end;
                match self.haystack[..end].chars().next_back() {
                    _ if is_match => SearchStep::Match(end, end),
                    None => SearchStep::Done,
                    Some(ch) => {
                        searcher.end -= ch.len_utf8();
                        SearchStep::Reject(searcher.end, end)
                    }
                }
            }
            StrSearcherImpl::TwoWay(ref mut searcher) => {
                if searcher.end == 0 {
                    return SearchStep::Done;
                }
                let is_long = searcher.memory == usize::MAX;
                match searcher.next_back::<RejectAndMatch>(
                    self.haystack.as_bytes(),
                    self.needle.as_bytes(),
                    is_long,
                ) {
                    SearchStep::Reject(mut a, b) => {
                        while !self.haystack.is_char_boundary(a) {
                            a -= 1;
                        }
                        searcher.end = cmp::min(a, searcher.end);
                        SearchStep::Reject(a, b)
                    }
                    otherwise => otherwise,
                }
            }
        }
    }

    fn next_match_back(&mut self) -> Option<(usize, usize)> {
        match self.searcher {
            StrSearcherImpl::Empty(..) => loop {
                match self.next_back() {
                    SearchStep::Match(a, b) => return Some((a, b)),
                    SearchStep::Done => return None,
                    SearchStep::Reject(..) => {}
                }
            },
            StrSearcherImpl::TwoWay(ref mut searcher) => {
                let is_long = searcher.memory == usize::MAX;
                if is_long {
                    searcher.next_back::<MatchOnly>(
                        self.haystack.as_bytes(),
                        self.needle.as_bytes(),
                        true,
                    )
                } else {
                    searcher.next_back::<MatchOnly>(
                        self.haystack.as_bytes(),
                        self.needle.as_bytes(),
                        false,
                    )
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct TwoWaySearcher {
    crit_pos: usize,
    crit_pos_back: usize,
    period: usize,
    byteset: u64,
    position: usize,
    end: usize,
    memory: usize,
    memory_back: usize,
}

impl TwoWaySearcher {
    fn new(needle: &[u8], end: usize) -> TwoWaySearcher {
        let (crit_pos_false, period_false) = TwoWaySearcher::maximal_suffix(needle, false);
        let (crit_pos_true, period_true) = TwoWaySearcher::maximal_suffix(needle, true);

        let (crit_pos, period) = if crit_pos_false > crit_pos_true {
            (crit_pos_false, period_false)
        } else {
            (crit_pos_true, period_true)
        };

        if needle[..crit_pos] == needle[period..period + crit_pos] {
            let crit_pos_back = needle.len()
                - cmp::max(
                    TwoWaySearcher::reverse_maximal_suffix(needle, period, false),
                    TwoWaySearcher::reverse_maximal_suffix(needle, period, true),
                );

            TwoWaySearcher {
                crit_pos,
                crit_pos_back,
                period,
                byteset: Self::byteset_create(&needle[..period]),

                position: 0,
                end,
                memory: 0,
                memory_back: needle.len(),
            }
        } else {
            TwoWaySearcher {
                crit_pos,
                crit_pos_back: crit_pos,
                period: cmp::max(crit_pos, needle.len() - crit_pos) + 1,
                byteset: Self::byteset_create(needle),

                position: 0,
                end,
                memory: usize::MAX,
                memory_back: usize::MAX,
            }
        }
    }

    fn byteset_create(bytes: &[u8]) -> u64 {
        bytes.iter().fold(0, |a, &b| (1 << (b & 0x3f)) | a)
    }

    fn byteset_contains(&self, byte: u8) -> bool {
        (self.byteset >> ((byte & 0x3f) as usize)) & 1 != 0
    }

    fn next<S>(&mut self, haystack: &[u8], needle: &[u8], long_period: bool) -> S::Output
    where S: TwoWayStrategy {
        let old_pos = self.position;
        let needle_last = needle.len() - 1;
        'search: loop {
            let tail_byte = match haystack.get(self.position + needle_last) {
                Some(&b) => b,
                None => {
                    self.position = haystack.len();
                    return S::rejecting(old_pos, self.position);
                }
            };

            if S::use_early_reject() && old_pos != self.position {
                return S::rejecting(old_pos, self.position);
            }

            if !self.byteset_contains(tail_byte) {
                self.position += needle.len();
                if !long_period {
                    self.memory = 0;
                }
                continue 'search;
            }

            let start =
                if long_period { self.crit_pos } else { cmp::max(self.crit_pos, self.memory) };
            for i in start..needle.len() {
                if needle[i] != haystack[self.position + i] {
                    self.position += i - self.crit_pos + 1;
                    if !long_period {
                        self.memory = 0;
                    }
                    continue 'search;
                }
            }

            let start = if long_period { 0 } else { self.memory };
            for i in (start..self.crit_pos).rev() {
                if needle[i] != haystack[self.position + i] {
                    self.position += self.period;
                    if !long_period {
                        self.memory = needle.len() - self.period;
                    }
                    continue 'search;
                }
            }

            let match_pos = self.position;

            self.position += needle.len();
            if !long_period {
                self.memory = 0;
            }

            return S::matching(match_pos, match_pos + needle.len());
        }
    }

    fn next_back<S>(&mut self, haystack: &[u8], needle: &[u8], long_period: bool) -> S::Output
    where S: TwoWayStrategy {
        let old_end = self.end;
        'search: loop {
            let front_byte = match haystack.get(self.end.wrapping_sub(needle.len())) {
                Some(&b) => b,
                None => {
                    self.end = 0;
                    return S::rejecting(0, old_end);
                }
            };

            if S::use_early_reject() && old_end != self.end {
                return S::rejecting(self.end, old_end);
            }

            if !self.byteset_contains(front_byte) {
                self.end -= needle.len();
                if !long_period {
                    self.memory_back = needle.len();
                }
                continue 'search;
            }

            let crit = if long_period {
                self.crit_pos_back
            } else {
                cmp::min(self.crit_pos_back, self.memory_back)
            };
            for i in (0..crit).rev() {
                if needle[i] != haystack[self.end - needle.len() + i] {
                    self.end -= self.crit_pos_back - i;
                    if !long_period {
                        self.memory_back = needle.len();
                    }
                    continue 'search;
                }
            }

            let needle_end = if long_period { needle.len() } else { self.memory_back };
            for i in self.crit_pos_back..needle_end {
                if needle[i] != haystack[self.end - needle.len() + i] {
                    self.end -= self.period;
                    if !long_period {
                        self.memory_back = self.period;
                    }
                    continue 'search;
                }
            }

            let match_pos = self.end - needle.len();
            self.end -= needle.len();
            if !long_period {
                self.memory_back = needle.len();
            }

            return S::matching(match_pos, match_pos + needle.len());
        }
    }

    fn maximal_suffix(arr: &[u8], order_greater: bool) -> (usize, usize) {
        let mut left = 0;
        let mut right = 1;
        let mut offset = 0;
        let mut period = 1;

        while let Some(&a) = arr.get(right + offset) {
            let b = arr[left + offset];
            if (a < b && !order_greater) || (a > b && order_greater) {
                right += offset + 1;
                offset = 0;
                period = right - left;
            } else if a == b {
                if offset + 1 == period {
                    right += offset + 1;
                    offset = 0;
                } else {
                    offset += 1;
                }
            } else {
                left = right;
                right += 1;
                offset = 0;
                period = 1;
            }
        }
        (left, period)
    }

    fn reverse_maximal_suffix(arr: &[u8], known_period: usize, order_greater: bool) -> usize {
        let mut left = 0;
        let mut right = 1;
        let mut offset = 0;
        let mut period = 1;
        let n = arr.len();

        while right + offset < n {
            let a = arr[n - (1 + right + offset)];
            let b = arr[n - (1 + left + offset)];
            if (a < b && !order_greater) || (a > b && order_greater) {
                right += offset + 1;
                offset = 0;
                period = right - left;
            } else if a == b {
                if offset + 1 == period {
                    right += offset + 1;
                    offset = 0;
                } else {
                    offset += 1;
                }
            } else {
                left = right;
                right += 1;
                offset = 0;
                period = 1;
            }
            if period == known_period {
                break;
            }
        }
        debug_assert!(period <= known_period);
        left
    }
}

trait TwoWayStrategy {
    type Output;
    fn use_early_reject() -> bool;
    fn rejecting(a: usize, b: usize) -> Self::Output;
    fn matching(a: usize, b: usize) -> Self::Output;
}

enum MatchOnly {}

impl TwoWayStrategy for MatchOnly {
    type Output = Option<(usize, usize)>;

    fn use_early_reject() -> bool {
        false
    }
    fn rejecting(_a: usize, _b: usize) -> Self::Output {
        None
    }
    fn matching(a: usize, b: usize) -> Self::Output {
        Some((a, b))
    }
}

enum RejectAndMatch {}

impl TwoWayStrategy for RejectAndMatch {
    type Output = SearchStep;

    fn use_early_reject() -> bool {
        true
    }
    fn rejecting(a: usize, b: usize) -> Self::Output {
        SearchStep::Reject(a, b)
    }
    fn matching(a: usize, b: usize) -> Self::Output {
        SearchStep::Match(a, b)
    }
}
