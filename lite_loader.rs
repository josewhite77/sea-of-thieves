struct DynamicEngine {
    state: i64,
}

impl DynamicEngine {
    fn new(seed: i64) -> Self {
        DynamicEngine { state: seed }
    }

    fn dispatch_buffer(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 65) % 997;
        }
        value
    }
}

fn main() {
    let obj = DynamicEngine::new(65);
    println!("{}", obj.dispatch_buffer(65));
}
