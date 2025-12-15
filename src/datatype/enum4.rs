#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Change(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        // match *self { // 与下面区别是什么？
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move({}, {})", x, y),
            Message::Write(s) => println!("Write(\"{}\")", s),
            Message::Change(a, b, c) => println!("Change({}, {}, {})", a, b, c),
            _ => println!("Other"),
        }
    }
}

fn main() {
    let msg = Message::Write(String::from("hello"));
    println!("{:?}", msg);

    msg.print();
}
