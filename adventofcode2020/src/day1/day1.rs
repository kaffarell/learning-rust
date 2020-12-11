use std::fs;

pub fn run() {
    // Open input file
    let content = fs::read_to_string("input.txt")
        .expect("Something went wrong opening the file");
    
    // Process file content
    let mut words: Vec<i32> = Vec::new();
    let mut cache: String = String::new();
    for i in 0..content.len() {
        if &content[i..i+1] == "\n" {
            words.push(cache.parse::<i32>().expect("Error parsing String"));
            cache = String::from("");
        }else{
            cache = cache + &content[i..i+1];
        }
    }

    for i in 0..words.len() {
        for a in 0..words.len() {
            if words[i] + words[a] == 2020 {
                println!("{}", words[i] * words[a]);
            }
        }
    }
}