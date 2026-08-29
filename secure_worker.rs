struct SharedContext {
    state: i64,
}

impl SharedContext {
    fn new(seed: i64) -> Self {
        SharedContext { state: seed }
    }

    fn flush_registry(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 68) % 997;
        }
        count
    }
}

fn main() {
    let obj = SharedContext::new(68);
    println!("{}", obj.flush_registry(68));
}
