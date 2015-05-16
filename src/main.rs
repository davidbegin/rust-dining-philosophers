#![allow(unused_variables)]
#![allow(dead_code)]

use std::thread;

fn main() {
    println!("\nDining Philosophers\n");

    let p1 = Philosopher::new("Baruch Spinoza");
    let p2 = Philosopher::new("Gilles Deleuze");
    let p3 = Philosopher::new("Karl Marx");
    let p4 = Philosopher::new("Friedrich Nietzsche");
    let p5 = Philosopher::new("Michel Foucault");
    let philosophers = vec![p1, p2, p3, p4, p5];

    for philo in &philosophers {
        philo.eat();
    }
}

struct Philosopher {
    name: String
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is eating...", self.name);
        thread::sleep_ms(100);
        println!("{} is finished eating.", self.name);
    }
}


