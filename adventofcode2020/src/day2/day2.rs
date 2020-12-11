use std::fs;

pub fn run() {
    // Open input file
    let mut content = fs::read_to_string("input.txt")
        .expect("Something went wrong opening the file");
    
    // Process file content
    let mut words: Vec<String> = Vec::new();
    let mut cache: String = String::new();
    content = content + "\n";
    for i in 0..content.len() {
        if &content[i..i+1] == "\n" {
            words.push(cache);
            cache = String::from("");
        }else{
            cache = cache + &content[i..i+1];
        }
    }

    // Part 1
    // Amount of valid passwords
    let mut counter: i32 = 0;
    for i in 0..words.len() {
        let min: i32;
        let max: i32;
        let letter: char;
        let pos_line: usize = words[i].find('-').unwrap();
        let pos_space: usize = words[i].find(' ').unwrap();
        let pos_points: usize = words[i].find(':').unwrap();
        min = words[i][0..pos_line].parse::<i32>()
            .expect("Error parsing");
        
        max = words[i][pos_line+1..pos_space].parse::<i32>()
            .expect("Error parsing");

        letter = words[i][pos_space+1..pos_points].parse::<char>()
            .expect("Error parsing");
       
        let password: String = words[i][pos_points+2..].to_string();
        
        if password.matches(letter).count() >= min as usize && password.matches(letter).count() <= max as usize {
            counter = counter + 1;
        }
    }

    println!("{}", counter);

    // Part 2
    let mut counter1: i32 = 0;
    for i in 0..words.len() {
        let min: usize;
        let max: usize;
        let letter: &str;
        let pos_line: usize = words[i].find('-').unwrap();
        let pos_space: usize = words[i].find(' ').unwrap();
        let pos_points: usize = words[i].find(':').unwrap();
        min = words[i][0..pos_line].parse::<usize>()
            .expect("Error parsing");
        
        max = words[i][pos_line+1..pos_space].parse::<usize>()
            .expect("Error parsing");

        letter = &words[i][pos_space+1..pos_points];       
        let password: String = words[i][pos_points+2..].to_string();
        
        let one = &password[..][min-1..min] == letter;
        let two = &password[..][max-1..max] == letter;

        if (one && !two) || (!one && two) {
            counter1 = counter1 + 1;
        }
    }

    println!("{}", counter1);
}