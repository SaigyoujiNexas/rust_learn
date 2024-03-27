use std::iter;

fn main() {
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    let x = 5;
    let (x, y, z) = (1, 2, 3);
    let (x, y) = (1, 2, 3, 4, 5, 6);
    let x = 5;
    match x {
        1..=5 => println!("x is {}", x),
        _ => println!("some thing else"),
    }
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("some numbers: {first}, {last}");
        }
    }
    if let [first, rest @ ..] = &numbers[..] {}
}

enum Message {
    Hello { id: i32 },
}
fn test() {
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 5..,
        } => println!("Found an id in range:{}", id_variable),
        _ => println!(""),
    };
}
