/*
消息传递（Channel）

Rust 倾向使用 消息传递（message passing） 的方式来实现线程间通信，避免共享状态。


 */

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
