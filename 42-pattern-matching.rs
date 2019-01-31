fn main() {

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // match

    match is_tuesday {
        true => println!("Using purple as background color"),
        false => println!("Using orange as background color"),
        _ => println!("Using black as background color")
    };

    // if let

    if let Some(color) = favorite_color {
        println!("Using your favorite color {:?}", favorite_color)
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as background color");
        } else {
            println!("Using orange as background color");
        }
    } else {
        println!("Using blue as background color");
    };

    // while let

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for let

    let v = vec!["a", "b", "c"];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index)
    }

    // Assignment

    let (x, y, z) = (1,2,3);
    println!("{} {} {}", x,y,z);

    // sytax

    let x = 1;

    match x {
        1|2 => println!("hey!"),
        3...5 => println!("hey?"),
        _ => println!("bye ... ")
    }

    // Ref and Ref mut

    let mut name = Some(String::from("Bob"))    ;

    match name {
        Some(ref mut name) => *name = String::from("John"),
        None => (),
    };

    println!("{:?}", name);

}