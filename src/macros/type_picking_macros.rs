macro_rules! make_type_picker {
    (
        for[$($gen_params:tt)*] struct $picker:ident[$($gen_args:tt)*];
        types = [$($ty:ty),* $(,)*];
    ) => (
        pub struct $picker<$($gen_params)* __Cond> (
            core::marker::PhantomData<fn()->(
                $(core::marker::PhantomData<$ty>,)*
                core::marker::PhantomData<__Cond>,
            )>
        );

        const _: () = {
            make_type_picker!(@inner
                for[$($gen_params)*]
                $picker[$($gen_args)*]
                [$($ty,)*]
                [
                    0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
                    16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
                    32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47
                    48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63
                    64
                ]
            );

        };
    );
    (@inner
        for[$($gen_params:tt)*]
        $picker:ident[$($gen_args:tt)*]
        [$ty:ty, $($rem_tys:tt)*]
        [$index:tt $($rem_idx:tt)*]
    )=>{
        impl<$($gen_params)*> crate::utils::AssocType
        for $picker<$($gen_args)* [(); $index]>
        {
            type Assoc = $ty;
        }

        make_type_picker!{@inner
            for[$($gen_params)*]
            $picker[$($gen_args)*]
            [$($rem_tys)*]
            [$($rem_idx)*]
        }
    };
    (@inner for[$($gen_params:tt)*] $picker:ident[$($gen_args:tt)*] [] [$($idx:tt)*] )=>{};
}

#[doc(hidden)]
#[macro_export]
macro_rules! pick_type {
    (
        $picker_l:ident $(:: $picker_t:ident)* $([$($gen_args:tt)*])?,
        $index:expr $(,)?
    ) => (
        <
            $picker_l$(::$picker_t)* < $($($gen_args)*)? [(); $index ]> as
            $crate::pmr::AssocType
        >::Assoc
    )
}
