/*
数据共享与同步

使用 Arc（原子引用计数）+ Mutex（互斥锁）
多线程间共享数据，Rust 不允许多个线程直接访问非线程安全的数据。你通常会使用 Arc<Mutex<T>> 来在多个线程中共享并修改数据。
 */

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
