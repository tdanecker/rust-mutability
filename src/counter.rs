#[derive(Debug)]
pub struct Counter {
    value: u64,
}

impl Counter {
    pub fn new() -> Self {
        Counter { value: 0 }
    }

    pub fn get(&self) -> u64 {
        self.value
    }

    pub fn inc(&mut self, amount: u64) -> u64 {
        self.value = self.value.saturating_add(amount);
        self.value
    }
}
