use multindex::pmr::{
    AreAllDisjoint, IndexArgument, IndexArgumentsAndStats as IAAS, IndexKind, IndexProperties,
    PrenormIndex,
};

use multindex::prenorm_indices_from;

#[derive(Debug, PartialEq, Clone)]
pub struct AssertVars {
    pub start: usize,
    pub sat_end: usize,
    pub len_or0: usize,
    pub index_kind: IndexKind,
}

impl AssertVars {
    fn from_tuple((start, sat_end, len_or0, index_kind): (usize, usize, usize, IndexKind)) -> Self {
        Self {
            start,
            sat_end,
            len_or0,
            index_kind,
        }
    }

    fn from_indargs(this: &IndexArgument) -> Self {
        Self {
            start: this.start(),
            sat_end: this.saturated_end(),
            len_or0: this.len_else_zero(),
            index_kind: this.index_kind(),
        }
    }
}

pub fn assert_indexargument_invariants(indarg: &IndexArgument) {
    let vars = AssertVars::from_indargs(indarg);

    let index_and_range_assertions = || {
        let end = indarg.end().unwrap();
        assert_eq!(indarg.saturated_end(), end, "{:?}", vars);
        assert_eq!(
            indarg.len_else_zero(),
            indarg.saturated_end() - indarg.start(),
            "{:?}",
            vars
        );
    };

    match indarg.index_kind() {
        IndexKind::Index => {
            assert_eq!(indarg.len_else_zero(), 1, "{:?}", vars);
            index_and_range_assertions();
        }
        IndexKind::Range => {
            index_and_range_assertions();
        }
        IndexKind::RangeFrom => {
            assert_eq!(indarg.end(), None, "{:?}", vars);
            assert_eq!(indarg.saturated_end(), usize::MAX, "{:?}", vars);
            assert_eq!(indarg.len_else_zero(), 0, "{:?}", vars);
            assert_eq!(indarg.index_kind(), IndexKind::RangeFrom, "{:?}", vars);
        }
    }
}

#[cfg(not(miri))]
// Checks that the max_bounded_end property is the same even if the elements are shifted around.
fn check_max_bounded_end_shuffled(prenorm: &mut [PrenormIndex], max_bounded_end: usize) {
    fastrand::shuffle(prenorm);
    let stats = IndexArgument::many_from_prenorm(prenorm).unwrap().stats;
    assert_eq!(stats.max_bounded_end, max_bounded_end);
}

#[cfg(miri)]
fn check_max_bounded_end_shuffled(_prenorm: &mut [PrenormIndex], _max_bounded_end: usize) {}

macro_rules! indarg_asserts {
    (
        prenorm = $prenorm:expr,
        max_bounded_end = $max_bounded_end:expr,
        are_sorted = $are_sorted:expr,
        are_disjoint = $are_disjoint:pat,
        expected_start_end_len = [
            $($expected:expr,)*
        ],
    ) => (
        let prenorm = $prenorm;
        let IAAS{stats, ind_args} = IndexArgument::many_from_prenorm(&prenorm).unwrap();
        let ind_props = IndexProperties::new(&ind_args, &stats, AreAllDisjoint::Yes);

        assert_eq!(stats.max_bounded_end, $max_bounded_end);
        assert_eq!(stats.are_sorted, $are_sorted);
        assert!(
            matches!(ind_props.are_disjoint, $are_disjoint),
            "{:?}",
            ind_props.are_disjoint,
        );

        let expected = [$( AssertVars::from_tuple($expected) ,)*];
        for (indarg, exp) in ind_args.iter().zip(expected.iter().cloned()) {
            let found = AssertVars::from_indargs(indarg);
            assert_indexargument_invariants(indarg);

            assert_eq!(found, exp);
        }
    )
}

const UMAX: usize = usize::MAX;

#[test]
fn range_index_argument() {
    {
        let prenorm = prenorm_indices_from!(0..0, 0..10, 10..UMAX, 10..5, UMAX..UMAX,);

        assert_eq!(
            prenorm,
            [
                PrenormIndex::Range {
                    start: Some(0),
                    end: Some(0)
                },
                PrenormIndex::Range {
                    start: Some(0),
                    end: Some(10)
                },
                PrenormIndex::Range {
                    start: Some(10),
                    end: Some(UMAX)
                },
                PrenormIndex::Range {
                    start: Some(10),
                    end: Some(5)
                },
                PrenormIndex::Range {
                    start: Some(UMAX),
                    end: Some(UMAX)
                },
            ],
        );

        indarg_asserts!(
            prenorm = prenorm,
            max_bounded_end = UMAX,
            are_sorted = false,
            are_disjoint = AreAllDisjoint::Yes,
            expected_start_end_len = [
                (0, 0, 0, IndexKind::Range),
                (0, 10, 10, IndexKind::Range),
                (10, UMAX, UMAX - 10, IndexKind::Range),
                (10, 10, 0, IndexKind::Range),
                (UMAX, UMAX, 0, IndexKind::Range),
            ],
        );
    }
    {
        let prenorm = prenorm_indices_from!(0..0, 0..10, 10..40,);

        let stats = IndexArgument::many_from_prenorm(&prenorm).unwrap().stats;

        assert_eq!(stats.max_bounded_end, 40);
        assert!(stats.are_sorted);

        let mut prenorm = prenorm;
        for _ in 0..10 {
            check_max_bounded_end_shuffled(&mut prenorm, 40);
        }
    }
}

#[test]
fn range_to_index_argument() {
    {
        let prenorm = prenorm_indices_from!(..0, ..10, ..UMAX,);

        assert_eq!(
            prenorm,
            [
                PrenormIndex::Range {
                    start: None,
                    end: Some(0)
                },
                PrenormIndex::Range {
                    start: None,
                    end: Some(10)
                },
                PrenormIndex::Range {
                    start: None,
                    end: Some(UMAX)
                },
            ],
        );

        indarg_asserts!(
            prenorm = prenorm,
            max_bounded_end = UMAX,
            are_sorted = true,
            are_disjoint = AreAllDisjoint::Yes,
            expected_start_end_len = [
                (0, 0, 0, IndexKind::Range),
                (0, 10, 10, IndexKind::Range),
                (10, UMAX, UMAX - 10, IndexKind::Range),
            ],
        );
    }
    {
        let prenorm = prenorm_indices_from!(..0, ..10, ..40,);

        indarg_asserts!(
            prenorm = prenorm,
            max_bounded_end = 40,
            are_sorted = true,
            are_disjoint = AreAllDisjoint::Yes,
            expected_start_end_len = [
                (0, 0, 0, IndexKind::Range),
                (0, 10, 10, IndexKind::Range),
                (10, 40, 30, IndexKind::Range),
            ],
        );

        let mut prenorm = prenorm;
        for _ in 0..10 {
            check_max_bounded_end_shuffled(&mut prenorm, 40);
        }
    }
}

#[test]
fn range_from_index_argument() {
    {
        let prenorm = prenorm_indices_from!(0.., 10.., 40..,);

        assert_eq!(
            prenorm,
            [
                PrenormIndex::Range {
                    start: Some(0),
                    end: None
                },
                PrenormIndex::Range {
                    start: Some(10),
                    end: None
                },
                PrenormIndex::Range {
                    start: Some(40),
                    end: None
                },
            ],
        );

        indarg_asserts!(
            prenorm = prenorm,
            max_bounded_end = 40,
            are_sorted = true,
            are_disjoint = AreAllDisjoint::Yes,
            expected_start_end_len = [
                (0, 10, 10, IndexKind::Range),
                (10, 40, 30, IndexKind::Range),
                (
                    40,
                    UMAX,
                    0, /*unbounded length == 0*/
                    IndexKind::RangeFrom
                ),
            ],
        );
    }
}

#[test]
fn range_to_inclusive_index_argument() {
    {
        let prenorm = prenorm_indices_from!(..=0, ..=10, ..=UMAX - 1,);

        assert_eq!(
            prenorm,
            [
                PrenormIndex::Range {
                    start: None,
                    end: Some(1)
                },
                PrenormIndex::Range {
                    start: None,
                    end: Some(11)
                },
                PrenormIndex::Range {
                    start: None,
                    end: Some(UMAX)
                },
            ],
        );

        indarg_asserts!(
            prenorm = prenorm,
            max_bounded_end = UMAX,
            are_sorted = true,
            are_disjoint = AreAllDisjoint::Yes,
            expected_start_end_len = [
                (0, 1, 1, IndexKind::Range),
                (1, 11, 10, IndexKind::Range),
                (11, UMAX, UMAX - 11, IndexKind::Range),
            ],
        );
    }
    {
        let mut prenorm = prenorm_indices_from!(..=0, ..=10, ..=40,);

        indarg_asserts!(
            prenorm = prenorm,
            max_bounded_end = 41,
            are_sorted = true,
            are_disjoint = AreAllDisjoint::Yes,
            expected_start_end_len = [
                (0, 1, 1, IndexKind::Range),
                (1, 11, 10, IndexKind::Range),
                (11, 41, 30, IndexKind::Range),
            ],
        );

        for _ in 0..10 {
            check_max_bounded_end_shuffled(&mut prenorm, 41);
        }
    }
}

#[test]
fn range_inclusive_index_argument() {
    {
        let prenorm = prenorm_indices_from!(0..=0, 1..=10, 11..=UMAX - 1, UMAX..=UMAX - 1, 10..=5,);

        assert_eq!(
            prenorm,
            [
                PrenormIndex::Range {
                    start: Some(0),
                    end: Some(1)
                },
                PrenormIndex::Range {
                    start: Some(1),
                    end: Some(11)
                },
                PrenormIndex::Range {
                    start: Some(11),
                    end: Some(UMAX)
                },
                PrenormIndex::Range {
                    start: Some(UMAX),
                    end: Some(UMAX)
                },
                PrenormIndex::Range {
                    start: Some(10),
                    end: Some(6)
                },
            ],
        );

        indarg_asserts!(
            prenorm = prenorm,
            max_bounded_end = UMAX,
            are_sorted = false,
            are_disjoint = AreAllDisjoint::Yes,
            expected_start_end_len = [
                (0, 1, 1, IndexKind::Range),
                (1, 11, 10, IndexKind::Range),
                (11, UMAX, UMAX - 11, IndexKind::Range),
                (UMAX, UMAX, 0, IndexKind::Range),
                (10, 10, 0, IndexKind::Range),
            ],
        );
    }
    {
        let mut prenorm = prenorm_indices_from!(0..=10, 11..=14, 15..=40);

        let stats = IndexArgument::many_from_prenorm(&prenorm).unwrap().stats;

        assert_eq!(stats.max_bounded_end, 41);
        assert!(stats.are_sorted);

        for _ in 0..10 {
            check_max_bounded_end_shuffled(&mut prenorm, 41);
        }
    }
}

#[test]
fn range_full_index_argument() {
    {
        let prenorm = prenorm_indices_from!(..);

        assert_eq!(
            prenorm,
            [PrenormIndex::Range {
                start: None,
                end: None,
            },]
        );

        indarg_asserts!(
            prenorm = prenorm,
            max_bounded_end = 0,
            are_sorted = true,
            are_disjoint = AreAllDisjoint::Yes,
            expected_start_end_len = [(0, usize::MAX, 0, IndexKind::RangeFrom),],
        );
    }
    {
        let prenorm = prenorm_indices_from!(10..20, ..);

        assert_eq!(
            prenorm,
            [
                PrenormIndex::Range {
                    start: Some(10),
                    end: Some(20),
                },
                PrenormIndex::Range {
                    start: None,
                    end: None,
                },
            ]
        );

        indarg_asserts!(
            prenorm = prenorm,
            max_bounded_end = 20,
            are_sorted = true,
            are_disjoint = AreAllDisjoint::Yes,
            expected_start_end_len = [
                (10, 20, 10, IndexKind::Range),
                (20, usize::MAX, 0, IndexKind::RangeFrom),
            ],
        );
    }
    {
        let prenorm = prenorm_indices_from!(.., 20..30, 31);

        assert_eq!(
            prenorm,
            [
                PrenormIndex::Range {
                    start: None,
                    end: None,
                },
                PrenormIndex::Range {
                    start: Some(20),
                    end: Some(30),
                },
                PrenormIndex::Index(31),
            ]
        );

        indarg_asserts!(
            prenorm = prenorm,
            max_bounded_end = 32,
            are_sorted = true,
            are_disjoint = AreAllDisjoint::Yes,
            expected_start_end_len = [
                (0, 20, 20, IndexKind::Range),
                (20, 30, 10, IndexKind::Range),
                (31, 32, 1, IndexKind::Index),
            ],
        );
    }
    {
        let prenorm = prenorm_indices_from!(10, .., 20, 21);

        assert_eq!(
            prenorm,
            [
                PrenormIndex::Index(10),
                PrenormIndex::Range {
                    start: None,
                    end: None,
                },
                PrenormIndex::Index(20),
                PrenormIndex::Index(21),
            ]
        );

        indarg_asserts!(
            prenorm = prenorm,
            max_bounded_end = 22,
            are_sorted = true,
            are_disjoint = AreAllDisjoint::Yes,
            expected_start_end_len = [
                (10, 11, 1, IndexKind::Index),
                (11, 20, 9, IndexKind::Range),
                (20, 21, 1, IndexKind::Index),
                (21, 22, 1, IndexKind::Index),
            ],
        );
    }
}
