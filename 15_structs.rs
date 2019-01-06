#[derive(Debug)]
struct User {
    email: String,
    username: String,
    age: i8
}

fn build(email : String, username: String, age : i8) -> User {
    User {
        email: email,
        username: username,
        age: age
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

}

fn main() {
    
    let mut user = User {
        email: String::from("email@example.org"),
        username: String::from("example-username"),
        age: 21
    };
    println!("{:?}", &user);

    user.age = 23;
    println!("{:?}", &user);

    println!("{:?}", build(String::from("an email"),String::from("a username"),24));

    let user2 = User {
        email: String::from("email2@example.org"),
        ..user
    };
    println!("{:?}", &user2);

    let rect = Rectangle { width: 30, height: 50};
    let rect2 = Rectangle { width: 25, height: 35 };
    let rect3 = Rectangle { width: 45, height: 35 };
    println!("Rectangle area: {}", &rect.area());
    println!("Rectangle can hold: {}", &rect.can_hold(&rect2));
    println!("Rectangle can hold: {}", &rect.can_hold(&rect3));

    let rect4 = Rectangle::square(16);
    println!("Square: {:?}, area: {}", &rect4, &rect4.area());

}