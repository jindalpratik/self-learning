enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum NewIpAddr {
    V4(String),
    V6(String),
}

enum NewNewIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum _Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("Hello, world!");

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    
    let home = NewIpAddr::V4(String::from("127.0.0.1"));

    let loopback = NewIpAddr::V6(String::from("::1"));

    let home = NewNewIpAddr::V4(127, 0, 0, 1);

    let loopback = NewNewIpAddr::V6(String::from("::1"));

    let somechar = Some('c);
    let somenum = Some(5);

    let absent_number: Option<i32> = None;

    
}
