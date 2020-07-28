/*!

Macros for indexing slices/arrays with multiple compile-time indices/ranges.

These indexing macros check that the indices/ranges don't overlap,
erroring at compile-time if they do,
then return tuples of references to elements / arrays / slices,
based on each passed-in argument.

# Shared Macro docs

Because the indexing macros are all very similar in how they work,
they have [shared documentation] with all of what they share in common.

# Examples

### Array as fields

This example demonstrates how you can borrow multiple indices and ranges into an array.

```rust
use multindex::multindex_mut;

let mut field = (100..200).collect::<Vec<u32>>();

const FIELD_A: usize = 32;
const FIELD_B: usize = 40;
const FIELD_C: usize = 60;
const FIELD_E: usize = 62;

// The type annotation is for the reader, the compiler can infer the types.
let (field_a, field_b, field_c_to_e): (&mut u32, &mut u32, &mut [u32; 3]) =
    multindex_mut!(field; FIELD_A, FIELD_B, FIELD_C..=FIELD_E);

assert_eq!(field_a, &mut 132);
assert_eq!(field_b, &mut 140);
assert_eq!(field_c_to_e, &mut [160, 161, 162]);

*field_a = *field_b;
assert_eq!(*field_a, 140);

*field_a += field_c_to_e.iter().sum::<u32>();
assert_eq!(*field_a, 623);

```

### Parsing integers

This example demonstrates how you can fallibly get the first 4 bytes of a
slice as an array, and the remainder as a slice.

```rust
use multindex::multiget;

let mut slice = &[0, 0, 1, 10, 20][..];

assert_eq!(grab_u32(&mut slice), Some(266));
assert_eq!(slice, &[20]);

assert_eq!(grab_u32(&mut slice), None);
assert_eq!(slice, &[20]);


fn grab_u32(slice: &mut &[u8]) -> Option<u32> {
    let (u32_bytes, rem) = multiget!(*slice; ..4, ..)?;
    *slice = rem;
    Some(u32::from_be_bytes(*u32_bytes))
}

```

### Splitting an array

This example demonstrates how you can split an array into reference to smaller arrays.

```rust
use multindex::multindex;

const ROW_SIZE: usize = 5;

let array: [u16; ROW_SIZE * 4] = [
    1, 2, 3, 5, 8,
    13, 21, 34, 55, 89,
    144, 233, 377, 610, 987,
    1597, 2584, 4181, 6765, 10946,
];


type Row = [u16; ROW_SIZE];
// The type annotation is for the reader, the type can be inferred.
let (row0, row1, row2, row3): (&Row, &Row, &Row, &Row) =
    multindex!(array; ..ROW_SIZE, ..ROW_SIZE * 2, ..ROW_SIZE * 3, ..ROW_SIZE * 4);

assert_eq!(row0, &[1, 2, 3, 5, 8]);
assert_eq!(row1, &[13, 21, 34, 55, 89]);
assert_eq!(row2, &[144, 233, 377, 610, 987]);
assert_eq!(row3, &[1597, 2584, 4181, 6765, 10946]);

```

# Minimum Supported Rust Version

This crate requires at least Rust 1.46.0 .

It uses branching and looping at compile-time,
to check that the indices/ranges passed to the macros don't overlap
(required for macros that index mutably).

# no-std support

This crate is `#[no-std]`.
If newer versions add features that require std,
they'll be conditional on a "std" feature being enabled
(it won't be enabled by default).


[shared documentation]: ./indexing_macro_docs/index.html

*/

#![allow(non_camel_case_types)]
#![cfg_attr(not(any(test, feature = "testing")), no_std)]

#[doc(hidden)]
pub extern crate core;

pub mod indexing_macro_docs;

#[doc(hidden)]
#[macro_use]
pub mod macros;

#[doc(hidden)]
pub mod index_argument;

#[doc(hidden)]
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
