// Variables hold primitive data or references to data
// Variables are immutable by default(can't re-assign it)
// Rust is a block scoped language

pub fn run() {
    let name = "cheran";

    //make variable mutable
    let mut age = 19;

    age = 20;
    println!("My name is {} and age {}", name, age);

    // When we define constant explicitly define it.
    const ID: i32 = 001;
    println!("ID: {}", ID);  

    // Assign multiple variables 
    let (myname, myage) = ("cheran", 19);
    
}