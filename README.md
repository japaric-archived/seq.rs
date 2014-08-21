Compared to `iter!`

```
$ ./bench.py 100
test iter_100 ... bench:      2525 ns/iter (+/- 26)
test seq_100  ... bench:      2262 ns/iter (+/- 15)

$ /memory.sh
> Compiling seq.rs
real    0m0.413s
user    0m0.377s
sys     0m0.033s

> Compiling iter.rs
real    0m5.742s
user    0m5.658s
sys     0m0.076s

> Size of seq binary
   text    data     bss     dec     hex filename
 697522   27468    5592  730582   b25d6 seq

> Size of iter binary
   text    data     bss     dec     hex filename
 759122   27468    5592  792182   c1676 iter

> valgrind ./seq
==11630== HEAP SUMMARY:
==11630==     in use at exit: 0 bytes in 0 blocks
==11630==   total heap usage: 118 allocs, 118 frees, 3,536 bytes allocated

> valgrind ./iter
==11631== HEAP SUMMARY:
==11631==     in use at exit: 0 bytes in 0 blocks
==11631==   total heap usage: 118 allocs, 118 frees, 3,536 bytes allocated
```

[![Build Status](https://travis-ci.org/japaric/seq.rs.svg?branch=master)](https://travis-ci.org/japaric/seq.rs)

Macro sugar to initialize almost any collection (`Vec`, `HashMap`, etc).

## [Documentation][docs]

## Example

``` rust
#![feature(phase)]

extern crate seq;
#[phase(plugin)]
extern crate seq_macros;

use std::collections::HashMap;

fn main() {
    let v: Vec<u8> = seq![1, 2, 3];

    let m: HashMap<char, String> = seq!{
      'a' => "apple".to_string(),
      'b' => "banana".to_string(),
      'c' => "coconut".to_string(),
    };

    println!("{}", v);
    println!("{}", m);
}
```

## License

seq.rs is dual licensed under the Apache 2.0 license and the MIT license.

See LICENSE-APACHE and LICENSE-MIT for more details.

[docs]: http://www.rust-ci.org/japaric/seq.rs/doc/seq/
