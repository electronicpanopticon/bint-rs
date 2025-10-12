#![warn(clippy::pedantic)]
#![allow(clippy::needless_doctest_main)]
#![cfg_attr(doc, doc = include_str!("../README.md"))]

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
/// let b = Bint {value: 5, boundary: 6 };
/// let c: Bint = b.up();
/// let d: Bint = c.up();
///
/// assert_eq!(5, b.value);
/// assert_eq!(0, c.value);
/// assert_eq!(1, d.value);
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bint {
    pub value: u8,
    pub boundary: u8,
}

impl Bint {
    /// ```
    /// use bint::Bint;
    ///
    /// let b: Bint = Bint::new(6);
    /// let c: Bint = b.down();
    /// let d: Bint = c.up();
    /// let e: Bint = d.up();
    ///
    /// assert_eq!(0, b.value);
    /// assert_eq!(5, c.value);
    /// assert_eq!(0, d.value);
    /// assert_eq!(1, e.value);
    /// ```
    #[must_use]
    pub fn new(boundary: u8) -> Bint {
        Bint { value: 0, boundary }
    }

    /// ```
    /// use bint::Bint;
    ///
    /// let bint = Bint::new_with_value(10, 7);
    /// assert_eq!(10, bint.boundary);
    /// assert_eq!(7, bint.value);
    ///
    /// let bint_out_of_range = Bint::new_with_value(10, 23);
    /// assert_eq!(10, bint_out_of_range.boundary);
    /// assert_eq!(0, bint_out_of_range.value);
    /// ```
    #[must_use]
    pub fn new_with_value(boundary: u8, value: u8) -> Bint {
        if value >= boundary {
            Bint::new(boundary)
        } else {
            Bint { value, boundary }
        }
    }

    /// ```
    /// use bint::Bint;
    ///
    /// let b: Bint = Bint {
    ///     value: 4,
    ///     boundary: 6,
    /// };
    ///
    /// let b: Bint = b.up();
    /// assert_eq!(5, b.value);
    ///
    /// let b: Bint = b.up();
    /// assert_eq!(0, b.value);
    /// ```
    #[must_use]
    pub fn up(&self) -> Bint {
        let v = (self.value + 1) % self.boundary;
        Bint {
            value: v,
            boundary: self.boundary,
        }
    }

    /// ```
    /// use bint::Bint;
    ///
    /// let b: Bint = Bint {
    ///     value: 4,
    ///     boundary: 6,
    /// };
    ///
    /// let b: Bint = b.up_x(3);
    /// assert_eq!(1, b.value);
    /// ```
    #[must_use]
    pub fn up_x(self, x: u8) -> Bint {
        let mut up = self;
        for _ in 0..x {
            up = up.up();
        }
        up
    }

    /// ```
    /// use bint::Bint;
    ///
    /// let b: Bint = Bint {
    ///     value: 1,
    ///     boundary: 6,
    /// };
    ///
    /// let b: Bint = b.down();
    /// assert_eq!(0, b.value);
    ///
    /// let b: Bint = b.down();
    /// assert_eq!(5, b.value);
    /// ```
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

    /// ```
    /// use bint::Bint;
    ///
    /// let b: Bint = Bint {
    ///     value: 4,
    ///     boundary: 6,
    /// };
    ///
    /// let b: Bint = b.down_x(6);
    /// assert_eq!(4, b.value);
    ///
    /// let b: Bint = b.down_x(3);
    /// assert_eq!(1, b.value);
    /// ```
    #[must_use]
    pub fn down_x(self, x: u8) -> Bint {
        let mut down = self;
        for _ in 0..x {
            down = down.down();
        }
        down
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
///
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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BintCell {
    pub cell: Cell<u8>,
    pub boundary: u8,
}

impl BintCell {
    /// ```
    /// use bint::BintCell;
    ///
    /// let b = BintCell::new(6);
    /// assert_eq!(0, b.value());
    /// assert_eq!(6, b.boundary);
    /// ```
    #[must_use]
    pub fn new(boundary: u8) -> BintCell {
        BintCell {
            cell: Cell::new(0),
            boundary,
        }
    }

    /// ```
    /// use bint::BintCell;
    ///
    /// let b = BintCell::new_with_value(6, 6);
    /// assert_eq!(0, b.value());
    /// assert_eq!(6, b.boundary);
    ///
    /// let b = BintCell::new_with_value(6, 3);
    /// assert_eq!(3, b.value());
    /// assert_eq!(6, b.boundary);
    /// ```
    #[must_use]
    pub fn new_with_value(boundary: u8, value: u8) -> BintCell {
        if value >= boundary {
            BintCell::new(boundary)
        } else {
            BintCell {
                cell: Cell::new(value),
                boundary,
            }
        }
    }

    /// ```
    /// use bint::BintCell;
    ///
    /// let b = BintCell::new(6);
    ///
    /// b.up();
    /// assert_eq!(1, b.value());
    ///
    /// b.up();
    /// assert_eq!(2, b.value());
    /// ```
    pub fn up(&self) {
        let bint = Bint {
            value: self.value(),
            boundary: self.boundary,
        }
        .up();
        self.cell.set(bint.value);
    }

    /// ```
    /// use bint::BintCell;
    ///
    /// let b = BintCell::new(6);
    ///
    /// b.up_x(3);
    /// assert_eq!(3, b.value());
    /// ```
    pub fn up_x(&self, x: u8) {
        for _ in 0..x {
            self.up();
        }
    }

    /// ```
    /// use bint::BintCell;
    ///
    /// let b = BintCell::new(6);
    ///
    /// b.down();
    /// assert_eq!(5, b.value());
    ///
    /// b.down();
    /// assert_eq!(4, b.value());
    /// ```
    pub fn down(&self) {
        let bint = Bint {
            value: self.value(),
            boundary: self.boundary,
        }
        .down();
        self.cell.set(bint.value);
    }

    /// ```
    /// use bint::BintCell;
    ///
    /// let b = BintCell::new(6);
    ///
    /// b.down_x(2);
    /// assert_eq!(4, b.value());
    /// ```
    pub fn down_x(&self, x: u8) {
        for _ in 0..x {
            self.down();
        }
    }

    /// ```
    /// use bint::BintCell;
    ///
    /// let b = BintCell::new_with_value(8, 5);
    /// b.reset();
    ///
    /// assert_eq!(0, b.value());
    /// ```
    pub fn reset(&self) {
        self.set(0);
    }

    /// ```
    /// use bint::BintCell;
    ///
    /// let b = BintCell::new(8);
    /// b.set(5);
    ///
    /// assert_eq!(5, b.value());
    /// ```
    pub fn set(&self, value: u8) {
        self.cell.set(value);
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
        assert_eq!(
            Bint::new(6),
            Bint {
                value: 0,
                boundary: 6
            }
        );
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
