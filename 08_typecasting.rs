fn main() {
    let a:i32 = 10;
    let b:i64 = a.into();
    let b:i64 = a as i64 + 10;
    println!("{}", b)
}