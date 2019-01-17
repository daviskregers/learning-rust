fn main() {
    
    fn add_one_v1(x:u32) -> u32 { x + 1}
    let add_one_v2 = |x:u32| -> u32 { x + 1 };

    println!("{}", add_one_v1(5));
    println!("{}", add_one_v2(5));

    let x = 4;
    let equal=|z| z == x;

    println!("{}", equal(4));
    println!("{}", equal(5));

}