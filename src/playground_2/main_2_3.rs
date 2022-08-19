use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
    println!("Data: {:?}", slice);
}

pub fn main_2_3() {
    println!("\n====== main_2_3 ======");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    println!("array occupies {} bytes", mem::size_of_val(&arr));

    analyze_slice(&arr);
    analyze_slice(&arr[1 .. 4]); // [2, 3, 4]

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..arr.len() + 1 { // OOPS, one element too far
        match arr.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
        arr.get(i).expect("Oh no, something causes overflow");
    }
}

