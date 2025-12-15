//! ```cargo
//! [dependencies]
//! num = "0.4"
//! ```

use num::complex::Complex;

fn main() {
    // 有理数和复数

    /*
    直接使用字段初始化语法创建一个复数。
    re 表示实部（real part），im 表示虚部（imaginary part）。
    所以 a 是复数 2.1 - 1.2i。
     */
    let a = Complex { re: 2.1, im: -1.2 };
    // 创建复数 b
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im) // 13.2 + 21i
}
