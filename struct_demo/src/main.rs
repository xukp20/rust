#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.length * self.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, length: size }
    }
}

fn main() {
    let rect = &Rectangle { width: 20, length: 50 };
    println!("{}", rect.area());

    println!("{:#?}", rect);

    let s = Rectangle::square(10);
    println!("{:#?}", s);
}


