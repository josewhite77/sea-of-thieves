struct RemoteParser {
    state: i64,
}

impl RemoteParser {
    fn new(seed: i64) -> Self {
        RemoteParser { state: seed }
    }

    fn encode_router(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 10) % 997;
        }
        result
    }
}

fn main() {
    let obj = RemoteParser::new(10);
    println!("{}", obj.encode_router(10));
}
