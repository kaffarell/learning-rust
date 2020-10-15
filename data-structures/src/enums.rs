#[derive(Debug)]
enum Car {
    Tesla(String),
    Audi(bool),
}

pub fn run() {
    let test = Car::Tesla(String::from("Electric"));
    println!("{:?}", test);
}
