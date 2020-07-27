#![allow(dead_code)]

pub mod slice_m {
    #[inline]
    pub const fn first<T>(this: &[T]) -> Option<&T> {
        match this {
            [] => None,
            [first, ..] => Some(first),
        }
    }

    #[inline]
    pub const fn last<T>(this: &[T]) -> Option<&T> {
        match this {
            [] => None,
            [.., last] => Some(last),
        }
    }

    #[inline]
    pub const fn get<T>(this: &[T], index: usize) -> Option<&T> {
        if index < this.len() {
            Some(&this[index])
        } else {
            None
        }
    }
}

pub mod usize_m {
    pub const fn max(l: usize, r: usize) -> usize {
        if l >= r {
            l
        } else {
            r
        }
    }

    pub const fn min(l: usize, r: usize) -> usize {
        if l <= r {
            l
        } else {
            r
        }
    }

    pub const fn saturating_sub(l: usize, r: usize) -> usize {
        if l < r {
            0
        } else {
            l - r
        }
    }
}

pub mod option_m {
    pub const fn is_some<T>(this: &Option<T>) -> bool {
        matches!(this, Some(_))
    }
    pub const fn is_none<T>(this: &Option<T>) -> bool {
        matches!(this, None)
    }
}

pub mod result_m {
    pub const fn is_ok<T, E>(this: &Result<T, E>) -> bool {
        matches!(this, Ok(_))
    }
    pub const fn is_err<T, E>(this: &Result<T, E>) -> bool {
        matches!(this, Err(_))
    }
}
