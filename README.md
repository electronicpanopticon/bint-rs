[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/bint.svg)](https://crates.io/crates/bint)

# bint-rs

Bounded Integer in Rust

## Usage

```
extern crate bint;

let b: bint::Bint = bint::Bint {value: 5, boundary: 6 };
let c: Bint = b.up();
let d: Bint = c.up();

println!("{} {} {}", b, c, d); // Prints 5 0 1
```

## Other examples
* [Bounded Integer in Rust](https://github.com/programble/bounded-integer)
* [C++ bounded::integer library](http://doublewise.net/c++/bounded/)
