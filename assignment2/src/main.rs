struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
fn main() {
    let mut newcounter = Counter::new();
    println!("next count: {}", newcounter.next().unwrap());
    for num in newcounter {
        println!("{}", num);
    }
}
