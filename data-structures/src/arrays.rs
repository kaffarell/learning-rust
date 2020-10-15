// Arrays- Fixed List where elements are the same data types

use std::mem;

pub fn run() {
    // [type; length]
    // cannot add 4 numbers here, have to be 5
    let mut numbers: [i32; 4] =[1,2,3,4];
    // Print whole
    println!("{:?}", numbers);
    // Reassign value
    numbers[0] = 20;
    // Print single val
    println!("{}", numbers[0]);
    // Get length
    println!("Length {}", numbers.len());
    // Arrays are stack allocated
    println!("arrays occupies {} bytes", mem::size_of_val(&numbers));
    // Get slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);
}