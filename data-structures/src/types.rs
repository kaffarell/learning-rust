/*
 Primitive Types:
 Integers: u8, i8. u16, i16, u32, i32, u64, i64, u128. i128
 (Number of bits they take in memory, u = unsigned (no negative), most commonly used: i32)
 floats: f32, f64
 Boolean: bool
 Characters: char
 TUples,
 Arrays
*/

pub fn run() {
    // Default is i23
    let _x = 1;
    // Default is f64
    let _y = 2.5;
    // Add explicit
    let _z: i64 = 45454545;
    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    let is_greater: bool = 10 > 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (_x, _y, _z, is_active, is_greater, a1, face));
}