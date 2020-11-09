use std::io;

trait AdvancedArithmetic {
    fn divisorSum(self, n: i32) -> i32;
}

struct Calculator {}

impl AdvancedArithmetic for Calculator {
    fn divisorSum(self, n: i32) -> i32 {
        let mut counter: i32 = 0;
        for i in 1..n+1 {
            if n % i == 0 {
                counter = counter + i;
            }
        }
        return counter;
    }
}

pub fn run() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading from input");
    
    let cal = Calculator{};
    println!("{}", cal.divisorSum(input.trim().parse::<i32>().unwrap()));    
}