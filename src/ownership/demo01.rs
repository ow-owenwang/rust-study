fn main() {
    /*
String类型：
String是一个拥有所有权的字符串类型，存储在堆上。
当你将一个String类型的变量赋值给另一个变量时，所有权会被移动，原变量会变得无效。

&str类型：
&str是一个不可变的字符串切片，它是对字符串数据的引用，不拥有数据。
当你将一个&str类型的变量赋值给另一个变量时，实际上是创建了另一个引用，它们指向同一块内存。
引用不会转移所有权，因此原变量依然有效。
     */


    let s1 = String::from("hello");
    // let s2 = s1; // 报错
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2); // s1 = hello, s2 = hello


    let x: &str = "world";
    let y = x;
    println!("x = {}, y = {}", x, y); // x = world, y = world
}