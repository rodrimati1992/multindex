use core::fmt::{self, Debug};
use core::ops::Range;

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, PartialEq)]
pub struct BitSet(u64);

impl Debug for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#b}", self.0)
    }
}

impl BitSet {
    pub const EMPTY: Self = Self(0);

    pub fn set_range(&mut self, range: Range<u8>) {
        self.0 |= bits_set_in_range(range);
    }

    pub fn is_set(self, bit: u8) -> bool {
        assert!(
            bit < 64,
            "Expected an index smaller than 64, found: {}",
            bit
        );
        (self.0 & (1 << bit)) != 0
    }

    pub fn is_unset(self, bit: u8) -> bool {
        assert!(
            bit < 64,
            "Expected an index smaller than 64, found: {}",
            bit
        );
        (self.0 & (1 << bit)) == 0
    }

    pub fn to_bits(self) -> u64 {
        self.0
    }

    pub fn capacity(self) -> usize {
        64
    }
}

fn bits_set_in_range(range: Range<u8>) -> u64 {
    assert!(
        range.start <= range.end,
        "\nThe range end must be greater than or equal to the start: {:?} \n",
        range,
    );
    assert!(
        range.end <= 64,
        "\nExpected a range with bounds less than or equal to 64, found {:?}\n",
        range,
    );

    let start = range.start;
    saturating_shr_u64(u64::MAX, 64u8 - range.end + range.start).wrapping_shl(start as _)
}

fn saturating_shr_u64(n: u64, shift: u8) -> u64 {
    if shift > 63 {
        0
    } else {
        n.wrapping_shr(shift as _)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_bits() {
        for start in 0..=64 {
            for end in start..=64 {
                let mut set = BitSet::EMPTY;
                let range = start..end;
                set.set_range(range.clone());
                let bits = set.to_bits();
                assert_eq!(
                    bits.count_ones() as usize,
                    range.len(),
                    "\nstart = {}, end = {}\nbits = {:?}\n",
                    start,
                    end,
                    set,
                );

                assert_eq!(
                    bits.trailing_zeros() as usize,
                    if range.len() == 0 { 64 } else { start as usize },
                    "\nstart = {}, end = {}\nbits = {:?}\n",
                    start,
                    end,
                    set,
                );
            }
        }
    }

    #[test]
    #[should_panic]
    fn set_bits_wrong_range() {
        BitSet::EMPTY.set_range(1..0);
    }

    #[test]
    fn is_set() {
        macro_rules! assert_set {
            ($set:ident [$index:expr], $expected_is_set:expr) => {{
                let isset = $set.is_set($index);
                let isunset = $set.is_unset($index);
                assert!(
                    isset == $expected_is_set && isset != isunset,
                    "is_set: {} is_unset: {}\nset: {:?}\n",
                    isset,
                    isunset,
                    $set,
                );
            }};
        }

        {
            let mut set = BitSet::EMPTY;
            set.set_range(0..0);
            assert_set!(set[0], false);
            assert_set!(set[1], false);
        }
        {
            let mut set = BitSet::EMPTY;
            set.set_range(0..1);
            assert_set!(set[0], true);
            assert_set!(set[1], false);
        }
        {
            let mut set = BitSet::EMPTY;
            set.set_range(10..20);
            assert_set!(set[8], false);
            assert_set!(set[9], false);
            for i in 10..20 {
                assert_set!(set[i], true);
            }
            assert_set!(set[20], false);
            assert_set!(set[21], false);
        }
        {
            let mut set = BitSet::EMPTY;
            set.set_range(63..64);
            assert_set!(set[62], false);
            assert_set!(set[63], true);
        }
    }
}
