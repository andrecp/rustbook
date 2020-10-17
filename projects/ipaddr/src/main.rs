enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// You can have data straight in the enum
enum IpAddrEnumWithData {
    V4(String),
    V6(String),
}

// Differnt enum kinds can store different data
enum IpAddrEnumWithDifferentData {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Can also implement methods in Enums
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("Hello, world!");

    let home = IpAddrEnumWithData::V4(String::from("127.0.0.1"));

    let loopback = IpAddrEnumWithData::V6(String::from("::1"));

    let home = IpAddrEnumWithDifferentData::V4(127, 0, 0, 1);

    let loopback = IpAddrEnumWithDifferentData::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn route(ip_kind: IpAddrKind) {}
