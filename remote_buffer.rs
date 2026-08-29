struct HybridParser {
    state: i64,
}

impl HybridParser {
    fn new(seed: i64) -> Self {
        HybridParser { state: seed }
    }

    fn decode_adapter(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 82) % 997;
        }
        result
    }
}

fn main() {
    let obj = HybridParser::new(82);
    println!("{}", obj.decode_adapter(82));
}
