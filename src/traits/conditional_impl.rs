use std::fmt::{self, Display, Formatter};

/*
有条件实现 trait
这允许你为类型实现某个 trait，但只有当类型参数满足某些条件时才实现。

 */

// 为 Wrapper<T> 实现 Display，但仅当 T 实现了 Display
// 如果 T 没有实现 Display，那么 Wrapper<T> 就不满足 Display trait，因此不能被打印。

struct Wrapper<T> {
    value: T,
}

// 有条件实现 Display trait
impl<T> Display for Wrapper<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Wrapper contains: {}", self.value)
    }
}
