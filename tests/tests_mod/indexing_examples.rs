use multindex::{multiget, multiget_mut, multindex, multindex_mut, std_const_fns::usize_m};

use std::convert::TryInto;

fn stop_unwind(func: impl FnOnce()) -> std::thread::Result<()> {
    use std::panic::{catch_unwind, AssertUnwindSafe};
    catch_unwind(AssertUnwindSafe(func))
}

/// Asserts that two values of the same type have the same value
///
/// Using this macro to ensure that the "expected" values in the tests are not coerced
/// to any other type.
macro_rules! assert_eq_mono {
    ($left:expr, $right:expr $(, $($fmt:tt)* )? ) => ({
        if false {
            match ($left, $right) {
                (mut _left, mut _right)=>{
                    let _left = &mut &mut _left;
                    let _right = &mut &mut _right;
                    [_left, _right];
                }
            }
        }

        match (&$left, &$right) {(__left, __right)=>{
            assert_eq!(*__left, *__right $($($fmt)*)?);
        }}
    })
}

macro_rules! index_with_all {
    (
        array = $array:ident : $array_ty:ty,
        index_args = [$($index:tt)*],
        bounded_exclusive_end = $bounded_exclusive_end:expr,
        expected_ref = $expected_ref:tt : $expected_ref_ty:ty,
        expected_mut = $expected_mut:tt : $expected_mut_ty:ty $(,)*
    ) => ({
        let mut array: $array_ty = $array;

        fn arr_index(array: &$array_ty) -> $expected_ref_ty {
            multindex!(array; $($index)*)
        }
        fn arr_get(array: &$array_ty) -> Option<$expected_ref_ty> {
            multiget!(array; $($index)*)
        }
        fn arr_index_mut(array: &mut $array_ty) -> $expected_mut_ty {
            multindex_mut!(array; $($index)*)
        }
        fn arr_get_mut(array: &mut $array_ty) -> Option<$expected_mut_ty> {
            multiget_mut!(array; $($index)*)
        }

        assert_eq_mono!(arr_index(&array), $expected_ref);
        assert_eq_mono!(arr_get(&array).unwrap(), $expected_ref);

        assert_eq_mono!(arr_index_mut(&mut array), $expected_mut);
        assert_eq_mono!(arr_get_mut(&mut array).unwrap(), $expected_mut);


        ////////////////////////////////////////////////////////////////////////////////
        // Ensuring that it returns the same types and values with slices as arrays,
        // these tests will probably be changed once const generics stabilize and get better.
        //
        // Then the indexing macros can return arrays when indexing arrays with
        // `n..` trailing ranges,
        // and error at compile-time instead of at runtime for out of bounds indices.
        //
        // Example of future code:
        // ```
        //  fn foo<T>(arr: &[T; 6]) -> (&T, &T, &[T; 4]) {
        //      multindex!(arr; 0, 1, 2..)
        //  }
        //  assert_eq!(foo(&[3,5,8,13,21,34]), (&3, &5, &[8,13,21,34]));
        // ```
        fn slice_index(array: &[u16]) -> $expected_ref_ty {
            multindex!(array; $($index)*)
        }
        fn slice_get(array: &[u16]) -> Option<$expected_ref_ty> {
            multiget!(array; $($index)*)
        }
        fn slice_index_mut(array: &mut [u16]) -> $expected_mut_ty {
            multindex_mut!(array; $($index)*)
        }
        fn slice_get_mut(array: &mut [u16]) -> Option<$expected_mut_ty> {
            multiget_mut!(array; $($index)*)
        }

        assert_eq_mono!(slice_index(&array), $expected_ref);
        assert_eq_mono!(slice_get(&array).unwrap(), $expected_ref);

        assert_eq_mono!(slice_index_mut(&mut array), $expected_mut);
        assert_eq_mono!(slice_get_mut(&mut array).unwrap(), $expected_mut);


        ////////////////////////////////////////////////////////////////////////
        //      Index out of bounds

        const SMALL_LEN: usize = usize_m::saturating_sub($bounded_exclusive_end, 1);

        fn tinyarr_index(array: &[u16]) -> $expected_ref_ty {
            let array: &[u16; SMALL_LEN] = array.try_into().unwrap();
            multindex!(array; $($index)*)
        }
        fn tinyarr_get(array: &[u16]) -> Option<$expected_ref_ty> {
            let array: &[u16; SMALL_LEN] = array.try_into().unwrap();
            multiget!(array; $($index)*)
        }
        fn tinyarr_index_mut(array: &mut [u16]) -> $expected_mut_ty {
            let array: &mut [u16; SMALL_LEN] = array.try_into().unwrap();
            multindex_mut!(array; $($index)*)
        }
        fn tinyarr_get_mut(array: &mut [u16]) -> Option<$expected_mut_ty> {
            let array: &mut [u16; SMALL_LEN] = array.try_into().unwrap();
            multiget_mut!(array; $($index)*)
        }

        let bee = $bounded_exclusive_end;
        if bee != 0 {
            let end = bee - 1;

            assert!(stop_unwind(||{ slice_index(&array[..end]); }).is_err());
            assert!(stop_unwind(||{ tinyarr_index(&array[..end]); }).is_err());

            assert!(stop_unwind(||{ slice_index_mut(&mut array[..end]); }).is_err());
            assert!(stop_unwind(||{ tinyarr_index_mut(&mut array[..end]); }).is_err());

            assert_eq_mono!(slice_get(&array[..end]), None);
            assert_eq_mono!(tinyarr_get(&array[..end]), None);
            assert_eq_mono!(slice_get_mut(&mut array[..end]), None);
            assert_eq_mono!(tinyarr_get_mut(&mut array[..end]), None);
        }


    })
}

#[test]
fn each_indexing() {
    const SIZE: usize = 7;
    const ARR: [u16; SIZE] = [3, 5, 8, 13, 21, 34, 55];

    index_with_all! {
        array = ARR : [u16; SIZE],
        index_args = [2],
        bounded_exclusive_end = 3,
        expected_ref = (&8,): (&u16,),
        expected_mut = (&mut 8,): (&mut u16,),
    }
    index_with_all! {
        array = ARR : [u16; SIZE],
        index_args = [2..4],
        bounded_exclusive_end = 4,
        expected_ref = (&[8, 13],): (&[u16; 2],),
        expected_mut = (&mut [8, 13],): (&mut [u16; 2],),
    }
    index_with_all! {
        array = ARR : [u16; SIZE],
        index_args = [2..],
        bounded_exclusive_end = 2,
        expected_ref = (&[8, 13, 21, 34, 55][..],): (&[u16],),
        expected_mut = (&mut [8, 13, 21, 34, 55][..],): (&mut [u16],),
    }
    index_with_all! {
        array = ARR : [u16; SIZE],
        index_args = [..2],
        bounded_exclusive_end = 2,
        expected_ref = (&[3, 5],): (&[u16; 2],),
        expected_mut = (&mut [3, 5],): (&mut [u16; 2],),
    }
    index_with_all! {
        array = ARR : [u16; SIZE],
        index_args = [..],
        bounded_exclusive_end = 0,
        expected_ref = (&[3, 5, 8, 13, 21, 34, 55][..],): (&[u16],),
        expected_mut = (&mut [3, 5, 8, 13, 21, 34, 55][..],): (&mut [u16],),
    }
    index_with_all! {
        array = ARR : [u16; SIZE],
        index_args = [2..=4],
        bounded_exclusive_end = 5,
        expected_ref = (&[8, 13, 21],): (&[u16; 3],),
        expected_mut = (&mut [8, 13, 21],): (&mut [u16; 3],),
    }
    index_with_all! {
        array = ARR : [u16; SIZE],
        index_args = [..=2],
        bounded_exclusive_end = 3,
        expected_ref = (&[3, 5, 8],): (&[u16; 3],),
        expected_mut = (&mut [3, 5, 8],): (&mut [u16; 3],),
    }
}

#[test]
fn multi_indexing() {
    const SIZE: usize = 7;
    const ARR: [u16; SIZE] = [3u16, 5, 8, 13, 21, 34, 55];

    // Many single-elem indices
    index_with_all! {
        array = ARR : [u16; SIZE],
        index_args = [2, 4, 6],
        bounded_exclusive_end = 7,
        expected_ref = (&8, &21, &55): (&u16, &u16, &u16),
        expected_mut = (&mut 8, &mut 21, &mut 55): (&mut u16, &mut u16, &mut u16),
    }

    // With a trailing index
    index_with_all! {
        array = ARR : [u16; SIZE],
        index_args = [0, 1, 2..4, 4..],
        bounded_exclusive_end = 4,
        expected_ref = (&3, &5, &[8, 13], &[21, 34, 55][..]):
                       (&u16, &u16, &[u16; 2], &[u16]),
        expected_mut = (&mut 3, &mut 5, &mut [8, 13], &mut [21, 34, 55][..]):
                       (&mut u16, &mut u16, &mut [u16; 2], &mut [u16]),
    }

    // With RangeFull in the middle and end.
    // The trailing RangeFull produces a slice rather than an reference to an array.
    index_with_all! {
        array = ARR : [u16; SIZE],
        index_args = [0, .., 5, ..],
        bounded_exclusive_end = 5,
        expected_ref = (&3, &[5, 8, 13, 21], &34, &[55][..]):
                       (&u16, &[u16; 4], &u16, &[u16]),
        expected_mut = (&mut 3, &mut [5, 8, 13, 21], &mut 34, &mut [55][..]):
                       (&mut u16, &mut [u16; 4], &mut u16, &mut [u16]),
    }
}

#[test]
fn aliasing_shared_indexing() {
    let arr = [3u16, 5, 8, 13, 21, 34, 55];
    assert_eq!(
        multindex!(arr; 1, 3, 5, 2..6 ),
        (&5, &13, &34, &[8, 13, 21, 34])
    );
    assert_eq!(
        multiget!(arr; 1, 3, 5, 2..6 ),
        Some((&5, &13, &34, &[8, 13, 21, 34]))
    );
}
