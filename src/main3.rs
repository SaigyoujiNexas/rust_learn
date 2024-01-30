use std::net::IpAddr;

fn main(){

    let home = IpAddrKind::V4("123".to_string());
    let cconfig_max: Option<u8> = Some(3u8);
    match cconfig_max {
        Some(max) => println!("The maxmum is configured"),
        _ => ()
    };

}

enum IpAddrKind {
    V4(String),
    V6(String),
    V8{
        a1: i32,
        a2: i32,
        a3: i32,
    }
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Dime => 1,
        Coin::Penny => 2,
        Coin::Nickel => 3,
        Coin::Quarter => 4,
    }
}

fn route(ip_kind: IpAddrKind){

}
