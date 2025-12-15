/*
Box 指针（Box<T>）：堆分配指针，适合 递归类型 或 大对象堆存储

- 指向堆上数据的智能指针。
- 所有权模型仍然适用，当 Box 被 drop，堆内存自动释放。


Rust 需要在编译期知道类型的大小。递归类型如链表，直接定义会导致无限大小，必须使用 Box<T> 来“打断递归”。

特点：
- 所有权明确，自动释放。
- 只读或可变时都适合。
- 不支持共享。
 */

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::*;

fn main() {
    let b = Box::new(42);
    println!("b = {}", b); // b = 42

    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{:?}", list);
}
