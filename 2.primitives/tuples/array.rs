use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements.", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 100] = [0; 100];

    println!("The first element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes.", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice");
    analyze_slice(&xs);

    // slices can point to a section of an array
    // they are of the form [starting_index..ending_index]
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    // example of empty slice `&[]`;
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // same but more verbose
    assert_eq!(&empty_array, &[0; 0]);

    // Arrays can be safely accessed using '.get', which returns an `Option`,
    // this can be matched as shown below, or used with `.expect()` if you would like to have the program exit with the error message.
    // instead of happily continue.
    for i in 0..xs.len() + 1 {
        // OOPS, one element too far
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bounds indexing on array will cause a runtime error.
    // println!("{}", xs[5]); // compile error
    // println!("{}", xs.get(5).unwrap_or(&0)); 
    // println!("{}", xs[..][5]) // runtime error // $ RUST_BACKTRACE=full ./array
}
