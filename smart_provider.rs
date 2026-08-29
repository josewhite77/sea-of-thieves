struct SecureEngine {
    state: i64,
}

impl SecureEngine {
    fn new(seed: i64) -> Self {
        SecureEngine { state: seed }
    }

    fn build_manager(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 85) % 997;
        }
        total
    }
}

fn main() {
    let obj = SecureEngine::new(85);
    println!("{}", obj.build_manager(85));
}
