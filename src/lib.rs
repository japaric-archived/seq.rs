#![deny(warnings)]

use std::collections::{Bitv, HashMap, HashSet, SmallIntMap, TreeMap, TreeSet};
use std::hash::Hash;

/// A growable collection
pub trait Seq<T> {
    /// Creates a new collection with an initial capacity (if applicable)
    fn with_capacity(uint) -> Self;

    /// Adds a new element to the collection
    // FIXME (UFCS) This should be a normal method instead of a static method
    fn add_elem(&mut Self, T);
}

impl Seq<bool> for Bitv {
    fn with_capacity(_: uint) -> Bitv {
        // NB Bitv's `with_capacity` + `push` grows the collection beyond its initial capacity
        Bitv::new()
    }

    fn add_elem(v: &mut Bitv, elem: bool) {
        v.push(elem)
    }
}

impl<K, V> Seq<(K, V)> for HashMap<K, V> where K: Eq + Hash {
    fn with_capacity(n: uint) -> HashMap<K, V> {
        HashMap::with_capacity(n)
    }

    fn add_elem(m: &mut HashMap<K, V>, (key, value): (K, V)) {
        m.insert(key, value);
    }
}

impl<T> Seq<T> for HashSet<T> where T: Eq + Hash {
    fn with_capacity(n: uint) -> HashSet<T> {
        HashSet::with_capacity(n)
    }

    fn add_elem(s: &mut HashSet<T>, elem: T) {
        s.insert(elem);
    }
}

impl<T> Seq<(uint, T)> for SmallIntMap<T> {
    fn with_capacity(_: uint) -> SmallIntMap<T> {
        // XXX Ideally `n` should be the biggest key passed to the `seq!` macro
        SmallIntMap::new()
    }

    fn add_elem(m: &mut SmallIntMap<T>, (key, value): (uint, T)) {
        m.insert(key, value);
    }
}

impl<K, V> Seq<(K, V)> for TreeMap<K, V> where K: Ord {
    fn with_capacity(_: uint) -> TreeMap<K, V> {
        // NB There is not a `with_capacity` method in `TreeMap`
        TreeMap::new()
    }

    fn add_elem(m: &mut TreeMap<K, V>, (key, value): (K, V)) {
        m.insert(key, value);
    }
}

impl<T> Seq<T> for TreeSet<T> where T: Ord {
    fn with_capacity(_: uint) -> TreeSet<T> {
        // NB There is not a `with_capacity` method in `TreeSet`
        TreeSet::new()
    }

    fn add_elem(s: &mut TreeSet<T>, elem: T) {
        s.insert(elem);
    }
}

impl<T> Seq<T> for Vec<T> {
    fn with_capacity(n: uint) -> Vec<T> {
        Vec::with_capacity(n)
    }

    fn add_elem(v: &mut Vec<T>, elem: T) {
        v.push(elem)
    }
}
