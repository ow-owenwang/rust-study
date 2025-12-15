/*
RefCell<T>：运行时借用检查，适用于单线程内部可变性

应用场景：当需要在不可变结构中修改数据，比如 UI 树 或 单元测试模拟对象

特点
编译时允许 &self 修改数据（动态 borrow check）。
多个可变/不可变借用冲突将导致 panic。
不能用于多线程。
 */

use std::cell::RefCell;

struct MyStruct {
    value: RefCell<i32>,
}

fn main() {
    let s = MyStruct { value: RefCell::new(10) };

    *s.value.borrow_mut() += 5;
    println!("{}", s.value.borrow());
}
