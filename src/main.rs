#![allow(unused_variables)]
#![allow(dead_code)]

use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    println!("\nDining Philosophers\n");

    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let p1 = Philosopher::new("Baruch Spinoza", 0, 1);
    let p2 = Philosopher::new("Gilles Deleuze", 1, 2);
    let p3 = Philosopher::new("Karl Marx", 2, 3);
    let p4 = Philosopher::new("Friedrich Nietzsche", 3, 4);
    let p5 = Philosopher::new("Michel Foucault", 0, 4);
    let philosophers = vec![p1, p2, p3, p4, p5];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}

struct Table {
    forks: Vec<Mutex<()>>
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating...", self.name);
        thread::sleep_ms(100);
        println!("{} is finished eating.", self.name);
    }
}


