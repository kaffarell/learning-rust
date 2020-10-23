use std::io;

pub fn run() {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error when getting inputp");

    let n: i32 = input.trim().parse::<i32>().unwrap();
    for i in 1..11 {
        println!("{} x {} = {}", n, i, n * i);
    }
}