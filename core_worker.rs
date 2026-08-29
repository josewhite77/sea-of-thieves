struct SharedContext {
    state: i64,
}

impl SharedContext {
    fn new(seed: i64) -> Self {
        SharedContext { state: seed }
    }

    fn fetch_router(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 75) % 997;
        }
        value
    }
}

fn main() {
    let obj = SharedContext::new(75);
    println!("{}", obj.fetch_router(75));
}
