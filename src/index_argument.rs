use crate::{
    error::Error,
    index_properties::IndexArgumentStats,
    std_const_fns::{slice_m, usize_m},
};

#[cfg(feature = "testing")]
use crate::index_properties::IndexArgumentsAndStats;

use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

////////////////////////////////////////////////////////////////////////////////

pub struct IntoPrenormIndex<T>(pub T);

impl IntoPrenormIndex<usize> {
    #[inline]
    pub const fn call(self) -> PrenormIndex {
        PrenormIndex::Index(self.0)
    }
}

impl IntoPrenormIndex<Range<usize>> {
    #[inline]
    pub const fn call(self) -> PrenormIndex {
        PrenormIndex::Range {
            start: Some(self.0.start),
            end: Some(self.0.end),
        }
    }
}

impl IntoPrenormIndex<RangeFull> {
    #[inline]
    pub const fn call(self) -> PrenormIndex {
        PrenormIndex::Range {
            start: None,
            end: None,
        }
    }
}

impl IntoPrenormIndex<RangeFrom<usize>> {
    #[inline]
    pub const fn call(self) -> PrenormIndex {
        PrenormIndex::Range {
            start: Some(self.0.start),
            end: None,
        }
    }
}

impl IntoPrenormIndex<RangeTo<usize>> {
    #[inline]
    pub const fn call(self) -> PrenormIndex {
        PrenormIndex::Range {
            start: None,
            end: Some(self.0.end),
        }
    }
}

impl IntoPrenormIndex<RangeInclusive<usize>> {
    #[inline]
    pub const fn call(self) -> PrenormIndex {
        let start = Some(*self.0.start());
        let (end, overflowed) = (*self.0.end()).overflowing_add(1);
        if overflowed {
            PrenormIndex::InclusiveToMax { start }
        } else {
            PrenormIndex::Range {
                start,
                end: Some(end),
            }
        }
    }
}

impl IntoPrenormIndex<RangeToInclusive<usize>> {
    #[inline]
    pub const fn call(self) -> PrenormIndex {
        let (end, overflowed) = self.0.end.overflowing_add(1);
        if overflowed {
            PrenormIndex::InclusiveToMax { start: None }
        } else {
            PrenormIndex::Range {
                start: None,
                end: Some(end),
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrenormIndex {
    Index(usize),
    Range {
        start: Option<usize>,
        end: Option<usize>,
    },
    /// A poison value for `n ..= usize::MAX` ranges.
    InclusiveToMax {
        start: Option<usize>,
    },
}

impl PrenormIndex {
    const fn start(&self) -> Option<usize> {
        match *self {
            Self::Index(i) => Some(i),
            Self::Range { start, .. } => start,
            Self::InclusiveToMax { start } => start,
        }
    }
    const fn end(&self) -> Option<usize> {
        match *self {
            Self::Index(i) => Some(i + 1),
            Self::Range { end, .. } => end,
            Self::InclusiveToMax { .. } => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexArgument {
    pub start: usize,
    index_kind: IndexKind,
    saturated_len: usize,
}

impl IndexArgument {
    pub const EMPTY: Self = Self {
        start: 0,
        index_kind: IndexKind::Range,
        saturated_len: 0,
    };

    #[cfg(feature = "testing")]
    pub fn many_from_prenorm(
        prenorm: &[PrenormIndex],
    ) -> Result<IndexArgumentsAndStats<Vec<Self>>, Error> {
        let mut ind_args = vec![Self::EMPTY; prenorm.len()];

        Ok(new_IndexArgumentsAndStats! {
            @initialize
            prenorm = prenorm,
            ind_args = ind_args,
            error_handling(|e| return Err(e) ),
        })
    }

    pub const fn from_prenorm(
        prenorm: &[PrenormIndex],
        current_index: u16,
        mut stats: IndexArgumentStats,
    ) -> Result<(Self, IndexArgumentStats), Error> {
        let candidate_max_end: usize;

        let i = current_index as usize;

        let this = match prenorm[i] {
            PrenormIndex::Index(start) => {
                if start == usize::MAX {
                    return Err(Error::UsizeMaxIndex { current_index });
                }

                candidate_max_end = start + 1;

                Self {
                    start,
                    index_kind: IndexKind::Index,
                    saturated_len: 1,
                }
            }
            PrenormIndex::Range { start, end } => {
                let start = if let Some(start) = start {
                    start
                } else if let Some(prev) = slice_m::get(&prenorm, i.wrapping_sub(1)) {
                    option_unwrap_or_else!(
                        prev.end(),
                        return Err(Error::PrevEndIsUnbounded { current_index })
                    )
                } else
                /*This is the first PrenormIndex in the slice, and it's unbounded*/
                {
                    0
                };

                let index_kind;
                let saturated_len;

                if let Some(end) = end {
                    let len = usize_m::saturating_sub(end, start);
                    candidate_max_end = start + len;

                    index_kind = IndexKind::Range;
                    saturated_len = len;
                } else if let Some(next) = slice_m::get(&prenorm, i + 1) {
                    let next_start = option_unwrap_or_else!(
                        next.start(),
                        return Err(Error::NextStartIsUnbounded { current_index })
                    );

                    candidate_max_end = next_start;

                    let (len, overflowed) = next_start.overflowing_sub(start);

                    if overflowed {
                        return Err(Error::NextStartIsLessThanCurrent { current_index });
                    }

                    index_kind = IndexKind::Range;
                    saturated_len = len;
                } else
                /*This is the last PrenormIndex in the slice, and it's unbounded*/
                {
                    // When the IndexArgument has an unbounded end,
                    // only the start of it needs to be bounds checked.
                    candidate_max_end = start;

                    index_kind = IndexKind::RangeFrom;
                    saturated_len = usize::MAX - start;
                };
                Self {
                    start,
                    index_kind,
                    saturated_len,
                }
            }
            PrenormIndex::InclusiveToMax { .. } => {
                return Err(Error::InclusiveUptoUsizeMax { current_index })
            }
        };

        let prev_max_bounded_end = mem_replace!(
            stats.max_bounded_end,
            usize_m::max(stats.max_bounded_end, candidate_max_end)
        );
        stats.are_sorted = stats.are_sorted && prev_max_bounded_end <= this.start;

        Ok((this, stats))
    }
}

impl IndexArgument {
    /// Whether an IndexArgument intersects another one.
    #[inline]
    pub(crate) const fn intersects(&self, other: &IndexArgument) -> bool {
        let start = usize_m::max(self.start, other.start);
        let end = usize_m::min(self.saturated_end(), other.saturated_end());
        start < end
    }

    #[inline]
    pub const fn index_kind(&self) -> IndexKind {
        self.index_kind
    }

    #[inline]
    pub const fn len_else_zero(&self) -> usize {
        if let IndexKind::RangeFrom = self.index_kind {
            0
        } else {
            self.saturated_len
        }
    }

    #[inline]
    pub const fn start(&self) -> usize {
        self.start
    }

    #[inline]
    pub const fn end(&self) -> Option<usize> {
        if let IndexKind::RangeFrom = self.index_kind {
            None
        } else {
            Some(self.saturated_end())
        }
    }

    /// Gets the exclusive end index of this IndexArgument.
    ///
    /// If `self` has an unbounded end this function returns `usize::MAX`
    #[inline]
    pub const fn saturated_end(&self) -> usize {
        self.start + self.saturated_len
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexKind {
    // Making this a 0 means that RangeArgument::EMPTY is an all-zeroes bitpattern.
    Range = 0,
    Index,
    RangeFrom,
}

pub enum IK_Index {}
pub enum IK_Range {}
pub enum IK_RangeFrom {}

make_type_picker! {
    for[] struct IndexKindPicker[];
    values_to_types = [
        IndexKind::Range => IK_Range,
        IndexKind::Index => IK_Index,
        IndexKind::RangeFrom => IK_RangeFrom,
    ];
}
