
// struct V4 {}
// struct V6 {}


enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrString {
    V4(String),
    V6(String),
}

enum IpAddrTyped {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum StandardIpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}



fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let m = Message::Write(String::from("hello"));
    m.call();


    //the option enum is the way of rust to implement
    // te null pointer
    let some_number = Some(5);
    let some_char = Some('e');

    //this will allow for some number to some times assume the null
    //value
    let absent_number: Option<i32> = None;
    

}

fn route(ip_kind: IpAddrKind) {}
