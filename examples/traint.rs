use std::ops::Add;

fn main() {
    let c1 = Box::new(Complex::new(1.0, 1f64));
    let c2 = Box::new(Complex::new(2_f64, 3.0));
    println!("{:?}", *c1 + *c2);
    use std::{fs::File, io::Write};
    let mut f = File::create("/tmp/test_write_trait").unwrap();
    let w: &mut dyn Write = &mut f;
    w.write_all(b"hello ").unwrap();
}
#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}
impl Complex {
    pub fn new(real: f64, imagine: f64) -> Complex {
        Self { real, imagine }
    }
}
impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Self::new(real, imagine)
    }
}
