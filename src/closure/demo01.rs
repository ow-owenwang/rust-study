fn main() {
    // 简单闭包：两个数相加
    let add = |a, b| a + b;
    println!("Sum: {}", add(2, 3));

    // 捕获变量
    let x = 5;
    let add_x = |y| y + x;
    println!("5 + 3 = {}", add_x(3)); // x 被借用
    println!("{}", x);
}
