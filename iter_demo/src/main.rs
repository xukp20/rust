fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<_> = v.into_iter().filter(|x| {x > &2}).collect();
    println!("{:?}", v1);

    let counter = Counter::new();
    for i in counter {
        println!("{}", i);
    }

    let sum: u32 = Counter::new()
                    .zip(Counter::new().skip(1))
                    .map(|(a,b)| a * b)
                    .filter(|x| x%3 == 0)
                    .sum();

    println!("{}", sum);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
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