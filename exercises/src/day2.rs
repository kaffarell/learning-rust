use std::io;

pub fn run() {
    let meal_cost: f64; 
    let mut meal_tip: f32; 
    let mut meal_tax: f32; 
    let mut input: String = String::from("");
    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading string");

    meal_cost = input.trim().parse::<f64>().unwrap();
    input.clear();

    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading string");

    meal_tip = input.trim().parse::<f32>().unwrap();
    input.clear();

    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading string");

    meal_tax = input.trim().parse::<f32>().unwrap();

    meal_tip = meal_cost as f32 * (meal_tip / 100.0);
    meal_tax = meal_cost as f32 * (meal_tax / 100.0);
    println!("{}", (meal_tip + meal_tax + meal_cost as f32) as i32);
}