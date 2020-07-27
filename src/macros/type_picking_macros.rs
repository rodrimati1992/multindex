macro_rules! make_type_picker {
    (
        for $gen_params:tt struct $picker:ident $gen_args:tt;
        values_to_types = [$( $variant:expr => $ty:ty ),* $(,)*];
    ) => (
        make_type_picker!{
            @declare_struct
            for $gen_params $picker $gen_args;
            types = [$($ty,)*];
        }

        $(
            make_type_picker!{
                @animpl
                for $gen_params $picker $gen_args;
                $variant => $ty
            }
        )*
    );
    (@declare_struct
        for[$($gen_params:tt)*] $picker:ident[$($gen_args:tt)*];
        types = [$( $ty:ty ),* $(,)*];
    )=>{
        pub struct $picker<$($gen_params)* __Cond> (
            core::marker::PhantomData<fn()->(
                $(core::marker::PhantomData<$ty>,)*
                core::marker::PhantomData<__Cond>,
            )>
        );
    };
    (@animpl
        for[$($gen_params:tt)*] $picker:ident[$($gen_args:tt)*];
        $variant:expr => $ty:ty
    )=>{
        impl<$($gen_params)*> crate::utils::AssocType
        for $picker<$($gen_args)* [(); $variant as usize]>
        {
            type Assoc = $ty;
        }
    };
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
