struct StreamBuilder {
    state: i64,
}

impl StreamBuilder {
    fn new(seed: i64) -> Self {
        StreamBuilder { state: seed }
    }

    fn dispatch_session(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 38) % 997;
        }
        value
    }
}

fn main() {
    let obj = StreamBuilder::new(38);
    println!("{}", obj.dispatch_session(38));
}
