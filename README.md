[![Build Status](https://github.com/electronicpanopticon/bint-rs/actions/workflows/CI.yaml/badge.svg)](https://github.com/electronicpanopticon/bint-rs/actions/workflows/CI.yaml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/electronicpanopticon/bint-rs/blob/main/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/bint.svg)](https://crates.io/crates/bint)

# bint-rs

Bounded Integer in Rust

## Usage

Original immutable Bint:

```
use bint::Bint;

let b: bint::Bint = bint::Bint {value: 5, boundary: 6 };
let c: Bint = b.up();
let d: Bint = c.up_x(2);

assert_eq!(5, b.value);
assert_eq!(0, c.value);
assert_eq!(2, d.value);
```

New and improved BintCell:

```
use bint::BintCell;

let b = BintCell::new(6);
assert_eq!(5, b.down());

b.up();
b.up();
b.up_x(2);
assert_eq!(3, b.value());
```

New DrainableBintCell that expires after a certain number of usages:

```
use bint::DrainableBintCell;

let b = DrainableBintCell::new(4, 4);

assert_eq!(1, b.up().unwrap());
assert_eq!(2, b.up().unwrap());
assert_eq!(3, b.up().unwrap());
assert_eq!(0, b.up().unwrap());
assert!(b.up().is_none());
```

## Other examples
* [Bounded Integer in Rust](https://github.com/programble/bounded-integer)
* [C++ bounded::integer library](http://doublewise.net/c++/bounded/)
