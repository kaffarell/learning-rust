// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - User when you modify string

pub fn run() {
    // Primitive String
    let hello = "helo";
    // String
    let mut hello1 = String::from("hello ");

    // Get length
    println!("Lenght: {}, {}", hello.len(), hello1.len());

    // push char
    hello1.push('W');
    // push string
    hello1.push_str("orld");

    // memory in bytes
    println!("Capacity {}", hello1.capacity());

    // Check if empty
    println!("Is Empty {}", hello1.is_empty());

    // Contains substring
    println!("Contains 'World' {}", hello1.contains("World"));
    // Replace
    println!("replace: {}", hello1.replace("World", "There"));
    // Loop trough string by whitespace
    for word in hello1.split_whitespace() {
        println!("{}", word);
    }

    // Create String with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}, {}", hello, hello1);
}