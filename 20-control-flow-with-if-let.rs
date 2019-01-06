fn do_something(value: Option<u8>) {
    if let Some(3) = value {
        println!("Three")
    } else if let Some(4) = value {
        println!("Four")
    } else {
        println!("Other value")
    }
}

fn main() {
    let some_u8 = Some(3);
    let other_u8 = Some(4);
    let yet_another = Some(5);
    
    do_something(some_u8);
    do_something(other_u8);
    do_something(yet_another);

}