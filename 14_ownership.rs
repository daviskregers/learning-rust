fn main() {
    
    let mut s = String::from("hello");
    let s2 = s.clone();

    take(&mut s);

    println!("{}", s);
    println!("{}", s2);

    let x = 5;
    let y = x;
    println!("{}", x);
    println!("{}", y);

    let r1 = &s[0..5];
    let r2 = &s[0..=5];
    let r3 = &s[ .. 5];
    let r4 = &s[0 ..];
    let r5 = &s[..];

    println!("{}", r1);
    println!("{}", r2);
    println!("{}", r3);
    println!("{}", r4);
    println!("{}", r5);

}

fn take(s: &mut String)  {
    s.push_str(" world");
}