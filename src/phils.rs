struct Philosopher{
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher{
            name: name.to_string(),
        }
    }
}

pub fn run() {
    let p1 = Philosopher::new("Judith something");
    let p2 = Philosopher::new("someone awsome");

    println!("{}", p1.name)
}