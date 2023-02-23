mod counter;

use counter::Counter;

fn main() {
    let mut counter = Counter::new();
    println!("get(): {}", counter.get());
    println!("inc(1): {}", counter.inc(1));
    println!("inc(4): {}", counter.inc(4));
}
