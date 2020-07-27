#[macro_export]
macro_rules! _in_bounds_behavior {
    (option, $expr:expr) => {
        $crate::pmr::Some($expr)
    };
    (panic, $expr:expr) => {
        $expr
    };
}

#[macro_export]
macro_rules! _on_out_of_bounds {
    (panic, ind_stats = $ind_stats:expr, slice = $slice:ident) => {
        $crate::pmr::panic_on_oob_max_index($ind_stats.max_bounded_end, $slice.len());
    };
    (option, $($anything:tt)*) => {
        $crate::pmr::None
    };
}

#[macro_export]
macro_rules! _index_impl {
    (
        slice = $slice:expr;
        indices[];
        $args:tt
    ) => ({
        let _ = $slice;
        ()
    });
    (
        slice = $slice:expr;
        indices[$($index:expr,)+];
        $args:tt
    ) => (
        $crate::_index_impl!{
            @accum
            $slice;
            []
            [$($index,)*]
            [
                0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
                16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
                32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47
                48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63
                64
            ]
            $args
        }
    );
    (@accum
        $slice:expr;
        [$($prev:tt)*]
        [ $index:expr, $($rem_index:tt)*]
        [ $count:tt $($rem_count:tt)*]
        $args:tt
    )=>{
        $crate::_index_impl!{
            @accum
            $slice;
            [$($prev)* ($count, $index) ]
            [$($rem_index)*]
            [$($rem_count)*]
            $args
        }
    };
    (@accum
        $slice:expr;
        [$(($count:tt, $index:expr))*]
        []
        [$index_arg_count:tt $($rem_count:tt)*]
        (
            expected_are_disjoint = $expected_are_disjoint:expr,
            on_out_of_bounds = $on_out_of_bounds:tt,
            auto_borrow_method = $auto_borrow_method:ident,
            slice_parts = $slice_parts:ident,
            index_method = $index_method:ident,
        )
    )=>({
        const __COMP_CONSTS: &$crate::pmr::ComputedConstants<
            [$crate::pmr::IndexArgument; $index_arg_count]
        > = {
            let mut comp_consts;
            $crate::block!{'constant:
                comp_consts = $crate::new_IndexArgumentsAndStats!(@from_index_macro; $($index,)*);
                if $crate::pmr::is_err(&comp_consts.err) { break 'constant; }

                let props = $crate::pmr::IndexProperties::new(
                    &comp_consts.ind_args,
                    &comp_consts.stats,
                    $expected_are_disjoint,
                );

                // This errors if the expected are_disjoint of the indices differs
                // from the actual are_disjoint.
                //
                // Passing `AreAllDisjoint::No` to IndexProperties's constructor
                // skips the are_disjoint checks, always returning `AreAllDisjoint::No`.
                comp_consts.err = props.are_disjoint.check_is_expected(&$expected_are_disjoint);
                if $crate::pmr::is_err(&comp_consts.err) { break 'constant; }
            }
            comp_consts.err_tuple = $crate::error::result_to_tuple(comp_consts.err);
            &{comp_consts}
        };

        const _: $crate::pmr::NoErrorsFound =
            <$crate::error_tuple_to_error_type!(__COMP_CONSTS.err_tuple)>::NEW;

        use $crate::utils::BorrowSelf as _;
        // The `*borrow_**` method here ensures that `$slice`
        // is not more layers of mutable references than necessary.
        //
        // The match ensures that temporary expressions passed to this macro lives
        // for the duration of the scope
        match $slice.$auto_borrow_method() { slice => unsafe{
            use $crate::pmr::{
                Indexer, IndexerParams, IndexArgument,
                IndexPointer, $slice_parts,
            };

            if __COMP_CONSTS.stats.max_bounded_end > slice.len() {
                $crate::_on_out_of_bounds!(
                    $on_out_of_bounds,
                    ind_stats = __COMP_CONSTS.stats,
                    slice = slice
                )
            } else {
                // `lifetime` is a `PhantomData<&'a (mut) T>` used to ensure that the
                // reference returned by `IndexPointer::index_ptr_*` has the correct lifetime.
                let $slice_parts{ptr, len, lifetime} = $slice_parts::new({slice});

                let ret = ($(
                    {
                        const __IND_ARG: &IndexArgument = &__COMP_CONSTS.ind_args[$count];

                        type __IndexerAlias<T> = Indexer<
                            T,
                            [T; __IND_ARG.len_else_zero()],
                            $crate::index_argument_to_kind_type!(__IND_ARG),
                        >;

                        let caster: __IndexerAlias<_> =
                            IndexerParams{
                                index: __IND_ARG.start as _,
                                slice_len: len,
                            }.build();

                        IndexPointer::$index_method(caster, ptr, lifetime)
                    },
                )*);
                $crate::_in_bounds_behavior!($on_out_of_bounds, ret )
            }

        }}

    });

}

#[macro_export]
macro_rules! multindex {
    ( $slice:expr; $($index:expr),* $(,)? ) => (
        $crate::_index_impl!{
            slice = $slice;
            indices[$($index,)*];
            (
                expected_are_disjoint = $crate::pmr::AreAllDisjoint::NO,
                on_out_of_bounds = panic,
                auto_borrow_method = _11748397628858797803_borrow_self,
                slice_parts = SliceParts,
                index_method = index_ptr,
            )
        }
    );
}

#[macro_export]
macro_rules! multindex_mut {
    ( $slice:expr; $($index:expr),* $(,)? ) => (
        $crate::_index_impl!{
            slice = $slice;
            indices[$($index,)*];
            (
                expected_are_disjoint = $crate::pmr::AreAllDisjoint::YES,
                on_out_of_bounds = panic,
                auto_borrow_method = _11748397628858797803_borrow_self_mut,
                slice_parts = SlicePartsMut,
                index_method = index_ptr_mut,
            )
        }
    );
}

#[macro_export]
macro_rules! multiget {
    ( $slice:expr; $($index:expr),* $(,)? ) => (
        $crate::_index_impl!{
            slice = $slice;
            indices[$($index,)*];
            (
                expected_are_disjoint = $crate::pmr::AreAllDisjoint::NO,
                on_out_of_bounds = option,
                auto_borrow_method = _11748397628858797803_borrow_self,
                slice_parts = SliceParts,
                index_method = index_ptr,
            )
        }
    );
}

#[macro_export]
macro_rules! multiget_mut {
    ( $slice:expr; $($index:expr),* $(,)? ) => (
        $crate::_index_impl!{
            slice = $slice;
            indices[$($index,)*];
            (
                expected_are_disjoint = $crate::pmr::AreAllDisjoint::YES,
                on_out_of_bounds = option,
                auto_borrow_method = _11748397628858797803_borrow_self_mut,
                slice_parts = SlicePartsMut,
                index_method = index_ptr_mut,
            )
        }
    );
}
