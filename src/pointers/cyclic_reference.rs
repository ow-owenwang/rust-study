/*
循环引用

循环引用是指两个或多个值互相持有对方的引用，形成一个“环”，导致引用计数永远不为0，从而内存无法释放，形成内存泄漏（在使用 Rc 时尤其明显）。
 */

use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    value: i32,
    next: RefCell<Option<Rc<Node>>>,
}

// ❌错误做法
fn main() {
    let a = Rc::new(Node {
        value: 1,
        next: RefCell::new(None),
    });
    let b = Rc::new(Node {
        value: 2,
        next: RefCell::new(Some(a.clone())),
    });

    *a.next.borrow_mut() = Some(b.clone()); // 形成了一个环
}
