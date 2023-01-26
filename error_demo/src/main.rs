use std::{io::ErrorKind, fs::File};


fn main() {
    let f = File::open("Hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(_e) => panic!("Error creating file"),
            },
            _other_error => panic!("Error opening file"),
        }
    };
}
