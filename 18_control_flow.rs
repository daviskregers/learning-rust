
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State)
}

#[derive(Debug)]
enum State {
    Alaska,
    Arizona
}

fn value_in_cents(c: Coin) {
    match c {
        Coin::Penny => {
            println!("This is a Penny and it's value is 1.");
        },
        Coin::Nickel => {
            println!("This is a Nickel and it's value is 5.");
        },
        Coin::Dime => {
            println!("This is a Dime and it's value is 10.");
        },
        Coin::Quarter(state) => {
            println!("This is a Quarter and it's value is 25. It comes from {:?}", state);
        }
    }
}

fn main() {
    
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(State::Alaska));
    value_in_cents(Coin::Quarter(State::Arizona));

}