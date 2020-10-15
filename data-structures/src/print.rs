pub fn run() {
    // Print to console
    println!("Hello other file");

    // Basic formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Brad", "Mass");

    // Positional Formatting
    println!("{} is from {} and {0} likes to {2}", "Brad", "Mass", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "John", activity = "football");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}