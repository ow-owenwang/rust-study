// 求两个无符号数的平均数

// 考虑整数溢出问题

/*// 当 a + b > u32::MAX 时会直接溢出。
fn avg(a: u32, b: u32) -> u32 {
    (a + b) / 2
}*/

// 使用位运算
fn avg(a: u32, b: u32) -> u32 {}

fn main() {
    assert_eq!(avg(4294967295, 4294967295), 4294967295);
    assert_eq!(avg(0, 0), 0);
    assert_eq!(avg(10, 20), 15);
    assert_eq!(avg(4294967295, 1), 2147483648);
    println!("passed");
}
