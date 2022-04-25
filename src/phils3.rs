use std::thread;

struct Philosopher{
    name: String,
}

impl Philosopher{
    fn new(name: &str) -> Philosopher{
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is eating", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating", self.name);
    }
}

pub fn run(){
    let philosophers = vec![
        Philosopher::new("batman"),
        Philosopher::new("superman"),
        Philosopher::new("ironman")
    ];
    for p in philosophers{
        p.eat();
    }
}