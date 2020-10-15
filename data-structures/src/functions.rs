pub fn run() {
    greeting("hello", "james");
    let get_sum: i32 = add(5, 5);
    println!("Sum {}", get_sum);

    // Closure
    // Inline function which can use local functions
    let add_nums = |n1: i32, n2: i32| n1 + n2 + get_sum;
    println!("C Sum {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}