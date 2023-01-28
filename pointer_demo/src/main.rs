
fn hello(name: &str) {
    println!("Hello, {}", name);
}

struct Dropper {
    data: String,
}

impl Drop for Dropper {
    fn drop(&mut self) {
        println!("Dropping dropper with {}", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let m = Box::new(String::from("Rust"));

    hello(&m);

    hello("Rust");

    let _dropper = Dropper {
        data: String::from("abc"),
    };
}
