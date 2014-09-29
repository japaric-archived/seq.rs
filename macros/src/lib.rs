#![deny(warnings)]
#![feature(macro_rules)]

/// Count the number of arguments
// FIXME (rust-lang/rfcs#88) Remove this macro in favor of the `$#$($arg)` syntax
#[macro_export]
macro_rules! count_args {
    () => { 0 };
    ($x:expr) => { 1 };
    ($head:expr, $($tail:expr),+) => { 1 + count_args!($($tail),+) };
}

/// Create a new collection from this sequence
#[macro_export]
macro_rules! seq {
    // List style: seq![1, 2, 3]
    ($($x:expr),*) => ({
        let mut _temp = ::seq::Seq::with_capacity(count_args!($($x),*));

        $(::seq::Seq::add_elem(&mut _temp, $x);)*

        _temp
    });
    // Map style: seq!{"I" => 1, "II" => 2}
    ($($k:expr => $v:expr),*) => ({
        let mut _temp = ::seq::Seq::with_capacity(count_args!($(($k, $v)),*));

        $(::seq::Seq::add_elem(&mut _temp, ($k, $v));)*

        _temp
    });
    // Trailing commas <3
    ($($x:expr),+,) => { seq!($($x),+) };
    ($($k:expr => $v:expr),+,) => { seq!($($k => $v),+) };
}
