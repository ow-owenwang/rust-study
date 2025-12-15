/*
非线程安全的数据是指不能在多个线程之间安全共享或修改的内存数据。如果你在多个线程中同时访问（特别是修改）这类数据，而没有同步机制，就可能出现数据竞争（Data Race），造成程序崩溃或行为异常。
Rust 的编译器在编译时会静态检查所有权和借用关系，防止任何可能导致数据竞争的代码通过编译。


Rust 如何判断类型是否线程安全？
通过两个 trait 来决定：
1. Send —— 能否在线程间转移所有权
如果一个类型是 Send，那么它的所有权可以从一个线程传到另一个线程。
举例：i32, Vec<T>, Box<T> 都是 Send，但 Rc<T> 不是。
2. Sync —— 能否在多个线程中共享引用
如果一个类型是 Sync，那么它可以在多个线程中安全地通过 &T 被共享。
举例：i32, Arc<T>, Mutex<T> 是 Sync，但 RefCell<T> 不是。
编译器在编译时自动推导类型是否实现了 Send 和 Sync，并且会在不满足线程安全要求时报错。
 */

use std::thread;

fn main() {
    /*
    data 是在主线程中创建的 Vec<i32>。
    thread::spawn 启动的闭包中试图访问和修改这个 Vec。
    但 Vec<T> 默认不是 Sync + Send，它是“非线程安全的数据”，不能直接被多个线程共享访问。
     */
    let mut data = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        data.push(4);
        println!("{:?}", data);
    });

    println!("{:?}", data);

    handle.join().unwrap();
}
