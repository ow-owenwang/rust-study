/*
Rc<T>：单线程下的多个拥有者（共享所有权）

应用场景：图结构、树结构中，多个节点指向同一子节点

特点
- 引用计数，自动 drop。
- 不可变共享。
- 只能用于单线程（不是线程安全）。
 */

use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

fn main() {
    let node1 = Rc::new(Node { value: 1, next: None });
    let node2 = Rc::new(Node { value: 2, next: Some(Rc::clone(&node1)) });
    let node3 = Rc::new(Node { value: 3, next: Some(Rc::clone(&node1)) }); // 多个共享 node1
}
