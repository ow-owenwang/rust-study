/*
闭包中的借用也会自动推断为可变或不可变
 */

fn main() {
    let mut s = String::from("hello");

    // 不可变借用
    let print = || println!("{}", s);

    // print(); // OK

    // 可变借用
    let mut modify = || s.push_str(" world");

    modify(); // OK
    println!("{}", s);
}
