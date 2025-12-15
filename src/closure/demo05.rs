/*
闭包作为函数参数
 */

fn apply<F>(f: F)
where
    F: Fn(), // 传入一个实现 Fn trait 的闭包
{
    f();
}

fn main() {
    let message = "Hello from closure!";
    apply(|| println!("{}", message));
}
