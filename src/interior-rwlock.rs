use std::sync::RwLock;

mod counter;

use counter::Counter;

struct CounterService {
    counter: RwLock<Counter>,
}

impl CounterService {
    fn new() -> Self {
        CounterService {
            counter: RwLock::new(Counter::new()),
        }
    }

    fn get(&self) -> u64 {
        let counter = self.counter.read().expect("rw-lock poisoned");
        counter.get()
    }

    fn inc(&self, amount: u64) -> u64 {
        let mut counter = self.counter.write().expect("rw-lock poisoned");
        counter.inc(amount)
    }
}

fn main() {
    let counter = CounterService::new();
    println!("get(): {}", counter.get());
    println!("inc(1): {}", counter.inc(1));
    println!("inc(4): {}", counter.inc(4));
}
