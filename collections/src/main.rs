use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

fn simple_vec() {
    // new vector
    let mut v: Vec<i32> = Vec::new();

    // use vec!
    let v1 = vec![1, 2, 3];

    // add item
    v.push(1);

    // index
    let third = &v1[2];
    println!("{}", third);

    // get
    match v1.get(2) {
        Some(third) => println!("{}", third),
        None => println!("Missing element"),
    }

    // go through
    for i in &v1 {
        println!("{}", i);
    }
}

fn vec_enum(){
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}

fn simple_string(){
    // create
    let mut s = "Hello".to_string();
    s = String::from("Hello World");

    // update
    s.push_str("111");
    s.push('l');            
    let s1 = String::from("HaHa");
    let s2 = s + &s1;

    println!("{}", s2);

}

fn simple_hashmap(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 5);

}

fn main() {
    
    // simple_vec();

    // vec_enum();

    // simple_string();

    simple_hashmap();
}
