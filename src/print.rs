pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // except for strings we have to use {}
    // Basic formatting
    println!("Number: {}",1);

    // Positional Arguments
    println!("{0} is from {1} and {0} like to {2}", "cheran" , "RJY", "code");

    // Named arguments
    println!("{name} like to play {game}", name="cheran", game="COD");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10+10);
}