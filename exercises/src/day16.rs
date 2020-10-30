use std::io;

pub fn run() {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading string");
    
    let parsing_result = input.trim().parse::<i32>();
    if let Err(_e) = parsing_result {
        println!("Bad String");
    }else if let Ok(e) = parsing_result {
        println!("{}", e);
    }
}