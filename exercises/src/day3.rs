use std::io;

pub fn run() {
    let n: u8;
    let mut input: String = String::from("");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed getting input");
    n = input.trim().parse::<u8>().unwrap();

    if n % 2 != 0 {
        println!("Weird");
    }else if n % 2 == 0 {
        if n >= 2 && n <= 5 {
            println!("Not Weird");
        }else if n >= 6 && n <= 20 {
            println!("Weird");
        }else if n >= 20 {
            println!("Not Weird")
        }
    }
}