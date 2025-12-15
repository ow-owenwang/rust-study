/*
在 Rust 中，Copy 是一个 标记 trait（marker trait），它表示一个类型的值在进行赋值（比如 let b = a）或传参时 可以按位复制，而不是移动（move）。

Copy Trait 的意义：
当一个类型实现了 Copy trait，那么它的变量在赋值或作为函数参数传递时，会复制一份数据，原来的变量仍然可以使用。

如果没有实现 Copy（但实现了 Drop 或默认行为），那么该变量在赋值后会被移动（moved），原变量不再可用。



Rust 中的**标量类型（scalar types）**默认实现了 Copy：
- 所有整数类型（u8, i32 等）
- 浮点类型（f32, f64）
- char
- bool
- 引用类型（比如 &T，但不是 &mut T）
数组和元组也可以是 Copy 的，只要它们的所有元素都是 Copy 的



注意：
Copy 类型 不能实现 Drop trait。两者是互斥的，因为实现了 Drop 意味着对象销毁时需要特殊处理，而 Copy 要求是简单的按位复制。
所有 Copy 类型也必须实现 Clone，不过 Copy 的 clone() 实际上只是一个浅拷贝。
 */

fn main() {
    // 这里 i32 是实现了 Copy trait 的类型，所以 x 被复制到 y，x 并未失效。
    let x = 5; // i32 默认实现了 Copy
    let y = x; // 发生了按位复制
    println!("x = {}, y = {}", x, y); // x 仍然可以使用

    // String 没有实现 Copy，它是堆分配类型，赋值操作会发生移动（move），之后 s 不再可用。
    let s = String::from("hello");
    let t = s; // s 被 move 到 t
               // println!("{}", s); // ❌ 编译错误，s 已失效

    // 数组中的元素必须都实现了 Copy，比如 [String; 3] 就不行。
    let arr = [1, 2, 3];
    let arr_copy = arr;
    println!("arr = {:?}, arr_copy = {:?}", arr, arr_copy);

    // 元组
    let tup = (1, true, 'x'); // i32, bool, char 都是 Copy
    let tup_copy = tup;
    println!("tup = {:?}, tup_copy = {:?}", tup, tup_copy);

    // 自定义类型，必须同时实现 Copy 和 Clone，否则报错
    #[derive(Copy, Clone, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 10, y: 20 };
    let p2 = p1;
    println!("p1 = {:?}, p2 = {:?}", p1, p2);
}