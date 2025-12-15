/*

*/

use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    next: RefCell<Option<Rc<Node>>>,
    prev: RefCell<Option<Weak<Node>>>, // 用 Weak 避免循环引用
}

fn main() {
    let a = Rc::new(Node {
        value: 1,
        next: RefCell::new(None),
        prev: RefCell::new(None),
    });

    let b = Rc::new(Node {
        value: 2,
        next: RefCell::new(None),
        prev: RefCell::new(Some(Rc::downgrade(&a))),
    });

    *a.next.borrow_mut() = Some(b.clone());
}
