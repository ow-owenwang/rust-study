fn main() {
    // 序列：范围语法（range syntax）
    for i in 1..=5 {
        println!("{}", i);
    }

    for i in 1..5 {
        println!("{}", i);
    }

    for i in 'a'..='f' {
        println!("{}", i);
    }

    for i in 1..0 {
        println!("{}", i); // 不会执行任何内容，但不会报错
    }
}
