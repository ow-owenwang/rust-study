/*
Rust 中的 迭代器（Iterator） 是一种强大且灵活的抽象，用于遍历集合（如数组、向量、哈希表等）或生成序列。理解迭代器是掌握 Rust 泛型和闭包的重要一步。

任何实现了 Iterator trait 的类型都可以被当作迭代器使用。
- next() 是核心方法，每次调用返回 Some(item) 或 None。
- 迭代器是“惰性”的，只有在调用像 next、collect 这样的消费方法时才真正执行。



 */

fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);

    println!("{}", &100);
}
