use crate::{index_argument::IndexArgument, uniqueness::AreAllUnique};

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
pub struct IndexProperties {
    pub uniqueness: AreAllUnique,
}

impl IndexProperties {
    pub const fn new(
        IndexArgumentsAndStats { ind_args, stats }: &IndexArgumentsAndStats<[IndexArgument]>,
        expected: AreAllUnique,
    ) -> Self {
        Self {
            uniqueness: block! {'outer:
                if let (AreAllUnique::Yes, false) = (expected, stats.are_sorted) {
                    for_range! { i in 0..ind_args.len() =>
                        for_range!{ j in 0..ind_args.len() =>
                            // Because the `intersects` method is symetric,we don't need to check
                            // both ind_args[i].intersects(&ind_args[j])
                            // and  ind_args[j].intersects(&ind_args[i])
                            if j >= i { break }
                            if ind_args[i].intersects(&ind_args[j]) {
                                break 'outer AreAllUnique::No;
                            }
                        }
                    }
                }
                expected
            },
        }
    }
}
