#![allow(non_camel_case_types)]
#![cfg_attr(not(any(test, feature = "testing")), no_std)]

#[doc(hidden)]
pub extern crate core;

#[doc(hidden)]
#[macro_use]
pub mod macros;

#[doc(hidden)]
pub mod index_argument;

pub mod error;

#[doc(hidden)]
pub mod index_properties;

#[doc(hidden)]
pub mod ptr_indexing;

#[doc(hidden)]
pub mod std_const_fns;

#[doc(hidden)]
pub mod are_disjoint;

#[doc(hidden)]
pub mod utils;

#[doc(hidden)]
#[cfg(feature = "testing")]
pub mod test_utils;

#[doc(hidden)]
#[cfg(feature = "testing")]
pub mod doc_based_tests;

#[doc(hidden)]
pub use error::Error;

#[doc(hidden)]
pub mod pmr {
    pub use crate::{
        are_disjoint::AreAllDisjoint,
        error::{ErrorPicker, ErrorTuple, NoErrorsFound},
        index_argument::{
            IK_Index, IK_Range, IK_RangeFrom, IndexArgument, IndexKind, IndexKindPicker,
            IntoPrenormIndex, PrenormIndex,
        },
        index_properties::{
            ComputedConstants, IndexArgumentStats, IndexArgumentsAndStats, IndexProperties,
        },
        ptr_indexing::{IndexPointer, Indexer, IndexerParams},
        std_const_fns::result_m::is_err,
        utils::{panic_on_oob_max_index, AssocType, BorrowSelf, SliceParts, SlicePartsMut},
    };

    pub use core::option::Option::{self, None, Some};
    pub use core::result::Result::Err;
}

#[cfg(all(test, not(feature = "testing")))]
compile_error! { "tests must be run with the \"testing\" feature" }
