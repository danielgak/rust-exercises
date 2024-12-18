#[allow(dead_code)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn _route(_ip_kind: IpAddr) {}

#[allow(dead_code)]
struct Ipv4Addr {}

#[allow(dead_code)]
struct Ipv6Addr {}

#[allow(dead_code)]
enum IpAddrStruct {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[allow(dead_code)]
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

pub fn main() {
    let _four = IpAddr::V4(127, 0, 0, 1);
    let _six = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = plus_one(Some(5 as i128));
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;
    let num = some_number.expect("there is a numeber for sure!");
    let sum = num + 128;
    println!("{sum}");

    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        // similar as match but less verbose
        println!("The maximum is configured to be {max}");
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i128>) -> Option<i128> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
