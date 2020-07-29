use super::range_conversion_examples::assert_indexargument_invariants;

use multindex::test_utils::bitset::BitSet;

use multindex::pmr::{
    AreAllDisjoint, IndexArgument, IndexArgumentsAndStats as IAAS, IndexProperties, PrenormIndex,
};

use std::convert::TryInto;

const MAX_SLICE_LEN: usize = 64;

// The length of covered integers up to usize max.
const TO_USIZE_MAX: usize = 32;

fn rand_index(state: &fastrand::Rng) -> usize {
    state
        .usize(..MAX_SLICE_LEN * 2 + TO_USIZE_MAX)
        .wrapping_sub(TO_USIZE_MAX)
}

fn rand_optusize(state: &fastrand::Rng) -> Option<usize> {
    if state.bool() {
        Some(rand_index(state))
    } else {
        None
    }
}

fn generate_prenormindex(state: &fastrand::Rng) -> PrenormIndex {
    match state.usize(1..=100) {
        1..=25 => PrenormIndex::Index(rand_index(state)),
        25..=90 => PrenormIndex::Range {
            start: rand_optusize(state),
            end: rand_optusize(state),
        },
        _ => PrenormIndex::InclusiveToMax {
            start: rand_optusize(state),
        },
    }
}

fn generate_prenormindices(state: &fastrand::Rng) -> impl Iterator<Item = PrenormIndex> + '_ {
    std::iter::repeat_with(move || generate_prenormindex(state))
}

// This test is too expensive to run in miri, and it's not using `unsafe` code,
// so regular tests should be enough.
#[cfg(not(miri))]
#[test]
fn range_nonoverlapping() {
    let rng = fastrand::Rng::new();
    let mut iter = generate_prenormindices(&rng);
    let mut prenorms = Vec::with_capacity(4);

    for _ in 0..100000 {
        let mut set = BitSet::EMPTY;

        'inner: loop {
            prenorms.splice(.., iter.by_ref().take(rng.usize(0..=4)));

            let IAAS { stats, ind_args } = match IndexArgument::many_from_prenorm(&prenorms) {
                Ok(x) => x,
                Err(_) => continue 'inner,
            };

            let ind_props = IndexProperties::new(&ind_args, &stats, AreAllDisjoint::Yes);

            if ind_props
                .are_disjoint
                .check_is_expected(&AreAllDisjoint::YES)
                .is_err()
            {
                continue 'inner;
            }

            assert_eq!(MAX_SLICE_LEN, set.capacity());

            fn tou8(n: usize) -> u8 {
                std::cmp::min(n, MAX_SLICE_LEN).try_into().unwrap()
            }

            for ia in &ind_args {
                let start: u8 = tou8(ia.start());
                let end: u8 = tou8(ia.end().unwrap_or(set.capacity()));

                assert_indexargument_invariants(ia);

                if let Some(already_set) = (start..end).find(|x| set.is_set(*x)) {
                    panic!(
                        "\n\
                         The {} bit was already_set in\n\t{:?}\n\
                         Vec<PrenormIndex>: {:#?}\n\
                         Vec<IndexArgument>: {:#?}\n\
                        ",
                        already_set, set, prenorms, ind_args,
                    );
                }

                set.set_range(start..end);
            }

            break 'inner;
        }
    }
}
