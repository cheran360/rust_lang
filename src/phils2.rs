struct Philosopher{
    name:String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self){
        println!("{} is done with eating.", self.name);
    }
}

pub fn run() {
    let philosophers = vec![
        Philosopher::new("spiderman"),
        Philosopher::new("superman"),
        Philosopher::new("batman")
    ];

    for p in &philosophers {
        p.eat();
    }
}