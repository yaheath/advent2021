use std::cmp;
use std::ops::Range;
use std::mem;
use num::{Num, One};

pub struct MergedRanges<T,I> {
    values: I,
    last: Option<Range<T>>
}

pub fn merge_ranges<T,I>(iterator: I) -> MergedRanges<T,I::IntoIter>
    where I: IntoIterator<Item=Range<T>>
{
    let mut iterator = iterator.into_iter();
    let last = iterator.next();

    MergedRanges {
        values: iterator,
        last: last,
    }
}

impl<T,I> Iterator for MergedRanges<T,I>
    where T: Ord + Clone, I: Iterator<Item=Range<T>>
{
    type Item = Range<T>;

    fn next(&mut self) -> Option<Range<T>> {
        if let Some(mut last) = self.last.clone() {
            for new in &mut self.values {
                if last.end < new.start {
                    self.last = Some(new);
                    return Some(last);
                }

                last.end = cmp::max(last.end, new.end);
            }

            self.last = None;
            return Some(last);
        }

        None
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BidirRangeInclusive<T: Clone> {
    start: T,
    end: T,
    exhausted: bool,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BidirRangeInclIter<T: Clone>(BidirRangeInclusive<T>);

impl<T: Clone> BidirRangeInclusive<T> {
    pub const fn new(start: T, end: T) -> Self {
        Self { start, end, exhausted: false }
    }
    pub fn into_iter(self) -> BidirRangeInclIter<T> {
        BidirRangeInclIter(self)
    }
    pub fn iter(&self) -> BidirRangeInclIter<T> {
        BidirRangeInclIter(self.clone())
    }
}

impl<T: Clone + PartialOrd + Num + One> Iterator for BidirRangeInclIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.0.exhausted {
            None
        } else if self.0.start > self.0.end {
            let n = self.0.start.clone() - T::one();
            Some(mem::replace(&mut self.0.start, n))
        } else if self.0.start < self.0.end {
            let n = self.0.start.clone() + T::one();
            Some(mem::replace(&mut self.0.start, n))
        } else {
            self.0.exhausted = true;
            Some(self.0.end.clone())
        }
    }
}

impl<T: Clone + PartialOrd + Num + One> DoubleEndedIterator for BidirRangeInclIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.0.exhausted {
            None
        } else if self.0.start > self.0.end {
            let n = self.0.end.clone() + T::one();
            Some(mem::replace(&mut self.0.end, n))
        } else if self.0.start < self.0.end {
            let n = self.0.end.clone() - T::one();
            Some(mem::replace(&mut self.0.end, n))
        } else {
            self.0.exhausted = true;
            Some(self.0.end.clone())
        }
    }
}
