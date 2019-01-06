pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_module() {
                println!("Nested module called");
            }
        }
    }
}

#[derive(Debug)]
enum TrafficLight{
    Red, Yellow, Green
}

use TrafficLight::{Red,Yellow};
use a::series::of::nested_module;

fn main() {
    nested_module();
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
    println!("{:?} {:?} {:?}", red, yellow, green)
}