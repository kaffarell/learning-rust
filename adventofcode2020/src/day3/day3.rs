use std::fs;

pub fn run(){
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

    // Extend slope
    for i in 0..words.len() {
        words[i] = words[i][..].repeat(100).to_string();
    }

    let mut counter_max: u64 = 1;
    let steps_x: Vec<i32> = vec!(1, 3, 5, 7, 1);
    let steps_y: Vec<i32> = vec!(1, 1, 1, 1, 2);
    for a in 0..steps_x.len() {
        let mut counter_x: usize = 0;
        let mut counter_y: usize = 0;
        let mut counter_trees: u32 = 0;

        // Search for trees
        loop {
            if &words[counter_y][counter_x..counter_x+1] == "#" {
            counter_trees = counter_trees + 1; 
            }
            counter_x = counter_x + steps_x[a] as usize;
            counter_y = counter_y + steps_y[a] as usize;
            if counter_y >= words.len() {
                break;
            }
        }
        counter_max = counter_max * counter_trees as u64;
        println!("{}", counter_max);
    }
    println!("{}", counter_max);
}