#[macro_use]
mod construction_macros;

#[macro_use]
mod control_flow;

#[macro_use]
mod indexing_macros;

#[macro_use]
mod std_functions;

#[macro_use]
mod type_picking_macros;

////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
#[macro_export]
macro_rules! _ignore {
    ($($anything:tt)*) => {};
}

////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
#[macro_export]
macro_rules! _ignore_then_unit {
    ($($anything:tt)*) => {
        ()
    };
}

////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
#[macro_export]
macro_rules! index_argument_to_kind_type {
    ($expr:expr) => {
        $crate::pick_type!(
            $crate::pmr::IndexKindPicker,
            $crate::pmr::IndexArgument::index_kind($expr) as usize,
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! error_tuple_to_error_type {
    ($expr:expr) => {
        $crate::pick_type!(
            $crate::pmr::ErrorPicker[ [(); $expr.first ], [(); $expr.second], ],
            $expr.kind as usize,
        )
    };
}
