#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

use std::fmt;

/// Bint: A bounded integer.
#[allow(dead_code)]
pub struct Bint {
    pub value: u8,
    pub boundary: u8,
}

#[allow(dead_code)]
impl Bint {
    /// Returns a struct that represents an unsigned integer and a boundary that represents when
    /// the value will be reset to 0.
    ///
    /// # Usage:
    /// ```
    /// extern crate bint;
    /// let b: bint::Bint = bint::Bint {value: 5, boundary: 6 };
    /// b.up();
    /// println!("{}", b ); // Prints 0
    /// ```

    pub fn new(boundary: u8) -> Bint {
        Bint {
            value: 0,
            boundary: boundary,
        }
    }

    pub fn up(&self) -> Bint {
        let v = (self.value + 1) % self.boundary;
        Bint {
            value: v,
            boundary: self.boundary,
        }
    }

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
}
