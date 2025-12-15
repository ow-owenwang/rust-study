/*
Rust 中有两种宏：
- 宏规则（Declarative Macros）
又叫做 macro_rules! 宏，类似于 C 语言中的预处理宏，但更强大、类型安全。
你可以通过匹配规则和重复模式来构造复杂的代码生成逻辑。

- 过程宏（Procedural Macros）
比 macro_rules! 更灵活，可用于自定义语法扩展，分为三类：
函数宏（#[proc_macro])：操作自由格式代码。
派生宏（#[proc_macro_derive])：自动为结构体/枚举生成实现。
属性宏（#[proc_macro_attribute])：用于自定义属性。
需要注意：过程宏必须定义在独立的 crate 中（类型为 proc-macro）。

#[derive(Debug, MyMacro)]  // 通过 #[proc_macro_derive(MyMacro)] 实现
struct MyStruct;

宏的用途
- 代码生成（如 serde 的 #[derive(Serialize)]）
- 自动实现 trait
- 实现 DSL（领域特定语言）
- 提高代码复用性
 */


macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

fn main() {
    say_hello!();
}
