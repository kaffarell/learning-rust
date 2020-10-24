use std::io;

pub fn run() {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading");

    let n: i32 = input.trim().parse().unwrap();

    println!("{}", factorial(n));
}

fn factorial(n: i32) -> i32 {
    if n < 2 {
        return 1;
    }else {
        return n * factorial(n-1);
    }
}