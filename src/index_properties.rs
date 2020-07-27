use crate::{
    are_disjoint::AreAllDisjoint,
    error::{Error, ErrorTuple},
    index_argument::IndexArgument,
};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IndexArgumentStats {
    pub max_bounded_end: usize,

    /// Whether all the IndexArgument in a slice are less than the next one,
    /// IndexArguments are less than each other only if the end of one
    /// is less than the start of the next.
    pub are_sorted: bool,
}

impl IndexArgumentStats {
    pub const NEW: Self = Self {
        max_bounded_end: 0,
        are_sorted: true,
    };
}

#[derive(Debug)]
pub struct IndexArgumentsAndStats<IA: ?Sized> {
    pub stats: IndexArgumentStats,
    /// An array of IndexArgument.
    pub ind_args: IA,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct ComputedConstants<IA: ?Sized> {
    pub err: Result<(), Error>,
    pub err_tuple: ErrorTuple,
    pub stats: IndexArgumentStats,
    /// An array of IndexArgument.
    pub ind_args: IA,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct IndexProperties {
    pub are_disjoint: AreAllDisjoint<Error>,
}

impl IndexProperties {
    pub const fn new(
        ind_args: &[IndexArgument],
        stats: &IndexArgumentStats,
        expected: AreAllDisjoint,
    ) -> Self {
        Self {
            are_disjoint: block! {'outer:
                if let (AreAllDisjoint::Yes, false) = (expected, stats.are_sorted) {
                    for_range! { i in 0..ind_args.len() =>
                        for_range!{ j in 0..ind_args.len() =>
                            // Because the `intersects` method is symetric,we don't need to check
                            // both ind_args[i].intersects(&ind_args[j])
                            // and  ind_args[j].intersects(&ind_args[i])
                            if j >= i { break }
                            if ind_args[i].intersects(&ind_args[j]) {
                                break 'outer AreAllDisjoint::No(Error::OverlappingIndexArgs{
                                    left: j as u16,
                                    right: i as u16,
                                });
                            }
                        }
                    }
                }
                expected.with_dummy_error()
            },
        }
    }
}
