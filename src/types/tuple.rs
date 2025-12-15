fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}

fn main() {
    let tup: (i32, f64, u8) = (500, 6.5, 1);

    // 用模式匹配解构元组
    let (x, y, z) = tup;
    println!("{}", y);

    // 用 . 来访问元组
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{}", five_hundred);

    // 元组在函数返回值场景中很常见
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}
