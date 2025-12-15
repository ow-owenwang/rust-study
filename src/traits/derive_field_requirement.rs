/*
当你对一个结构体（struct）或枚举（enum）使用 #[derive(SomeTrait)] 时，Rust 编译器 会尝试自动为这个类型实现对应的 trait。
但编译器必须递归地访问结构体的每个字段（或枚举的每个变体），看看它们是否也实现了这个 trait，才能生成合法的代码。

这种限制不仅适用于 Debug，还适用于其他可以自动派生的 trait


 */

// 可以编译通过的，因为 String 和 u8 都已经实现了 Debug。
#[derive(Debug)]
struct Person {
    name: String, // String 实现了 Debug
    age: u8,      // u8 也实现了 Debug
}

// 给字段类型手动实现或派生对应的 trait
#[derive(Debug)]
struct Secret;

#[derive(Debug)]
struct Vault {
    secret: Secret, // Secret 必须也得实现 Debug，否则给 Vault 自动派生 Debug 会失败
}
