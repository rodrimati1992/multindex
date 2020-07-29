use core::marker::PhantomData;

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
    /// When an index argument was is `usize::MAX`.
    UsizeMaxIndex {
        current_index: u16,
    },
    OverlappingIndexArgs {
        left: u16,
        right: u16,
    },
}

#[derive(Debug, Copy, Clone)]
pub struct ErrorTuple {
    pub kind: ErrorKind,
    pub first: usize,
    pub second: usize,
}

impl ErrorTuple {
    const fn new(kind: ErrorKind, first: usize, second: usize) -> Self {
        Self {
            kind,
            first,
            second,
        }
    }

    pub const OK: Self = Self::new(ErrorKind::OK, 0, 0);
}

pub const fn result_to_tuple(result: Result<(), Error>) -> ErrorTuple {
    match result {
        Ok(()) => ErrorTuple::OK,
        Err(m_error) => m_error.to_tuple(),
    }
}

impl Error {
    pub const fn to_tuple(&self) -> ErrorTuple {
        match *self {
            Error::PrevEndIsUnbounded { current_index } => {
                ErrorTuple::new(ErrorKind::PrevEndIsUnbounded, current_index as _, 0)
            }
            Error::NextStartIsUnbounded { current_index } => {
                ErrorTuple::new(ErrorKind::NextStartIsUnbounded, current_index as _, 0)
            }
            Error::NextStartIsLessThanCurrent { current_index } => {
                ErrorTuple::new(ErrorKind::NextStartIsLessThanCurrent, current_index as _, 0)
            }
            Error::InclusiveUptoUsizeMax { current_index } => {
                ErrorTuple::new(ErrorKind::InclusiveUptoUsizeMax, current_index as _, 0)
            }
            Error::UsizeMaxIndex { current_index } => {
                ErrorTuple::new(ErrorKind::UsizeMaxIndex, current_index as _, 0)
            }
            Error::OverlappingIndexArgs { left, right } => {
                ErrorTuple::new(ErrorKind::OverlappingIndexArgs, left as _, right as _)
            }
        }
    }

    /*
    /// Converts this error to a usize, this is potentially lossy.
    const fn to_usize(&self) -> usize {
        const MAX_CURR_IND: u16 = 1000;

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
    */
}

macro_rules! declare_error_tys {
    (
        $(
            $variant:ident => $error:ident <$($typaram:ident),* $(,)*>,
        )*
    ) => (
        #[repr(u8)]
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum ErrorKind {
            $($variant,)*
        }

        make_type_picker! {
            for[A, B,] struct ErrorPicker[A, B,];
            values_to_types = [
                $( ErrorKind::$variant => $error<$($typaram),*>, )*
            ];
        }

        $(
            pub struct $error<$($typaram),*>(
                PhantomData<fn()->($($typaram,)*)>,
            );

            impl<$($typaram,)*> $error<$($typaram,)*> {
                pub const NEW: Self = Self(PhantomData);
            }
        )*
    )
}

declare_error_tys!(
    OK                         => NoErrorsFound<>,
    PrevEndIsUnbounded         => PreviousEndIsUnbounded__CurrentArgumentIs<A>,
    NextStartIsUnbounded       => NextStartIsUnbounded__CurrentArgumentIs<A>,
    NextStartIsLessThanCurrent => NextStartIsLessThanCurrent__CurrentArgumentIs<A>,
    InclusiveUptoUsizeMax      => InclusiveUptoUsizeMax__CurrentArgumentis<A>,
    UsizeMaxIndex              => UsizeMaxIndex__CurrentArgumentis<A>,
    OverlappingIndexArgs       => OverlappingIndexArguments__ArgumentsAre<A, B>,
);
