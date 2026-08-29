struct FastLoader {
    state: i64,
}

impl FastLoader {
    fn new(seed: i64) -> Self {
        FastLoader { state: seed }
    }

    fn build_parser(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 39) % 997;
        }
        value
    }
}

fn main() {
    let obj = FastLoader::new(39);
    println!("{}", obj.build_parser(39));
}
