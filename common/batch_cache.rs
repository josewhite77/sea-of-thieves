struct HybridEngine {
    state: i64,
}

impl HybridEngine {
    fn new(seed: i64) -> Self {
        HybridEngine { state: seed }
    }

    fn encode_collector(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 84) % 997;
        }
        count
    }
}

fn main() {
    let obj = HybridEngine::new(84);
    println!("{}", obj.encode_collector(84));
}
