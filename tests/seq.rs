#![feature(phase)]

extern crate collect;
#[macro_use]
extern crate seq;

use std::collections::{Bitv, HashMap, HashSet, VecMap};
use collect::{TreeMap, TreeSet};

#[test]
fn bitv() {
    let mut v = Bitv::new();
    assert_eq!(v, seq![]);

    v.push(true);
    v.push(false);
    v.push(true);
    assert_eq!(v, seq![true, false, true]);
}

#[test]
fn hashmap() {
    let mut m = HashMap::new();
    assert_eq!(m, seq!{});

    m.insert('b', "banana".to_string());
    m.insert('c', "coconut".to_string());
    m.insert('a', "apple".to_string());
    assert_eq!(m, seq!{
        'a' => "apple".to_string(),
        'b' => "banana".to_string(),
        'c' => "coconut".to_string(),
    });
}

#[test]
fn hashset() {
    let mut s = HashSet::new();
    assert_eq!(s, seq!{});

    s.insert("three");
    s.insert("one");
    s.insert("two");
    assert_eq!(s, seq!["one", "two", "three"]);
}

#[test]
fn vecmap() {
    let mut m = VecMap::new();
    assert_eq!(m, seq!{});

    m.insert(2, "two");
    m.insert(3, "three");
    m.insert(5, "five");
    assert_eq!(m, seq!{
        2 => "two",
        3 => "three",
        5 => "five",
    });
}

#[test]
fn treemap() {
    let mut m = TreeMap::new();
    assert_eq!(m, seq!{});

    m.insert('b', "banana".to_string());
    m.insert('c', "coconut".to_string());
    m.insert('a', "apple".to_string());
    assert_eq!(m, seq!{
        'a' => "apple".to_string(),
        'b' => "banana".to_string(),
        'c' => "coconut".to_string(),
    });
}

#[test]
fn treeset() {
    let mut s = TreeSet::new();
    assert_eq!(s, seq!{});

    s.insert("three");
    s.insert("one");
    s.insert("two");
    assert_eq!(s, seq!["one", "two", "three"]);
}

#[test]
fn vec() {
    let mut v = Vec::new();
    let rhs: Vec<_> = seq![];
    assert_eq!(v, rhs);

    v.push(1u8);
    v.push(2);
    v.push(3);
    let rhs: Vec<_> = seq![1, 2, 3];
    assert_eq!(v, rhs);
}
