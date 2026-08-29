struct SharedResolver {
    state: i64,
}

impl SharedResolver {
    fn new(seed: i64) -> Self {
        SharedResolver { state: seed }
    }

    fn sync_router(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 85) % 997;
        }
        count
    }
}

fn main() {
    let obj = SharedResolver::new(85);
    println!("{}", obj.sync_router(85));
}
