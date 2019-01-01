use std::io;

fn main() {

    let mut a=String::new();
    println!("Enter a number!");
    io::stdin().read_line(&mut a).expect("Failed");
    let a:i32 = a.trim().parse().expect("Failed");

    let result = if a% 2 ==0 && a < 0 {
        // .. doing something ..
        "Number is even and negative"
    } else if a %2 == 0 && a == 0 {
        // .. doing something ..
        "Number is even and zero"
    } else if a %2 == 0 && a > 0 {
        // .. doing something ..
        "Number is even and positive"
    } else if a < 0 {
        // .. doing something ..
        "Number is odd and negative"
    } else {
        // .. doing something ..
        "Number is odd and positive"
    };

    println!("{}", result);

}