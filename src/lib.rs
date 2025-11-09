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
        let v = match self.boundary {
            0 => 0,
            _ => (self.value + 1) % self.boundary,
        };
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
        // This deals with the issue where someone creates a default Bint with a zero boundqry
        // triggering a divide by zero error.
        if self.boundary == 0 {
            return *self;
        }
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

impl Default for Bint {
    /// Defaults to the maximum value of an unsigned 8 integer.
    ///
    /// ```
    /// use bint::Bint;
    ///
    /// let mut b = Bint::default();
    ///
    /// for _ in 0..u8::MAX {
    ///     b = b.down()
    /// }
    ///
    /// for _ in 0..u8::MAX {
    ///     b = b.down()
    /// }
    ///
    /// assert_eq!(b.value, 0)
    /// ```
    fn default() -> Self {
        Bint {
            value: 0,
            boundary: u8::MAX,
        }
    }
}

impl fmt::Display for Bint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<BintCell> for Bint {
    /// ```
    /// use bint::{Bint, BintCell};
    ///
    /// let cell = BintCell::new_with_value(8, 3);
    /// let expected = Bint {
    ///     value: cell.value(),
    ///     boundary: cell.boundary,
    /// };
    ///
    /// assert_eq!(expected, Bint::from(cell));
    /// ```
    fn from(cell: BintCell) -> Self {
        Bint {
            value: cell.value(),
            boundary: cell.boundary,
        }
    }
}

impl From<&BintCell> for Bint {
    /// ```
    /// use bint::{Bint, BintCell};
    ///
    /// let cell = BintCell::new_with_value(8, 3);
    /// let expected = Bint {
    ///     value: cell.value(),
    ///     boundary: cell.boundary,
    /// };
    ///
    /// assert_eq!(expected, Bint::from(cell));
    /// ```
    fn from(cell: &BintCell) -> Self {
        Bint {
            value: cell.value(),
            boundary: cell.boundary,
        }
    }
}

impl From<DrainableBintCell> for Bint {
    /// ```
    /// use bint::{Bint, DrainableBintCell};
    ///
    /// let bint_cell = DrainableBintCell::new_with_value(8, 8, 3);
    /// let expected = Bint::new_with_value(8, 3);
    ///
    /// assert_eq!(expected, Bint::from(bint_cell));
    /// ```
    fn from(cell: DrainableBintCell) -> Self {
        Bint::from(cell.bint_cell)
    }
}

impl From<&DrainableBintCell> for Bint {
    /// ```
    /// use bint::{Bint, DrainableBintCell};
    ///
    /// let bint_cell = DrainableBintCell::new_with_value(8, 8, 3);
    /// let expected = Bint::new_with_value(8, 3);
    ///
    /// assert_eq!(expected, Bint::from(&bint_cell));
    /// ```
    fn from(cell: &DrainableBintCell) -> Self {
        Bint::from(cell.bint_cell.clone())
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
    /// assert_eq!(2, b.up());
    ///
    /// b.up();
    /// assert_eq!(4, b.up());
    /// ```
    pub fn up(&self) -> u8 {
        let bint = Bint {
            value: self.value(),
            boundary: self.boundary,
        }
        .up();
        self.cell.set(bint.value);
        bint.value
    }

    /// ```
    /// use bint::BintCell;
    ///
    /// let b = BintCell::new(6);
    ///
    /// b.up_x(3);
    /// assert_eq!(3, b.value());
    /// ```
    pub fn up_x(&self, x: u8) -> u8 {
        for _ in 0..x {
            self.up();
        }
        self.value()
    }

    /// ```
    /// use bint::BintCell;
    ///
    /// let b = BintCell::new(6);
    ///
    /// b.down();
    /// assert_eq!(4, b.down());
    ///
    /// b.down();
    /// assert_eq!(2, b.down());
    /// ```
    pub fn down(&self) -> u8 {
        let bint = Bint {
            value: self.value(),
            boundary: self.boundary,
        }
        .down();
        self.cell.set(bint.value);
        bint.value
    }

    /// ```
    /// use bint::BintCell;
    ///
    /// let b = BintCell::new(6);
    ///
    /// assert_eq!(4, b.down_x(2));
    /// ```
    pub fn down_x(&self, x: u8) -> u8 {
        for _ in 0..x {
            self.down();
        }
        self.value()
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

    /// Returns a Bint version x number of spots up. This is a utility method to simplify
    /// something that I needed on my poker library.
    ///
    /// ```
    /// use bint::{Bint, BintCell};
    ///
    /// let cell = BintCell::new_with_value(6, 3);
    /// let expected = Bint {
    ///     value: 0,
    ///     boundary: 6
    /// };
    ///
    /// assert_eq!(expected, cell.static_down_x(3));
    /// assert_eq!(expected, cell.static_down_x(9));
    /// ```
    pub fn static_down_x(&self, x: u8) -> Bint {
        Bint::from(self).down_x(x)
    }

    /// Returns a Bint version x number of spots up. This is a utility method to simplify
    /// something that I needed on my poker library.
    ///
    /// ```
    /// use bint::{Bint, BintCell};
    ///
    /// let cell = BintCell::new(6);
    /// let expected = Bint {
    ///     value: 3,
    ///     boundary: 6
    /// };
    ///
    /// let actual = cell.static_up_x(3);
    ///
    /// assert_eq!(expected, cell.static_up_x(3));
    /// assert_eq!(expected, cell.static_up_x(9));
    /// ```
    pub fn static_up_x(&self, x: u8) -> Bint {
        Bint::from(self).up_x(x)
    }

    #[must_use]
    pub fn value(&self) -> u8 {
        self.cell.get()
    }
}

impl Default for BintCell {
    /// Defaults to the maximum value of an unsigned 8 integer.
    ///
    /// ```
    /// use bint::BintCell;
    ///
    /// let b = BintCell::default();
    ///
    /// for _ in 0..u8::MAX {
    ///     b.up();
    /// }
    ///
    /// for _ in 0..u8::MAX {
    ///     b.down();
    /// }
    ///
    /// assert_eq!(b.value(), 0)
    /// ```
    fn default() -> Self {
        BintCell {
            cell: Cell::new(0),
            boundary: u8::MAX,
        }
    }
}

impl fmt::Display for BintCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl From<Bint> for BintCell {
    /// ```
    /// use bint::{Bint, BintCell};
    ///
    /// let bint = Bint::new_with_value(8, 3);
    /// let expected = BintCell::new_with_value(8, 3);
    ///
    /// assert_eq!(expected, BintCell::from(bint));
    /// ```
    fn from(cell: Bint) -> Self {
        BintCell::new_with_value(cell.boundary, cell.value)
    }
}

impl From<&Bint> for BintCell {
    /// ```
    /// use bint::{Bint, BintCell};
    ///
    /// let bint = Bint::new_with_value(8, 3);
    /// let expected = BintCell::new_with_value(8, 3);
    ///
    /// assert_eq!(expected, BintCell::from(&bint));
    /// ```
    fn from(cell: &Bint) -> Self {
        BintCell::new_with_value(cell.boundary, cell.value)
    }
}

impl From<DrainableBintCell> for BintCell {
    /// ```
    /// use bint::{BintCell, DrainableBintCell};
    ///
    /// let bint_cell = DrainableBintCell::new_with_value(8, 8, 3);
    /// let expected = BintCell::new_with_value(8, 3);
    ///
    /// assert_eq!(expected, BintCell::from(bint_cell));
    /// ```
    fn from(cell: DrainableBintCell) -> Self {
        cell.bint_cell
    }
}

impl From<&DrainableBintCell> for BintCell {
    /// ```
    /// use bint::{BintCell, DrainableBintCell};
    ///
    /// let bint_cell = DrainableBintCell::new_with_value(8, 8, 3);
    /// let expected = BintCell::new_with_value(8, 3);
    ///
    /// assert_eq!(expected, BintCell::from(&bint_cell));
    /// ```
    fn from(cell: &DrainableBintCell) -> Self {
        cell.bint_cell.clone()
    }
}

/// Version of a `BintCell` that can only be called a limited number of times, after which it
/// returns none.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DrainableBintCell {
    bint_cell: BintCell,
    pub capacity: Cell<usize>,
}

impl DrainableBintCell {
    #[must_use]
    pub fn new(boundary: u8, capacity: usize) -> DrainableBintCell {
        DrainableBintCell {
            bint_cell: BintCell::new(boundary),
            capacity: Cell::new(capacity),
        }
    }

    /// ```
    /// use bint::DrainableBintCell;
    ///
    /// let b = DrainableBintCell::new_with_value(4, 4, 3);
    ///
    /// assert_eq!(3, b.value());
    /// assert_eq!(2, b.down().unwrap());
    /// assert_eq!(1, b.down().unwrap());
    /// assert_eq!(0, b.down().unwrap());
    /// assert_eq!(3, b.down().unwrap());
    /// assert!(b.down().is_none());
    /// ```
    #[must_use]
    pub fn new_with_value(boundary: u8, capacity: usize, value: u8) -> DrainableBintCell {
        DrainableBintCell {
            bint_cell: BintCell::new_with_value(boundary, value),
            capacity: Cell::new(capacity),
        }
    }

    /// ```
    /// use bint::DrainableBintCell;
    ///
    /// let b = DrainableBintCell::new(4, 8);
    ///
    /// assert_eq!(3, b.down().unwrap());
    /// assert_eq!(2, b.down().unwrap());
    /// assert_eq!(1, b.down().unwrap());
    /// assert_eq!(0, b.down().unwrap());
    /// assert_eq!(3, b.down().unwrap());
    /// assert_eq!(2, b.down().unwrap());
    /// assert_eq!(1, b.down().unwrap());
    /// assert_eq!(0, b.down().unwrap());
    /// assert!(b.down().is_none());
    /// ```
    #[must_use]
    pub fn down(&self) -> Option<u8> {
        self.drain()?;
        Some(self.bint_cell.down())
    }

    /// ```
    /// use bint::DrainableBintCell;
    ///
    /// let b = DrainableBintCell::new(4, 4);
    ///
    /// assert_eq!(2, b.down_x(2).unwrap());
    /// assert_eq!(0, b.down_x(2).unwrap());
    /// assert!(b.down_x(2).is_none());
    /// ```
    #[must_use]
    pub fn down_x(&self, x: u8) -> Option<u8> {
        for _ in 0..x {
            self.down()?;
        }
        Some(self.value())
    }

    /// Removes one from the capacity.
    pub fn drain(&self) -> Option<usize> {
        self.capacity.set(self.capacity.get().checked_sub(1)?);
        Some(self.capacity.get())
    }

    /// ```
    /// use bint::DrainableBintCell;
    ///
    /// let b = DrainableBintCell::new(4, 4);
    ///
    /// assert_eq!(1, b.up().unwrap());
    /// assert_eq!(2, b.up().unwrap());
    /// assert_eq!(3, b.up().unwrap());
    /// assert_eq!(0, b.up().unwrap());
    /// assert!(b.down().is_none());
    /// ```
    #[must_use]
    pub fn up(&self) -> Option<u8> {
        self.drain()?;
        Some(self.bint_cell.up())
    }

    /// ```
    /// use bint::DrainableBintCell;
    ///
    /// let b = DrainableBintCell::new(4, 4);
    ///
    /// assert_eq!(3, b.up_x(3).unwrap());
    /// assert_eq!(0, b.up_x(1).unwrap());
    /// assert!(b.up_x(2).is_none());
    /// ```
    #[must_use]
    pub fn up_x(&self, x: u8) -> Option<u8> {
        for _ in 0..x {
            self.up()?;
        }
        Some(self.value())
    }

    #[must_use]
    pub fn value(&self) -> u8 {
        self.bint_cell.value()
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
    fn up() {
        let mut b = Bint::new(8);

        for _ in 0..16 {
            b = b.up();
        }

        assert_eq!(0, b.value);
    }

    #[test]
    fn up_default_defect() {
        let b = Bint::new(0);

        let c = b.up();

        assert_eq!(0, c.value);
    }

    #[test]
    fn down() {
        let mut b = Bint::new(8);

        for _ in 0..16 {
            b = b.down();
        }

        assert_eq!(0, b.value);
    }

    #[test]
    fn down_default_defect() {
        let b = Bint::new(0);

        let c = b.down();

        assert_eq!(0, c.value);
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
    fn cell_up_loop() {
        let b = BintCell::new(8);

        for _ in 0..16 {
            b.up();
        }

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
    fn cell_down_loop() {
        let b = BintCell::new(8);

        for _ in 0..16 {
            b.down();
        }

        assert_eq!(0, b.value());
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

    #[test]
    fn drain_down() {
        let b = DrainableBintCell::new(8, 8);

        assert_eq!(7, b.down().unwrap());
        assert_eq!(6, b.down().unwrap());
        assert_eq!(5, b.down().unwrap());
        assert_eq!(4, b.down().unwrap());
        assert_eq!(3, b.down().unwrap());
        assert_eq!(2, b.down().unwrap());
        assert_eq!(1, b.down().unwrap());
        assert_eq!(0, b.down().unwrap());
        assert!(b.up().is_none());
    }

    #[test]
    fn drain_drain() {
        let b = DrainableBintCell::new(8, 8);

        assert_eq!(7, b.drain().unwrap());
        assert_eq!(6, b.drain().unwrap());
        assert_eq!(5, b.drain().unwrap());
        assert_eq!(4, b.drain().unwrap());
        assert_eq!(3, b.drain().unwrap());
        assert_eq!(2, b.drain().unwrap());
        assert_eq!(1, b.drain().unwrap());
        assert_eq!(0, b.drain().unwrap());
        assert!(b.drain().is_none());
    }

    #[test]
    fn drain_up() {
        let b = DrainableBintCell::new(8, 8);

        assert_eq!(1, b.up().unwrap());
        assert_eq!(2, b.up().unwrap());
        assert_eq!(3, b.up().unwrap());
        assert_eq!(4, b.up().unwrap());
        assert_eq!(5, b.up().unwrap());
        assert_eq!(6, b.up().unwrap());
        assert_eq!(7, b.up().unwrap());
        assert_eq!(0, b.up().unwrap());
        assert!(b.up().is_none());
    }
}
