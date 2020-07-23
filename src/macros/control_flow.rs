macro_rules! block {
    ( $lifetime:lifetime: $($code:tt)* ) => (
        $lifetime: loop{
            break{
                $($code)*
            };
        }
    )
}

/// A for loop over a half-open range (includes the start, excludes the end).
#[doc(hidden)]
#[macro_export]
macro_rules! for_range {
    ($pat:pat in $range:expr => $($body:tt)* ) => ({
        let mut range: $crate::core::ops::Range<_> = $range;

        while range.start < range.end {
            let $pat = range.start;
            $($body)*
            range.start += 1;
        }
    })
}
