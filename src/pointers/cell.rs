/*
Cell<T>：用于实现 Copy 类型的内部可变性，无需 borrow()
应用场景：存储小的 Copy 类型并允许在不可变上下文中修改

特点
get() / set() 不需要 borrow。
只能用于实现 Copy 的类型（如 bool, i32, 等等）。
比 RefCell 更轻量，但限制更多。
不能用于多线程。
 */

use std::cell::Cell;

struct MyStruct {
    flag: Cell<bool>,
}

fn main() {
    let s = MyStruct { flag: Cell::new(false) };

    s.flag.set(true);
    println!("{}", s.flag.get());
}
