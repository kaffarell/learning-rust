use std::fs;

pub fn run() {
    // Open input file
    let mut content = fs::read_to_string("input.txt")
        .expect("Something went wrong opening the file");
    
    // Process file content
    let mut words: Vec<String> = Vec::new();
    let mut cache: String = String::new();
    content = content + "\n";
    for i in 0..content.len()-1 {
        if &content[i..i+2] == "\n\n" {
            words.push(cache);
            cache = String::from("");
        }else{
            cache = cache + &content[i..i+1];
        }
    }
    words.push(cache);

    // Substitude \n with " "
    for i in 0..words.len() {
        words[i] = words[i].replace("\n", " ");
    }

    let mut counter: i32 = 0;
    for i in 0..words.len() {
        if is_valid(&words, i, "byr") && is_valid(&words, i, "iyr") &&
           is_valid(&words, i, "eyr") && is_valid(&words, i, "hgt") &&
           is_valid(&words, i, "hcl") && is_valid(&words, i, "ecl") &&
           is_valid(&words, i, "pid") {

            counter = counter + 1; 
        }
    }

    println!("{}", counter);
}

fn is_valid(words: &Vec<String>, counter: usize, field: &str) -> bool{
    let i = words[counter].find(field);
    match i {
        Some(_) => {
            return true
        },
        None => return false,
    }
}