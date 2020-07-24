/// Make sure not to use this inside of macros,
/// because the error message only appears if it's the top-most macro in the
/// macro call stack.
#[doc(hidden)]
#[macro_export]
macro_rules! option_expect {
    ($opt:expr, $message:expr) => {{
        let opt = $opt;
        {
            use $crate::std_const_fns::option_m;
            [$message][option_m::is_none(&opt) as usize];
            match opt {
                Some(x) => x,
                None => loop {},
            }
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! option_unwrap_or_else {
    ($opt:expr, $or_else:expr $(,)*) => {
        match $opt {
            $crate::pmr::Some(x) => x,
            $crate::pmr::None => $or_else,
        }
    };
}

#[allow(unused_macros)]
macro_rules! option_unwrap_or {
    ($opt:expr, $or:expr $(,)*) => {
        let or = $or;
        match $opt {
            Some(x) => x,
            None => $or,
        }
    };
}

/// A const-equivalent of `core::mem::replace` .
///
/// Unnecessary once mutable references are usable in a const context.
#[doc(hidden)]
#[macro_export]
macro_rules! mem_replace {
    ($place:expr, $with:expr) => {{
        let curr = $place;
        $place = $with;
        curr
    }};
}
