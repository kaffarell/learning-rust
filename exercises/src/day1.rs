use std::io;

pub fn run() {
    let i: i32;
    let d: f64;
    let s: String;
    let mut input: String = String::from("");

    let int: i32 = 4;
    let double: f64 = 4.0;

    io::stdin()
        .read_line(&mut input)
        .expect("Erro while reading");
    // trim() removes leading and trailing whitespaces
    // The return value of parse() is a Result type ->
    // The Result type can be Ok(returned value) or Err(error code)
    // unwrap() gets the value out of an Ok() type
    i = input.trim().parse::<i32>().unwrap();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro while reading");
    println!("\"{}\"", input.trim());
    d = input.trim().parse::<f64>().unwrap();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro while reading");
    s = input;

    println!("{}", i+int);
    println!("{}", d+double);
    let mut result_string: String = String::from("HackerRank");
    result_string.push_str(&s[..]);
    println!("{}", result_string);
}