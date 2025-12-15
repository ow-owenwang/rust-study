use std::fmt;

/*
**外部类型**是指那些在你当前的代码文件或模块中**没有定义**的类型。例如，Rust 标准库中的类型（如 `Vec<T>`、`String`、`HashMap<K, V>` 等）或者来自第三方库的类型。
**外部特征**指的是那些**没有由你自己定义**的特征。例如，标准库中的特征（如 `std::fmt::Display`、`std::clone::Clone`、`std::cmp::PartialEq` 等）或者来自第三方库的特征。


孤儿原则：
孤儿规则的核心思想是：**你只能为你自己定义的类型实现你自己定义的特征**，或者为**你自己定义的类型**实现**外部定义的特征**。换句话说，你不能为两个都不属于你定义的东西（即“孤儿”）之间创建新的实现。



### 为什么使用 Newtype？
1. **遵循孤儿规则**：解决了无法直接为外部类型实现外部特征的问题。
2. **增加类型安全性**：有时候你希望进一步区分数据类型，避免混淆，Newtype 可以帮助你创建更具体的类型。

Newtype 模式是一种非常有用的 Rust 技巧，它允许你通过定义一个新的结构体包装外部类型，从而为其实现外部特征，并绕过孤儿规则的限制。
 */
fn main() {
    struct Array(Vec<i32>);

    // 为结构体 Array 实现 Display 特征，当使用 println!("{}", arr); 打印 Array 实例时，他会按照定义的格式显示出来。
    impl fmt::Display for Array {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "数组是：{:?}", self.0)
        }
    }

    let a = Array(vec![1, 2, 3]);
    println!("{}", a);

    // 为自己定义的类型实现外部特征
    struct MyType;
    impl fmt::Display for MyType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "MyType")
        }
    }

    let my_type = MyType;
    println!("{}", my_type);

    // 为外部类型实现自己定义的特征
    trait MyTrait {
        fn do_something(&self);
    }

    impl MyTrait for Vec<i32> {
        fn do_something(&self) {
            println!("Doing something with Vec<i32>");
        }
    }

    let v = vec![1, 2, 3];
    v.do_something();
    println!("{:?}", v);
}