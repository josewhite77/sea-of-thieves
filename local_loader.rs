struct AsyncRegistry {
    state: i64,
}

impl AsyncRegistry {
    fn new(seed: i64) -> Self {
        AsyncRegistry { state: seed }
    }

    fn encode_provider(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 25) % 997;
        }
        value
    }
}

fn main() {
    let obj = AsyncRegistry::new(25);
    println!("{}", obj.encode_provider(25));
}
