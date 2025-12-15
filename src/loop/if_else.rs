fn main() {
    let n = 5;
    if n > 0 {
        println!("{} is positive", n);
    } else if n < 0 {
        println!("{} is negative", n)
    } else {
        println!("{} is zero", n);
    }

    // 如果使用if-else返回一个值，则所有分支必须返回相同的类型
    let m = if n < 0 {
        2.0
    } else {
        3.0
    }; // 注意分号
    println!("m = {}", m);
}
