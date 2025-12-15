fn main() {
    /*
    数值类型: 有符号整数 (i8, i16, i32, i64, isize)、 无符号整数 (u8, u16, u32, u64, usize) 、浮点数 (f32, f64)、以及有理数、复数
    字符串：字符串字面量和字符串切片 &str
    布尔类型： true和false
    字符类型: 表示单个 Unicode 字符，存储为 4 个字节
    单元类型: 即 () ，其唯一的值也是 ()


     */
    assert_eq!(100u8.saturating_add(1), 101); // .saturating_add(1)：饱和加法，当结果超过 u8 的最大值时，会“钉死”在 u8::MAX（255），不会溢出。
    assert_eq!(u8::MAX.saturating_add(127), u8::MAX);

    let a: u8 = 255;
    let b = a.wrapping_add(20); // .wrapping_add(20)：回绕加法，超过最大值后从 0 开始回绕（模 256 运算）。
    println!("{}", b); // 19
}
