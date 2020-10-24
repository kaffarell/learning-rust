use std::io;

pub fn run() {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
        
    let length: i32 = input.trim().parse::<i32>().unwrap();

    let mut array: Vec<char> = Vec::new();

    input.clear();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    
    string_to_array(&mut array, length, input);

    for i in (0..length).rev() {
        if i == length {
            break;
        }
        print!("{} ", array[i as usize]);
    }
    println!("");

}

fn string_to_array(array: &mut Vec<char>, length: i32, string: String) {
    let mut i: usize = 0;
    loop {
        if string.as_bytes()[i] != ' ' as u8 {
           array.push(string.as_bytes()[i] as char); 
        }
        if (length + 2) as usize == i {
            break;
        }
        i += 1;
    }
}