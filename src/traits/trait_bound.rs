/*

*/

// 这里的 T: std::fmt::Debug 就是一个 trait bound，意思是：泛型类型 T 必须实现 Debug 这个 trait。这样你才能对它使用 println!("{:?}", ...) 这种调试输出。
fn print_debug<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item);
}

// 要求类型 T 同时实现了 Debug 和 Clone 两个 trait。
fn print_and_clone<T: std::fmt::Debug + Clone>(item: T) {
    let cloned = item.clone();
    println!("{:?}", cloned);
}

// 使用 where 语法（可读性更好）
// 当泛型变复杂时，where 语法可以让代码更清晰。
fn process<T, U>(t: T, u: U)
where
    T: Clone + std::fmt::Debug,
    U: std::fmt::Display,
{
    println!("{:?}", t.clone());
    println!("{}", u);
}
