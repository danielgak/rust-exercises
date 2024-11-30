enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn _route(_ip_kind: IpAddr) {}

fn main() {
    let _four = IpAddr::V4(127, 0, 0, 1);
    let _six = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5 as i128);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;

    let result = some_number + 5:
}

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddrStruct {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
