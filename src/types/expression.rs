fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let y = if x % 2 == 1 { "odd" } else { "even" };
    // 或者写成一行
    let z = if x % 2 == 1 { "odd" } else { "even" };
}

fn main() {
    // 表达式
    let y = {
        // 语句块
        let x = 3;
        x + 1 // 不能以分号结尾
    };

    println!("The value of y is: {}", y);

    assert_eq!(ret_unit_type(), ()); // 表达式如果不返回任何值，会隐式地返回一个 ()
}
