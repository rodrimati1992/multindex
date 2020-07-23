#![allow(non_camel_case_types)]
#![cfg_attr(not(any(test, feature = "testing")), no_std)]

#[doc(hidden)]
pub extern crate core;

#[doc(hidden)]
#[macro_use]
pub mod macros;

#[doc(hidden)]
pub mod index_argument;

#[doc(hidden)]
pub mod index_properties;

#[doc(hidden)]
pub mod ptr_indexing;

#[doc(hidden)]
mod std_const_fns;

#[doc(hidden)]
pub mod uniqueness;

#[doc(hidden)]
pub mod utils;

#[doc(hidden)]
pub mod pmr {
    pub use crate::{
        index_argument::{
            IK_Index, IK_Range, IK_RangeFrom, IndexArgument, IndexKind, IndexKindPicker,
            IntoPrenormIndex, PrenormIndex,
        },
        index_properties::{IndexArgumentStats, IndexArgumentsAndStats, IndexProperties},
        ptr_indexing::{IndexPointer, Indexer, IndexerParams},
        uniqueness::AreAllUnique,
        utils::{panic_on_oob_max_index, AssocType, BorrowSelf, SliceParts, SlicePartsMut},
    };

    pub use core::option::Option::{self, None, Some};
}

#[cfg(all(test, not(feature = "testing")))]
compile_error! { "tests must be run with the \"testing\" feature" }

