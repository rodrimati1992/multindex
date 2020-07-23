use crate::index_argument::{IK_Index, IK_Range, IK_RangeFrom};

use core::marker::PhantomData;

pub struct IndexerParams {
    pub index: isize,
    pub slice_len: usize,
}

impl IndexerParams {
    #[inline(always)]
    pub fn build<Elem, RetArray, IK>(self) -> Indexer<Elem, RetArray, IK> {
        Indexer {
            index: self.index,
            slice_len: self.slice_len,
            _marker: PhantomData,
        }
    }
}

pub struct Indexer<Elem, RetArray, IK> {
    index: isize,
    slice_len: usize,
    _marker: PhantomData<fn() -> (Elem, RetArray, IK)>,
}

pub trait IndexPointer {
    type Elem;
    type Output: ?Sized;

    unsafe fn index_ptr<'a>(
        self,
        base: *const Self::Elem,
        lt: PhantomData<&'a Self::Elem>,
    ) -> &'a Self::Output;
    unsafe fn index_ptr_mut<'a>(
        self,
        base: *mut Self::Elem,
        lt: PhantomData<&'a mut Self::Elem>,
    ) -> &'a mut Self::Output;
}

impl<T, RetArray> IndexPointer for Indexer<T, RetArray, IK_Index> {
    type Elem = T;
    type Output = T;

    #[inline(always)]
    unsafe fn index_ptr<'a>(
        self,
        base: *const Self::Elem,
        _: PhantomData<&'a T>,
    ) -> &'a Self::Output {
        &*base.offset(self.index)
    }

    #[inline(always)]
    unsafe fn index_ptr_mut<'a>(
        self,
        base: *mut Self::Elem,
        _: PhantomData<&'a mut T>,
    ) -> &'a mut Self::Output {
        &mut *base.offset(self.index)
    }
}

impl<T, RetArray> IndexPointer for Indexer<T, RetArray, IK_Range> {
    type Elem = T;
    type Output = RetArray;

    #[inline(always)]
    unsafe fn index_ptr<'a>(
        self,
        base: *const Self::Elem,
        _: PhantomData<&'a T>,
    ) -> &'a Self::Output {
        &*(base.offset(self.index) as *const RetArray)
    }

    #[inline(always)]
    unsafe fn index_ptr_mut<'a>(
        self,
        base: *mut Self::Elem,
        _: PhantomData<&'a mut T>,
    ) -> &'a mut Self::Output {
        &mut *(base.offset(self.index) as *mut RetArray)
    }
}

impl<T, RetArray> IndexPointer for Indexer<T, RetArray, IK_RangeFrom> {
    type Elem = T;
    type Output = [T];

    #[inline(always)]
    unsafe fn index_ptr<'a>(
        self,
        base: *const Self::Elem,
        _: PhantomData<&'a T>,
    ) -> &'a Self::Output {
        core::slice::from_raw_parts(
            base.offset(self.index),
            self.slice_len - self.index as usize,
        )
    }

    #[inline(always)]
    unsafe fn index_ptr_mut<'a>(
        self,
        base: *mut Self::Elem,
        _: PhantomData<&'a mut T>,
    ) -> &'a mut Self::Output {
        core::slice::from_raw_parts_mut(
            base.offset(self.index),
            self.slice_len - self.index as usize,
        )
    }
}
