fn main() {
    
    let mut n =0;

    loop {
        if n < 5 {
            println!("{}", n);
            n += 1;
        } else {
            break;
        }
    }

    while n <= 10 {
        println!("{}", n);
        n += 1;
    }

    for n in 10 .. 21 {
        println!("{}", n);
    }

}
