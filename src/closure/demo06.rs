/*
闭包作为返回值
 */

fn returns_closure() -> impl Fn(i32) -> i32 {
    let base = 10;
    move |x| x + base
}

fn main() {
    let add_base = returns_closure();
    println!("{}", add_base(5)); // 输出 15
}
