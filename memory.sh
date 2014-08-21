#!/bin/bash

cargo build --release
echo '> Compiling seq.rs'
time rustc -O -L target/release -L target/release/deps examples/seq.rs
echo '> Compiling iter.rs'
time rustc -O -L target/release -L target/release/deps examples/iter.rs

echo '> Size of seq binary'
size --format=berkeley seq
echo '> Size of iter binary'
size --format=berkeley iter

echo '> valgrind ./seq'
valgrind ./seq
echo '> valgrind ./iter'
valgrind ./iter
