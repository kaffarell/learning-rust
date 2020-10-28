use std::io;

pub fn run() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");


    println!("Hello, World.");
    println!("{}", input);
}
