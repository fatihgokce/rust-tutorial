use std::any::Any;

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrV2 {
    V4(String),
    V6(String),
}

enum IpAddrV3 {
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
        // method body would be defined here
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let home2 = IpAddrV2::V4(String::from("127.0.0.1"));
    //
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    let x: i8 = 5;
    let y = Some(5).unwrap();

    let sum = x + y;
    println!("dda {}", sum);
    let number = Some(7);
    if let Some(i)=number{
        println!("matched {}",i);
    }
}
