mod actor;

use actor::Handler;

struct Counter {
    value: u64,
}

impl Counter {
    fn new() -> Self {
        Counter { value: 0 }
    }
}

struct Get;

impl Handler<Get> for Counter {
    type Result = u64;

    fn handle(&mut self, _m: Get) -> Self::Result {
        self.value
    }
}

struct Inc(u64);

impl Handler<Inc> for Counter {
    type Result = u64;

    fn handle(&mut self, Inc(amount): Inc) -> Self::Result {
        self.value = self.value.saturating_add(amount);
        self.value
    }
}

#[tokio::main]
async fn main() {
    let counter = actor::spawn(Counter::new(), 20);

    println!("Get: {}", counter.send(Get).await);
    println!("Inc(1): {}", counter.send(Inc(1)).await);
    println!("Inc(4): {}", counter.send(Inc(4)).await);
}
