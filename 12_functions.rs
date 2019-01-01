
fn sub(a:i32, b:i32) -> i32 {
        return a - b
}

fn sub_add(a:i32, b:i32) -> (i32, i32) {
    return (a-b, a+b)
}

fn factorial(a : i32) -> i32 {
    let mut fact = 1;
    for n in 1 .. a+1 {
        fact *= n;
    }
    return fact;
}

fn main() {

    fn add(a:i32, b:i32) -> i32 {
        return a +b
    }

    println!("{}", add(1,2));
    println!("{}", sub(1,2));
    println!("{:?}", sub_add(1,2));
    println!("{}", factorial(6));

}