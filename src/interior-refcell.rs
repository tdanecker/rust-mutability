use std::cell::RefCell;

mod counter;

use counter::Counter;

struct CounterService {
    counter: RefCell<Counter>,
}

impl CounterService {
    fn new() -> Self {
        CounterService {
            counter: RefCell::new(Counter::new()),
        }
    }

    fn get(&self) -> u64 {
        self.counter.borrow().get()
    }

    fn inc(&self, amount: u64) -> u64 {
        self.counter.borrow_mut().inc(amount)
    }
}

fn main() {
    let counter = CounterService::new();
    println!("get(): {}", counter.get());
    println!("inc(1): {}", counter.inc(1));
    println!("inc(4): {}", counter.inc(4));
}
