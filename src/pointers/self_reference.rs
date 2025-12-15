/*
自引用

自引用结构体指的是一个结构体中某个字段引用了自身的另一个字段。Rust 的所有权模型使得这种结构体在多数情况下是非法的，因为字段的地址在移动结构体实例时可能改变，导致悬垂引用。
 */

// ❌错误写法
struct SelfRef<'a> {
    data: String,
    reference: &'a str, // 引用了 data 的部分内容
}

fn main() {
    let s = String::from("hello");
    let r = SelfRef {
        reference: &s, // 编译器会拒绝，因为生命周期不明确
        data: s,
    };
}
