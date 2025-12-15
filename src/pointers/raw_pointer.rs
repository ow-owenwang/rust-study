/*
1.*const T 和 *mut T 是类似 C 的指针。
2.不安全，你必须在 unsafe 块中解引用

用途：
- 与 C FFI 交互（如调用 C 函数）
- 实现底层数据结构（如操作内存地址）
 */

fn main() {
    let mut x = 5;
    let r1 = &x as *const i32; // 不可变引用 → 不可变裸指针，合理
    let r2 = &mut x as *mut i32; // 可变引用 → 可变裸指针，合法安全

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let y = 10;
    let r3 = &y as *const i32;
    unsafe {
        println!("r3 is: {}", *r3);
    }
}
