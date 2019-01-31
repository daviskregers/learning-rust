use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (s, r) = mpsc::channel();
    let s1 = mpsc::Sender::clone(&s);

    thread::spawn( move || {
        let vals = vec!["hi", "from", "the", "thread"];

        for val in vals {
            s.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }

    });

    thread::spawn( move || {
        let vals = vec!["more", "messages", "for", "you"];

        for val in vals {
            s1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }

    });

    for rec in r {
        println!("{}", rec);
    }

}