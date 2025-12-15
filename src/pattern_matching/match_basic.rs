fn main() {
    let number = 10;

    match number {
        1 => println!("one"),                   // 精确匹配值。
        2 | 3 => println!("two or three"),      // 匹配多个值（逻辑“或”）。
        4..=10 => println!("between 4 and 10"), // 范围匹配。
        _ => println!("something else"),        // 通配符，匹配任意值（类似 default）。
    }
}
