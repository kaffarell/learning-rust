use std::io;

pub fn run() {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading");

    let n: i32 = input.trim().parse().unwrap();
    let binar_str: String = format!("{:b}", n);
    let mut amount_1: i32 = 0;
    let mut amount_1_max: i32 = 0;
    for i in 0..binar_str.len() {
        if binar_str.as_bytes()[i] == '1' as u8 {
            amount_1 += 1;
            if amount_1 > amount_1_max {
                amount_1_max = amount_1;
            }
        }else{
            amount_1 = 0;
        }
    }

    println!("{}", amount_1_max);
}