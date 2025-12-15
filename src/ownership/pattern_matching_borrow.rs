/*
模式匹配中的借用也要遵守借用规则
 */

fn main() {
    let mut tuple = (1, 2);
    /*
    模式匹配 + 引用绑定，不是赋值，而是结构解构绑定引用。

    ref mut x	可变引用 tuple.0，x: &mut i32
    ref y	不可变引用 tuple.1，y: &i32

    即：
    x 是 &mut i32 类型，指向 tuple.0。
    y 是 &i32 类型，指向 tuple.1。
     */
    let (ref mut x, ref y) = tuple;

    // *x 是对 &mut i32 的 解引用，得到实际的 i32 值。
    *x += 1;
    println!("x = {}, y = {}", x, y); // x = 2, y = 2
}
