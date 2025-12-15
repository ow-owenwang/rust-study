/*
引用必须总是有效（不能悬垂）
Rust 不允许悬垂引用（即指向已经被释放的内存）。
 */

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // ❌ 返回悬垂引用
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s // 所有权转移
}


fn main() {
    // let r = dangle();
    // println!("{}", r);

    let r1 = no_dangle();
    println!("{}", r1);
}
