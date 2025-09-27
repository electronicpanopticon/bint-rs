use bint::Bint;

fn perms(i: u8) -> (u8, u8) {
    (i % 6, i % 10)
}

fn main() {
    let mut bint = Bint::new(30);
    for _ in 0..60 {
        let (x, y) = perms(bint.value);
        bint = bint.up();
        println!("{x} {y}");
    }
}
