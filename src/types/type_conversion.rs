use std::convert::From;

/*
Rust 不像 C/C++ 那样允许隐式类型转换，所有转换都必须显式进行：
注意使用 as 有潜在的截断风险，尤其是整数之间的转换。


更“安全”且推荐的方式是通过标准库中的 trait：
- From<T> for U: 实现了从 T 到 U 的转换。
- Into<U> 是 From<T> 的反转实现：T: Into<U> 等价于 U: From<T>。
- TryFrom 和 TryInto 是带错误处理的版本，用于可能失败的转换。


 */

fn main() {
    let x: i32 = 42;
    let y: u8 = x as u8; // 显式转换
    println!("x is {}", x);
    println!("y is {}", y);

    let x1: i32 = 42;
    let y1 = u8::try_from(x1); // 使用 TryFrom 更安全
}
