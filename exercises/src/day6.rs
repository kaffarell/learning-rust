use std::io;

pub fn run() {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error when reading");
    
    let n: i32 = input.trim().parse::<i32>().unwrap();
    let mut strings_vec: Vec<String> = Vec::new();

    for i in 0..n {
        let mut string1: String = String::new();
        io::stdin()
            .read_line(&mut string1)
            .expect("Error when reading");
        strings_vec.push(string1.trim().to_string());
    }

    for i in 0..strings_vec.len() {
        for a in 0..strings_vec[i].len() {
            if (a as i32) % 2 == 0 {
                print!("{}", strings_vec[i].as_bytes()[a] as char);
            }
        }
        print!(" ");
        for a in 0..strings_vec[i].len() {
            if (a as i32) % 2 == 1 {
                print!("{}", strings_vec[i].as_bytes()[a] as char);
            }
        }
        println!("");
    }
}
