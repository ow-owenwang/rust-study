fn main() {
    let a: i32 = 10;
    let b: char = 'a';

    let t1 = (a, b);
    println!("t1 = {:?}", t1);

    let (c, d) = t1;
    println!("c = {} d = {}", c, d);
    println!("t1.0 = {} t1.1 = {}", t1.0, t1.1);

    // 多返回值
    let (result, is_overflow) = a.overflowing_add(10);
    println!("result = {}, is_overflow = {}", result, is_overflow);
}
