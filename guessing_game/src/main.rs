use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!(">");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("cannot read line");
    
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guess: {}", guess);
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("right");
                break;
            },
        }
    }
}
