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
            expected_uniqueness = $expected_uniqueness:expr,
            on_out_of_bounds = $on_out_of_bounds:tt,
            auto_borrow_method = $auto_borrow_method:ident,
            slice_parts = $slice_parts:ident,
            index_method = $index_method:ident,
        )
    )=>({
        const __IND_AAS:
            &$crate::pmr::IndexArgumentsAndStats<
                [$crate::pmr::IndexArgument; $index_arg_count]
            >
        = {
            let iaas = $crate::new_IndexArgumentsAndStats!($($index,)*);
            let props = $crate::pmr::IndexProperties::new(&iaas, $expected_uniqueness);

            // This errors if the expected uniqueness of the indices differs
            // from the actual uniqueness.
            //
            // Passing `AreAllUnique::No` to IndexProperties's constructor
            // skips the uniqueness checks, always returning `AreAllUnique::No`.
            props.uniqueness.assert_equals($expected_uniqueness);

            &{iaas}
        };

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
            
            if __IND_AAS.stats.max_bounded_end > slice.len() {
                $crate::_on_out_of_bounds!(
                    $on_out_of_bounds,
                    ind_stats = __IND_AAS.stats,
                    slice = slice
                )
            } else {
                // `lifetime` is a `PhantomData<&'a (mut) T>` used to ensure that the
                // reference returned by `IndexPointer::index_ptr_*` has the correct lifetime.
                let $slice_parts{ptr, len, lifetime} = $slice_parts::new({slice});

                let ret = ($(
                    {
                        const __IND_ARG: &IndexArgument = &__IND_AAS.ind_args[$count];

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
                expected_uniqueness = $crate::pmr::AreAllUnique::No,
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
                expected_uniqueness = $crate::pmr::AreAllUnique::Yes,
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
                expected_uniqueness = $crate::pmr::AreAllUnique::No,
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
                expected_uniqueness = $crate::pmr::AreAllUnique::Yes,
                on_out_of_bounds = option,
                auto_borrow_method = _11748397628858797803_borrow_self_mut,
                slice_parts = SlicePartsMut,
                index_method = index_ptr_mut,
            )
        }
    );
}
