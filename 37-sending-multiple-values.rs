use std::sync::mpsc; // multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, reciever) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread"];

        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }

    });

    for received in reciever {
        println!("Got {}", received);
    }  

}