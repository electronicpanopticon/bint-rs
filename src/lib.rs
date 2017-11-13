#[allow(dead_code)]
pub struct Bint {
    value: u8,
    boundary: u8,
}

#[allow(dead_code)]
impl Bint {
    fn up(&self) -> Bint {
        let v = (self.value + 1) % self.boundary;
        Bint {value: v, boundary: self.boundary}
    }

    fn down(&self) -> Bint {
        if self.value == 0 {
            return Bint {value: self.boundary - 1, boundary: self.boundary};
        }
        let v = (self.value - 1) % self.boundary;
        Bint {value: v, boundary: self.boundary}
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn init() {
        let b = Bint {value: 7, boundary: 10 };
        assert_eq!(7, b.value);
        assert_eq!(10, b.boundary);
    }

    #[test]
    fn up() {
        let b: Bint = Bint {value: 4, boundary: 6 };
        let b: Bint = b.up();
        assert_eq!(5, b.value);

        let b: Bint = b.up();
        assert_eq!(0, b.value);
    }

    #[test]
    fn down() {
        let b: Bint = Bint {value: 1, boundary: 6 };
        let b: Bint = b.down();
        assert_eq!(0, b.value);

        let b: Bint = b.down();
        assert_eq!(5, b.value);
    }
}
