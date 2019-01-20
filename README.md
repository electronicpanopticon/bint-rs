# bint-rs

[![Build Status](https://api.travis-ci.org/folkengine/bint-rs.svg?branch=master)](https://travis-ci.org/folkengine/bint-rs)

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
