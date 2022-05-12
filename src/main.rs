// using enums in rust

use std::arch::x86_64::_SIDD_NEGATIVE_POLARITY;

#[derive(Debug)]
enum IpAddrType {
    V4(String),
    V6(String, u64),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
// --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}


// enumerator with different types
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // implement the call method
    }
}

fn main() {
    let four = IpAddrType::V4(String::from("127.0.0.1"));
    let six = IpAddrType::V6(String::from(""), 10);

    route(six);

    let m = Message::Write(String::from("my message"));
    m.call();

    println!("{:?}", four);

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let my_dime = Coin::Dime;

    println!("{:?}", value_in_cents(my_dime));

    let five = Some(5);
    let six = add_one(five);
    let seven: Option<u32> = add_one(five);

    let some_u8_value = 5u8;
    match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),

    }

    // using the if let statement in rust
    let value = Some(3);
    if let Some(i) = value { println!("{}", i); }
    else {
        print!("Value is not a valid some")
    };
}

fn route(ip_type: IpAddrType) {
    println!("Correct value type");
}


fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny !");
            1
        },
        Coin::Nickel => 5, 
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

fn add_one(value: Option<u32>) -> Option<u32> {
    match value {
        Option::None => None,
        Option::Some(v) => Some(v + 1),
    }
}