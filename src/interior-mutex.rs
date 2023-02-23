use std::sync::Mutex;

mod counter;

use counter::Counter;

struct CounterService {
    counter: Mutex<Counter>,
}

impl CounterService {
    fn new() -> Self {
        CounterService {
            counter: Mutex::new(Counter::new()),
        }
    }

    fn get(&self) -> u64 {
        let counter = self.counter.lock().expect("mutex poisoned");
        counter.get()
    }

    fn inc(&self, amount: u64) -> u64 {
        let mut counter = self.counter.lock().expect("mutex poisoned");
        counter.inc(amount)
    }
}

fn main() {
    let counter = CounterService::new();
    println!("get(): {}", counter.get());
    println!("inc(1): {}", counter.inc(1));
    println!("inc(2): {}", counter.inc(4));
}
