use std::io;

pub fn run() {
    let n: i32;
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading");
    
    n = input.trim().parse::<i32>().unwrap();

    if n % 2 != 0 {
        println!("Weird");
    }else if n % 2 == 0 {
        if n >= 2 && n <= 5 {
            println!("Not Weird");
        }else if n >= 6 && n <= 20 {
            println!("Weird");
        }else if n > 20 {
            println!("Not Weird");
        }
    }
}
