#![warn(clippy::pedantic)]

use std::cell::Cell;
use std::fmt;

/// Bint: A bounded integer.
///
/// Returns a struct that represents an unsigned integer and a boundary that represents when
/// the value will be reset to 0.
///
/// Usage:
///
/// ```
/// use bint::Bint;
///
/// let b: Bint = Bint {value: 5, boundary: 6 };
/// let c: Bint = b.up();
/// let d: Bint = c.up();
///
/// assert_eq!(5, b.value);
/// assert_eq!(0, c.value);
/// assert_eq!(1, d.value);
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bint {
    pub value: u8,
    pub boundary: u8,
}

impl Bint {
    #[must_use]
    pub fn new(boundary: u8) -> Bint {
        Bint { value: 0, boundary }
    }

    #[must_use]
    pub fn up(&self) -> Bint {
        let v = (self.value + 1) % self.boundary;
        Bint {
            value: v,
            boundary: self.boundary,
        }
    }

    #[must_use]
    pub fn down(&self) -> Bint {
        if self.value == 0 {
            return Bint {
                value: self.boundary - 1,
                boundary: self.boundary,
            };
        }
        let v = (self.value - 1) % self.boundary;
        Bint {
            value: v,
            boundary: self.boundary,
        }
    }
}

impl fmt::Display for Bint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// `BintCell`: A bounded integer captured in a [`Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html).
///
/// Allows for Bint functionality in a single entity.
///
/// Usage:
///
/// ```
/// use bint::BintCell;
/// let b = BintCell::new(6);
///
/// b.down();
/// assert_eq!(5, b.value());
///
/// b.up();
/// b.up();
/// b.up();
/// assert_eq!(2, b.value());
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BintCell {
    pub cell: Cell<u8>,
    pub boundary: u8,
}

impl BintCell {
    #[must_use]
    pub fn new(boundary: u8) -> BintCell {
        BintCell {
            cell: Cell::new(0),
            boundary,
        }
    }

    pub fn up(&self) {
        let bint = Bint {
            value: self.value(),
            boundary: self.boundary,
        }
        .up();
        self.cell.set(bint.value);
    }

    pub fn down(&self) {
        let bint = Bint {
            value: self.value(),
            boundary: self.boundary,
        }
        .down();
        self.cell.set(bint.value);
    }

    pub fn reset(&self) {
        self.cell.set(0);
    }

    #[must_use]
    pub fn value(&self) -> u8 {
        self.cell.get()
    }
}

impl fmt::Display for BintCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let b = Bint::new(6);
        assert_eq!(0, b.value);
        assert_eq!(6, b.boundary);
    }

    #[test]
    fn format() {
        let b: Bint = Bint {
            value: 4,
            boundary: 6,
        };
        assert_eq!("4", format!("{}", b));
    }

    #[test]
    fn init() {
        let b = Bint {
            value: 7,
            boundary: 10,
        };
        assert_eq!(7, b.value);
        assert_eq!(10, b.boundary);
    }

    #[test]
    fn up() {
        let b: Bint = Bint {
            value: 4,
            boundary: 6,
        };
        let b: Bint = b.up();
        assert_eq!(5, b.value);

        let b: Bint = b.up();
        assert_eq!(0, b.value);
    }

    #[test]
    fn down() {
        let b: Bint = Bint {
            value: 1,
            boundary: 6,
        };
        let b: Bint = b.down();
        assert_eq!(0, b.value);

        let b: Bint = b.down();
        assert_eq!(5, b.value);
    }

    #[test]
    fn up_bint_outside() {
        let b: Bint = Bint {
            value: 50,
            boundary: 10,
        };
        let b: Bint = b.up();
        assert_eq!(1, b.value);

        let b: Bint = b.down();
        let b: Bint = b.down();
        assert_eq!(9, b.value);
    }

    #[test]
    fn cell_new() {
        let b = BintCell::new(6);
        assert_eq!(0, b.value());
        assert_eq!(6, b.boundary);
    }

    #[test]
    fn cell_format() {
        let b: BintCell = BintCell {
            cell: Cell::new(4),
            boundary: 6,
        };
        assert_eq!("4", format!("{}", b));
    }

    #[test]
    fn cell_up() {
        let b: BintCell = BintCell {
            cell: Cell::new(4),
            boundary: 6,
        };
        b.up();
        assert_eq!(5, b.value());

        b.up();
        assert_eq!(0, b.value());
    }

    #[test]
    fn cell_down() {
        let b: BintCell = BintCell {
            cell: Cell::new(1),
            boundary: 6,
        };
        b.down();
        assert_eq!(0, b.value());

        b.down();
        assert_eq!(5, b.value());
    }

    #[test]
    fn cell_reset() {
        let b = BintCell::new(8);
        b.up();
        b.up();
        b.up();

        b.reset();

        assert_eq!(0, b.value());
    }
}
