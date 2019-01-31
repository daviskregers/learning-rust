use std::sync::mpsc; // multiple producer, single consumer
use std::thread;

fn main() {
    let (sender, reciever) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        sender.send(val).unwrap();
    });

    let rec = reciever.recv().unwrap();
    println!("Got {}", rec);

}