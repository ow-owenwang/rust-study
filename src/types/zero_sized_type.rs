/*
零大小类型（ZST）

一些类型在编译期已知它们不需要占用任何内存（如 PhantomData<T>），这被称为 Zero-Sized Type。

用途：
类型标记（marker）
编译器元信息


 */

use std::marker::PhantomData;

struct MyType<T> {
    data: i32,
    marker: PhantomData<T>,
}
