fn main() {
    let num = Some(5);
    let text = Some("Hello");
    let not_yet:Option<i32> = None;

    println!("{:?} {:?}, {:?}", num, text, not_yet)
}