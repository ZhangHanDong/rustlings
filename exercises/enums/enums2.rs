// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

// 

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(i32,i32,i32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}
// use Message::*; // 当 Message 变体非常多
fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

// 思考： 了解一下 enum 的 match，去看一下 rust by example 
// 
