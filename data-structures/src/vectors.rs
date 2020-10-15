// Vectors
use std::mem;

pub fn run() {
    // [type; length]
    // cannot add 4 numbers here, have to be 5
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    // Print whole
    println!("{:?}", numbers);

    // Reassign value
    numbers[0] = 20;

    // Add to vector
    numbers.push(5);
    numbers.push(6);

    // Pop last value
    numbers.pop();

    // Print single val
    println!("{}", numbers[0]);

    // Get length
    println!("Length {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);

    // Loop trough vector values
    for x in numbers.iter() {
        println!("Numbers: {}", x)
    }

    // Loop and mutate trough vector values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers);
}