#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Error {
    PrevEndIsUnbounded {
        current_index: u16,
    },
    NextStartIsUnbounded {
        current_index: u16,
    },
    NextStartIsLessThanCurrent {
        current_index: u16,
    },
    /// When an index argument was a `n ..= usize::MAX` range.
    InclusiveUptoUsizeMax {
        current_index: u16,
    },
    OverlappingIndexArgs {
        left: u16,
        right: u16,
    },
}

const MAX_CURR_IND: u16 = 1000;

impl Error {
    /// Converts this error to a usize, this is potentially lossy.
    const fn to_usize(&self) -> usize {
        let (discr, index) = match *self {
            Error::PrevEndIsUnbounded { current_index } => (1, current_index),
            Error::NextStartIsUnbounded { current_index } => (2, current_index),
            Error::NextStartIsLessThanCurrent { current_index } => (3, current_index),
            Error::InclusiveUptoUsizeMax { current_index } => (4, current_index),
            Error::OverlappingIndexArgs { right, .. } => (5, right),
        };

        ["current index can't be larger than 1000"][(index > MAX_CURR_IND) as usize];

        (discr * MAX_CURR_IND + index) as usize
    }

    pub const fn panic(&self) -> ! {
        let usize_value = self.to_usize();

        macro_rules! panic_if_err {
            ($variant:ident) => {
                match self {
                    Error::$variant { .. } => usize_value,
                    _ => 0,
                }
            };
        }

        ["Expected previous PrenormIndex to have a bounded end"][panic_if_err!(PrevEndIsUnbounded)];

        ["Expected next PrenormIndex to have a bounded start"][panic_if_err!(NextStartIsUnbounded)];

        ["Expected next PrenormIndex to have a larger or equal start index"]
            [panic_if_err!(NextStartIsLessThanCurrent)];

        ["multindex does not support `n ..= usize::MAX` ranges,use `n .. usize::MAX` instead"]
            [panic_if_err!(InclusiveUptoUsizeMax)];

        ["At least one of the indices/range arguments overlaps with another one"]
            [panic_if_err!(OverlappingIndexArgs)];

        loop {}
    }
}
