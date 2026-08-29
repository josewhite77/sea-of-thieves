struct BatchSession {
    state: i64,
}

impl BatchSession {
    fn new(seed: i64) -> Self {
        BatchSession { state: seed }
    }

    fn load_manager(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 68) % 997;
        }
        acc
    }
}

fn main() {
    let obj = BatchSession::new(68);
    println!("{}", obj.load_manager(68));
}
