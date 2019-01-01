fn main() {
    
    let a: (i32, bool, f64) = (220, true, 8.5);
    print_tuple(a);

    let a: [i32; 5] = [3;5];
    print_array(a);

}

fn print_tuple(x :  (i32, bool, f64)) {
    let (a, y, z) = x;
    println!("{}, {}, {}", a, y, z);
}

fn print_array(x: [i32;5]) {
    for n in x.iter() {
        println! ("{}", n);
    }
}