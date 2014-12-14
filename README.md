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

[docs]: http://japaric.github.io/seq.rs/seq/
