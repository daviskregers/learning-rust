#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6
}

enum Fruits {
    Apple = 0,
    Mango = 10,
    Watermelon = 20,
}

#[derive(Debug)]
struct IPAddr {
    kind: IPAddrKind,
    address: String
}

#[derive(Debug)]
enum IP {
    V4(String),
    V6(String)
}

fn main() {
    let v4 = IPAddrKind::V4;
    route(v4);
    let v6 = IPAddrKind::V6;
    route(v6);

    let home = IPAddr {
        kind: IPAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IPAddr {
        kind: IPAddrKind::V6,
        address: String::from("::1")
    
    };

    println!("{:?}", home);
    println!("{:?}", loopback);

    let f = Fruits::Mango;
    println!("{:?}", f as i32);

    let v4 = IP::V4(String::from("127.0.0.1"));
    let v6 = IP::V6(String::from("::1"));

    println!("{:?}", v4);
    println!("{:?}", v6);

}

fn route(ip: IPAddrKind) {
    println!("{:?}", ip)
}