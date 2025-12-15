fn main() {
    let a = 1; // 自动推导类型
    let b: u32 = 1;
    println!("a = {}", a);
    println!("b = {}", b);

    // 报错
    // a = 10;
    // println!("a = {}", a);

    // mut 表示可变
    let mut x = 1;
    println!("x = {}", x);
    x = 10;
    println!("x = {}", x);
}