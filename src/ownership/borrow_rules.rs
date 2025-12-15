/*

借用规则：
- 任意时刻只能有一个可变引用，或任意多个不可变引用。不能同时有
- 引用不能悬空（no dangling pointers）→ 编译器通过生命周期检查确保安全。


 */

fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s; // ❌ 错误：不能同时有可变和不可变借用
    println!("{}, {}", r1, r2);
}

fn main() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2); // OK
}
