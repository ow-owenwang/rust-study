fn main() {
    let x = 10;
    let mut y = 20;
    let r = &x; // 不可变引用
    let m = &mut y; // 可变引用

    println!("r: {}", r); // r: 10
    println!("m: {}", m); // m: 20
    println!("{}", x == 10); // true
    println!("{}", y == 20); // true
    println!("{:p}", r); // 内存地址
    println!("{}", *r); // 10
}
