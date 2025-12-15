/*
可变借用时不能有任何其他借用（即独占访问）
 */

fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    r1.push_str(", world"); // OK: 唯一的可变借用

    // let r2 = &mut s; // ❌ 错误：不能第二次可变借用，直到 r1 生命周期结束
    println!("{}", r1);
}
