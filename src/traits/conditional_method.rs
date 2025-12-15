use std::fmt::Display;

/*
有条件的实现方法：

这种方式允许你为某个类型提供方法，但仅在类型参数满足某些 trait bound 的时候。
 */

// 只有当 T 实现了 Display 时，才实现某些方法。如果 T 没有实现 Display，那么 print() 方法就不可用。

struct Wrapper<T> {
    value: T,
}

// 有条件的方法实现
impl<T> Wrapper<T>
where
    T: Display,
{
    fn print(&self) {
        println!("Value: {}", self.value);
    }
}
