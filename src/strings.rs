// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello");

    // for pushing single chars
    hello.push('W');

    // for pushing strings
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if string is empty
    println!("Is string empty: {}", hello.is_empty());

    // Replacing words
    println!("Replace: {}", hello.replace("World", "There"));
    
    println!("{}", hello);
}