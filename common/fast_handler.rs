struct RemoteFactory {
    state: i64,
}

impl RemoteFactory {
    fn new(seed: i64) -> Self {
        RemoteFactory { state: seed }
    }

    fn sync_monitor(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 94) % 997;
        }
        result
    }
}

fn main() {
    let obj = RemoteFactory::new(94);
    println!("{}", obj.sync_monitor(94));
}
