use multindex::{
    pmr::{
        AreAllDisjoint, IndexArgument, IndexArgumentStats, IndexArgumentsAndStats as IAAS,
        IndexProperties, PrenormIndex,
    },
    Error,
};

use multindex::prenorm_indices_from as prenorm_from;

#[test]
fn is_unbounded_error() {
    {
        let prenorm = prenorm_from![0, 1.., ..10];
        let res = IndexArgument::many_from_prenorm(&prenorm).unwrap_err();
        assert_eq!(res, Error::NextStartIsUnbounded { current_index: 1 });
    }
    {
        let prenorm = prenorm_from![0, .., ..];
        let res = IndexArgument::many_from_prenorm(&prenorm).unwrap_err();
        assert_eq!(res, Error::NextStartIsUnbounded { current_index: 1 });
    }

    {
        let prenorm = prenorm_from![0, 1.., ..10];
        let res = IndexArgument::from_prenorm(&prenorm, 2, IndexArgumentStats::NEW).unwrap_err();
        assert_eq!(res, Error::PrevEndIsUnbounded { current_index: 2 });
    }
    {
        let prenorm = prenorm_from![0, .., ..];
        let res = IndexArgument::from_prenorm(&prenorm, 2, IndexArgumentStats::NEW).unwrap_err();
        assert_eq!(res, Error::PrevEndIsUnbounded { current_index: 2 });
    }
}

#[test]
fn next_start_is_less_than_current_error() {
    let prenorm = prenorm_from![0, 2, 10.., 9..];

    let res = IndexArgument::many_from_prenorm(&prenorm).unwrap_err();
    assert_eq!(res, Error::NextStartIsLessThanCurrent { current_index: 2 });
}

#[test]
fn inclusive_upto_usize_max_error() {
    {
        let prenorm = prenorm_from![0, 2, 20..=usize::MAX, 10];

        let res = IndexArgument::many_from_prenorm(&prenorm).unwrap_err();
        assert_eq!(res, Error::InclusiveUptoUsizeMax { current_index: 2 });
    }
    {
        let prenorm = prenorm_from![20, ..=usize::MAX, 10];

        let res = IndexArgument::many_from_prenorm(&prenorm).unwrap_err();
        assert_eq!(res, Error::InclusiveUptoUsizeMax { current_index: 1 });
    }
}

#[test]
fn overlapping_index_args_error() {
    #[derive(Copy, Clone)]
    struct Idxs {
        l: u16,
        r: u16,
    }

    fn err_case(prenorm: &[PrenormIndex], indices: Idxs) {
        let IAAS { stats, ind_args } = IndexArgument::many_from_prenorm(&prenorm).unwrap();
        let disjoint = IndexProperties::new(&ind_args, &stats, AreAllDisjoint::Yes).are_disjoint;
        let err = Error::OverlappingIndexArgs {
            left: indices.l,
            right: indices.r,
        };
        assert_eq!(disjoint, AreAllDisjoint::No(err));
    }

    // This tests that two bounded PrenormIndex that are considered to overlap are
    // still considered to overlap after being swapped.
    fn symm_err_case(prenorm: &[PrenormIndex], indices: Idxs) {
        err_case(prenorm, indices);

        let mut prenorm = prenorm.to_vec();
        prenorm.swap(indices.l as usize, indices.r as usize);
        err_case(&prenorm, indices);
    }

    symm_err_case(&prenorm_from![0, 1, 2, 1], Idxs { l: 1, r: 3 });
    symm_err_case(&prenorm_from![0, 4, 5, 1, 1..3, 6], Idxs { l: 3, r: 4 });
    symm_err_case(&prenorm_from![0, 4, 5, 2, 1..=3, 6], Idxs { l: 3, r: 4 });
    symm_err_case(&prenorm_from![1..10, 3..6], Idxs { l: 0, r: 1 });
    symm_err_case(&prenorm_from![40, 1..10, 5..15], Idxs { l: 1, r: 2 });
    symm_err_case(&prenorm_from![40, 1..10, 5..10], Idxs { l: 1, r: 2 });
    symm_err_case(&prenorm_from![40, 1..10, 1..4], Idxs { l: 1, r: 2 });

    err_case(&prenorm_from![1, .., 10, 5], Idxs { l: 1, r: 3 });
    err_case(&prenorm_from![1, 11, 10, ..], Idxs { l: 1, r: 3 });
}
