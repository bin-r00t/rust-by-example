use std::fmt;
#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {

    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?; // Notice: ? mark

        for (count, v) in vec.iter().enumerate() {
            if count !=0 { write!(f, ", ")?; } // Notice: ? mark
            write!(f, "{count}: {value}", count=count, value=v)?; // Notice: ? mark
        }

        write!(f, "]") // Notice: NO ? mark...
    }
}

fn main(){
    let v = List(vec![1,2,3,4,5]);
    println!("{}", v);
    println!("{:#?}", v);
}