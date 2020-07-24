use core::marker::PhantomData;

////////////////////////////////////////////////////////////////////////////////

// Used to get a `&T` from both a `T` and a `&T`
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait BorrowSelf {
    fn _11748397628858797803_borrow_self(&self) -> &Self;
    fn _11748397628858797803_borrow_self_mut(&mut self) -> &mut Self;
}

impl<T> BorrowSelf for T
where
    T: ?Sized,
{
    #[inline(always)]
    fn _11748397628858797803_borrow_self(&self) -> &Self {
        self
    }

    #[inline(always)]
    fn _11748397628858797803_borrow_self_mut(&mut self) -> &mut Self {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Used to emulate inherent associated types
pub trait AssocType {
    type Assoc: ?Sized;
}

////////////////////////////////////////////////////////////////////////////////

/// Used to decompose a slice into its parts
///
/// The `lifetime` field can be used to create references with the same lifetime.
pub struct SliceParts<'a, T> {
    pub ptr: *const T,
    pub len: usize,
    pub lifetime: PhantomData<&'a T>,
}

impl<'a, T> SliceParts<'a, T> {
    #[inline(always)]
    pub const fn new(slice: &'a [T]) -> Self {
        Self {
            ptr: slice.as_ptr(),
            len: slice.len(),
            lifetime: PhantomData,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Used to decompose a slice into its parts
///
/// The `lifetime` field can be used to create references with the same lifetime.
pub struct SlicePartsMut<'a, T> {
    pub ptr: *mut T,
    pub len: usize,
    pub lifetime: PhantomData<&'a mut T>,
}

impl<'a, T> SlicePartsMut<'a, T> {
    #[inline(always)]
    pub fn new(slice: &'a mut [T]) -> Self {
        Self {
            ptr: slice.as_mut_ptr(),
            len: slice.len(),
            lifetime: PhantomData,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Error message when the maximum exclusive end is outside the bounds of the slice.
#[cold]
#[inline(never)]
pub fn panic_on_oob_max_index(maximum_index: usize, slice_len: usize) -> ! {
    panic!(
        "Maximum index is {}, but slice length is {}",
        maximum_index, slice_len,
    );
}
