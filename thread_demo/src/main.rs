use std::{thread, time::Duration, sync::mpsc};

fn main() {

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("{} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    // move the tx to the thread
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    
}
