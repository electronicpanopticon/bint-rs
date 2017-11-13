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
        let b: Bint = Bint {value: 0, boundary: 6 };
        let b: Bint = b.up();
        assert_eq!(1, b.value);
    }
}
