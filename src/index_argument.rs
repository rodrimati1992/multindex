use crate::{
    index_properties::IndexArgumentStats,
    std_const_fns::{slice_m, usize_m},
};

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

impl IntoPrenormIndex<RangeInclusive<usize>> {
    #[inline]
    pub const fn call(self) -> PrenormIndex {
        PrenormIndex::Range {
            start: Some(*self.0.start()),
            end: Some(*self.0.end() + 1),
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

impl IntoPrenormIndex<RangeToInclusive<usize>> {
    #[inline]
    pub const fn call(self) -> PrenormIndex {
        PrenormIndex::Range {
            start: None,
            end: Some(self.0.end + 1),
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
}

impl PrenormIndex {
    const fn start(&self) -> Option<usize> {
        match *self {
            Self::Index(i) => Some(i),
            Self::Range { start, .. } => start,
        }
    }
    const fn end(&self) -> Option<usize> {
        match *self {
            Self::Index(i) => Some(i + 1),
            Self::Range { end, .. } => end,
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
        prenorm: &[PrenormIndex]
    ) -> crate::index_properties::IndexArgumentsAndStats<Vec<Self>> {
        let mut ind_args = vec![Self::EMPTY; prenorm.len()];

        new_IndexArgumentsAndStats! {
            @initialize
            prenorm = prenorm,
            ind_args = ind_args,
        }
    }

    pub const fn from_prenorm(
        prenorm: &[PrenormIndex],
        i: usize,
        mut stats: IndexArgumentStats,
    ) -> (Self, IndexArgumentStats) {
        let candidate_max_end: usize;

        let this = match prenorm[i] {
            PrenormIndex::Index(start) => {
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
                    option_expect!(
                        prev.end(),
                        "Expected previous PrenormIndex to have a bounded end"
                    )
                } else
                /*This is the first PrenormIndex in the slice*/
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
                    let next_start = option_expect!(
                        next.start(),
                        "Expected next PrenormIndex to have a bounded end"
                    );

                    candidate_max_end = next_start;

                    let (len, overflowed) = next_start.overflowing_sub(start);

                    ["Expected next PrenormIndex to have a larger or equal start index"]
                        [overflowed as usize];

                    index_kind = IndexKind::Range;
                    saturated_len = len;
                } else
                /*This is the last PrenormIndex in the slice*/
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
        };

        let prev_max_bounded_end = mem_replace!(
            stats.max_bounded_end,
            usize_m::max(stats.max_bounded_end, candidate_max_end)
        );
        stats.are_sorted = stats.are_sorted && prev_max_bounded_end <= this.start;

        (this, stats)
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

    /// Gets the exclusive end index of this IndexArgument.
    ///
    /// If `self` has an unbounded end this function returns `usize::MAX`
    #[inline]
    const fn saturated_end(&self) -> usize {
        self.start + self.saturated_len
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexKind {
    // Making this a 0 means that RangeArgument::EMPTY is an all-zeroes bitpattern.
    Range = 0,
    Index = 1,
    RangeFrom = 2,
}

pub enum IK_Index {}
pub enum IK_Range {}
pub enum IK_RangeFrom {}

make_type_picker! {
    for[] struct IndexKindPicker[];
    types = [
        IK_Range,
        IK_Index,
        IK_RangeFrom,
    ];
}
