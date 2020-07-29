/*!
Documents what the indexing macros share in common.

# Limitations

The  macros from this crate only accept compile-time indices/ranges not
derived from generic parameters.

The non-generic limitation might be lifted in newer versions,
when const generics and compile-time function evaluation support improves.

# Parameters

The macros take arguments of this form:<br>
`macro_name!(slice; indexing_argument0, indexing_argument1, indexing_argument2, etcetera )`.

Indexing arguments can be any of:

- Integers:
An individual index, returning a reference to the element at that index.
<br>Eg: `0`

- Ranges:
Returns a reference to an array with the elements from the start of the range,
as long as the range.
<br>Eg: `1..2`, `3..=4`.

- Unbounded ranges:
Returns a slice if the range is unbounded at the end of the argument list.
Otherwise returns an reference to an array.
<br>Eg: `..2`, `..=2`, `2..`, `..`.

For an example of using every type of argument [look here](#every-arg-type-example)

# Errors

### Compile-time errors

The macros report errors at compile-time using type errors with types of this form:
`NameOfTheError__MeaningOfTheTypeParameter<[(); Integer ]>`.

`NameOfTheError` is the kind of error that happened.

`MeaningOfTheTypeParameter` is what the `Integer` in the type parameter means,
usually which parameter(s) triggered that error.

### Runtime errors

The macros don't catch all errors at compile-time.

They always check *at runtime* that the arguments are in bounds of the array or slice,
with one check.

Once const-generics are stable (and powerful enough to express it generically),
a new release could be made with macros that check that the arguments are in bounds of arrays,
at compile-time.

# The error types

These are the errors that this macro encodes as types

### `OverlappingIndexArguments__ArgumentsAre<[(); LeftArgument ], [(); RightArgument ]>`:

When an argument overlaps with another one,
if this was allowed to compile it would allow aliasing `&mut` references.

`LeftArgument` is which argument overlaps with another one, starting at 0.

`RightArgument` is which argument overlaps with `LeftArgument`, starting at 0.

Examples:

In `multindex!(slice; 10, 20, 10)`, `LeftArgument` is `0` `RightArgument` is `2`.<br>
In `multindex!(slice; 0, 4..10, 20, 6)`, `LeftArgument` is `1` `RightArgument` is `3`.<br>

### `NextStartIsUnbounded__CurrentArgumentIs<[(); WhichArgument ]>`:

When a range argument with an unbounded end is followed by a
range argument with an unbounded start.

`WhichArgument` is which argument triggers the error, starting at 0.

Examples:

In `multindex!(slice; 3.., ..5)`, `WhichArgument` is `0`<br>
In `multindex!(slice; 1, 3, 5.., ..)`, `WhichArgument` is `2`.

### `NextStartIsLessThanCurrent__CurrentArgumentIs<[(); WhichArgument ]>`:

When a range argument with an unbounded end is followed by an
argument that compares less to it.

`WhichArgument` is which argument triggers the error, starting at 0.

Examples:

In `multindex!(slice; 13.., 4)`, `WhichArgument` is `0`<br>
In `multindex!(slice; 1, 3, 10.., 8)`, `WhichArgument` is `2`.

### `InclusiveUptoUsizeMax__CurrentArgumentis<[(); WhichArgument ]>`:

When an inclusive range argument ends at `usize::MAX`.

This is an error to simplify the implementation of this crate.

`WhichArgument` is which argument triggers the error, starting at 0.

Examples:

In `multindex!(slice; ..=usize::MAX)`, `WhichArgument` is `0`<br>
In `multindex!(slice; 1, 3, 5..=usize::MAX)`, `WhichArgument` is `2`.

### `UsizeMaxIndex__CurrentArgumentis<[(); WhichArgument ]>`:

When an integer argument is `usize::MAX`.

This is an error to simplify the implementation of this crate.

`WhichArgument` is which argument triggers the error, starting at 0.

Examples:

In `multindex!(slice; usize::MAX)`, `WhichArgument` is `0`<br>
In `multindex!(slice; 1, 3, usize::MAX)`, `WhichArgument` is `2`.


# Examples

<span id="every-arg-type-example"></span>

### Every argument type

This example demonstrates every type of argument being used.

The type annotations in this example are for the reader,
all the types can be inferred.

```rust
use multindex::multindex_mut;

{
    // Index:      0  1  2   3   4   5   6   7    8
    let mut arr = [3, 5, 8, 13, 21, 34, 55, 89, 144];

    // Single argument invocations of the macro return a single element tuple.
    assert_eq!(multindex_mut!(arr; 0), (&mut 3,));

    assert_eq!(multindex_mut!(arr; 0, 2, 4 ), (&mut 3, &mut 8, &mut 21));
}

{
    // Index:      0  1  2   3   4   5   6   7    8
    let mut arr = [3, 5, 8, 13, 21, 34, 55, 89, 144];

    // Trailing `n..` ranges return slice references with the rest of the slice
    let trailing: (&mut [i32],) = multindex_mut!(arr; 6..);
    assert_eq!(trailing, (&mut [55, 89, 144][..],));

    // Initial/middle `n..` ranges return references to arrays
    let middle: (&mut [i32; 2], &mut i32) = multindex_mut!(arr; 1.., 3);
    assert_eq!(middle, (&mut [5, 8], &mut 13));
}

{
    // Index:      0  1  2   3   4   5   6   7    8
    let mut arr = [3, 5, 8, 13, 21, 34, 55, 89, 144];

    // Trailing `..` ranges return slice references with the rest of the slice
    let trailing: (&mut i32, &mut [i32],) = multindex_mut!(arr; 5, ..);
    assert_eq!(trailing, (&mut 34, &mut [55, 89, 144][..]));

    // Initial/middle `..` ranges return references to arrays.
    let middle: (&mut i32, &mut [i32; 3], &mut i32) = multindex_mut!(arr; 0, .., 4);
    assert_eq!(middle, (&mut 3, &mut [5, 8, 13], &mut 21));
}

{
    // Index:      0  1  2   3   4   5   6   7    8
    let mut arr = [3, 5, 8, 13, 21, 34, 55, 89, 144];

    // `n..m` / `n..=m` ranges return references to arrays
    let exclusive: (&mut [i32; 2],) = multindex_mut!(arr; 2..4);
    assert_eq!(exclusive, (&mut [8, 13],));

    let inclusive: (&mut [i32; 3],) = multindex_mut!(arr; 2..=4);
    assert_eq!(inclusive, (&mut [8, 13, 21],));
}

{
    // Index:      0  1  2   3   4   5   6   7    8
    let mut arr = [3, 5, 8, 13, 21, 34, 55, 89, 144];

    // `..n`/`..=n` ranges return references to arrays.
    let exclusive: (&mut i32, &mut [i32; 2]) = multindex_mut!(arr; 1, ..4);
    assert_eq!(exclusive, (&mut 5, &mut [8, 13]));

    let inclusive: (&mut [i32; 5],) = multindex_mut!(arr; ..=4);
    assert_eq!(inclusive, (&mut [3, 5, 8, 13, 21],));
}


```


*/
