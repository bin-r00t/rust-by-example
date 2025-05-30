use std::fmt::{self, format, Display};


fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (digit, flag) = pair;
    (flag, digit)
}

#[derive(Debug)]
struct Matrix(i32, i32, i32, i32);

impl fmt::Display for Matrix {
    // NOTE: 'f' is before ':'
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HELLO, {}", self.0)
    }

}


fn main(){
    let mtx = Matrix( 10, 20, 30, 40 );
    println!("test print fmt::Display trait: {}", mtx);
}

/*
fn main() {
    let pair = (1, true);
    println!("tuple {:?}", pair);

    // let too_long_tuple = (1,2,3,4,5,6,7,8,9,0,11,12,13);
    // println!("Too long tuple cannot be print: {}", too_long_tuple);

     let too_long_tuple = (1,2,3,4,5,6,7,8,9,0,11,12);
     println!("Too long tuple cannot be print: {:?}", too_long_tuple);
}
*/
