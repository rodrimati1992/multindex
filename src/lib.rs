#![allow(non_camel_case_types)]
#![cfg_attr(not(any(test, feature = "testing")), no_std)]

#[doc(hidden)]
pub extern crate core;

#[doc(hidden)]
mod std_const_fns;

#[doc(hidden)]
pub mod utils;

#[cfg(all(test, not(feature = "testing")))]
compile_error! { "tests must be run with the \"testing\" feature" }

