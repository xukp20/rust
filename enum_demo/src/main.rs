enum Coin{
    Penny, 
    Nickel,
    Dime,
    Quarter(State),
}

#[derive(Debug)]
enum State{
    A,
    B,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("quarter in {:#?}", state);
            25
        },
    }
}

fn main() {
    let c = Coin::Dime;
    println!("{}", value_in_cents(c));
    let q = Coin::Quarter(State::A);
    println!("{}", value_in_cents(q));
}