use std::collections::HashMap;
use std::io;

pub fn run() {
    let mut phone_book: HashMap<String, String> = HashMap::new();
    
    let mut input: String = String::new();    

    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading");

    let input_queries = input.trim().parse::<i32>().unwrap();

    for _i in 0..input_queries {
        let mut input_query: String = String::new();
        io::stdin()
            .read_line(&mut input_query)
            .expect("Error reading");
        
        let tuple_query = get_values(input_query);
        phone_book.insert(tuple_query.0, tuple_query.1);
    }

    let mut output_vector: Vec<String> = Vec::new();
    loop {
        let mut input_search: String = String::new();

        io::stdin()
            .read_line(&mut input_search)
            .expect("Error while reading");

        if input_search.trim() == "" {
            break;
        }
        let result = phone_book.get(input_search.trim());
        if result == None {
            output_vector.push("Not found".to_string());
        }else{
            output_vector.push(format!("{}={}", input_search.trim(), result.unwrap()));
        }
    }
    for i in 0..output_vector.len() {
        println!("{}", output_vector[i]);
    }
}

fn get_values(string: String) -> (String, String) {
    let mut n: (String, String) = (String::new(), String::new());
    let mut i: usize = 0;
    loop {
        if string.as_bytes()[i] == ' ' as u8 {
            break;
        }
        i += 1;
    }
    n.0 = string[0..i].trim().to_string(); 
    n.1 = string[i..string.len()].trim().to_string(); 
    return n;
}