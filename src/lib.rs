#![deny(warnings)]

extern crate collect;

use std::collections::{Bitv, HashMap, HashSet, VecMap};
use std::hash::Hash;
use collect::{TreeMap, TreeSet};

/// Count the number of arguments
// FIXME (rust-lang/rfcs#88) Remove this macro in favor of the `$#$($arg)` syntax
#[macro_export]
macro_rules! seq_count_args {
    () => { 0 };
    ($x:expr) => { 1 };
    ($head:expr, $($tail:expr),+) => { 1 + seq_count_args!($($tail),+) };
}

/// Create a new collection from this sequence
#[macro_export]
macro_rules! seq {
    // List style: seq![1, 2, 3]
    ($($x:expr),*) => ({
        let mut _temp = $crate::Seq::with_capacity(seq_count_args!($($x),*));

        $($crate::Seq::add_elem(&mut _temp, $x);)*

        _temp
    });
    // Map style: seq!{"I" => 1, "II" => 2}
    ($($k:expr => $v:expr),*) => ({
        let mut _temp = $crate::Seq::with_capacity(seq_count_args!($(($k, $v)),*));

        $($crate::Seq::add_elem(&mut _temp, ($k, $v));)*

        _temp
    });
    // Trailing commas <3
    ($($x:expr),+,) => { seq!($($x),+) };
    ($($k:expr => $v:expr),+,) => { seq!($($k => $v),+) };
}

/// A growable collection
pub trait Seq<T> {
    /// Creates a new collection with an initial capacity (if applicable)
    fn with_capacity(uint) -> Self;

    /// Adds a new element to the collection
    fn add_elem(&mut self, T);
}

impl Seq<bool> for Bitv {
    fn with_capacity(_: uint) -> Bitv {
        // NB Bitv's `with_capacity` + `push` grows the collection beyond its initial capacity
        Bitv::new()
    }

    fn add_elem(&mut self, elem: bool) {
        self.push(elem)
    }
}

impl<K, V> Seq<(K, V)> for HashMap<K, V> where K: Eq + Hash {
    fn with_capacity(n: uint) -> HashMap<K, V> {
        HashMap::with_capacity(n)
    }

    fn add_elem(&mut self, (key, value): (K, V)) {
        self.insert(key, value);
    }
}

impl<T> Seq<T> for HashSet<T> where T: Eq + Hash {
    fn with_capacity(n: uint) -> HashSet<T> {
        HashSet::with_capacity(n)
    }

    fn add_elem(&mut self, elem: T) {
        self.insert(elem);
    }
}

impl<T> Seq<(uint, T)> for VecMap<T> {
    fn with_capacity(_: uint) -> VecMap<T> {
        // XXX Ideally `n` should be the biggest key passed to the `seq!` macro
        VecMap::new()
    }

    fn add_elem(&mut self, (key, value): (uint, T)) {
        self.insert(key, value);
    }
}

impl<K, V> Seq<(K, V)> for TreeMap<K, V> where K: Ord {
    fn with_capacity(_: uint) -> TreeMap<K, V> {
        // NB There is not a `with_capacity` method in `TreeMap`
        TreeMap::new()
    }

    fn add_elem(&mut self, (key, value): (K, V)) {
        self.insert(key, value);
    }
}

impl<T> Seq<T> for TreeSet<T> where T: Ord {
    fn with_capacity(_: uint) -> TreeSet<T> {
        // NB There is not a `with_capacity` method in `TreeSet`
        TreeSet::new()
    }

    fn add_elem(&mut self, elem: T) {
        self.insert(elem);
    }
}

impl<T> Seq<T> for Vec<T> {
    fn with_capacity(n: uint) -> Vec<T> {
        Vec::with_capacity(n)
    }

    fn add_elem(&mut self, elem: T) {
        self.push(elem)
    }
}
