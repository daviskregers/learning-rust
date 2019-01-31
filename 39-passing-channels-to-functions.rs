use std::thread;
use std::sync::mpsc;

fn main() {
    
    let (s,r) = mpsc::channel();
    let handle = thread::spawn(|| {
        run(s);
        run1(r);
    });
    handle.join().unwrap();
}

fn run (s: mpsc::Sender<i32>) {
    s.send(2).unwrap();
    s.send(3).unwrap();
}

fn run1(r : mpsc::Receiver<i32>) {
    let rc = r.recv().unwrap();
    println!("{}", rc);
    println!("{}", r.recv().unwrap());
}