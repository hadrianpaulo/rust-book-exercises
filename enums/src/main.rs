enum IpAddKind {
    V4,
    V6,
}

struct IpAddrNaive {
    kind: IpAddKind,
    address: String,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Write(some_string) => println!("{}", some_string),
            _ => println!("This is a Message Enum."),
        }
    }
}

fn route(ip_kind: IpAddKind) {}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let four = IpAddKind::V4;
    let six = IpAddKind::V6;

    route(four);
    route(six);

    let home = IpAddrNaive {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrNaive {
        kind: IpAddKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let msg = Message::Write(String::from("hello"));

    msg.call();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = None;

    if let Some(3) = some_u8_value {
        println!("three")
    };

    let mut count = 0;

    let coin = Coin::Quarter(UsState::Alaska);

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}
