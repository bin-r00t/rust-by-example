use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let a = self.0;
        let b = self.1;
        let c = self.2;
        let d = self.3;
        write!(f, "( {}, {} )\n", a, b)?;
        write!(f, "( {}, {} )", c, d)
    }
}

fn transpose(m: Matrix) -> Matrix {
    let a = m.0;
    let b = m.1;
    let c = m.2;
    let d = m.3;

    Matrix(a, c, b, d)
}

fn main() {
    let mtx = Matrix(3.1, 1.1, 2.1, 3.0);
    println!("Matrix:\n{}", mtx);
    println!("Transpose:\n{}", transpose(mtx));
}