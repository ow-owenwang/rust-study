fn main() {
    // 在Rust中，数组本身确实涉及所有权的概念，但需要根据数组的类型来理解它的所有权属性。

    /*
    数组中的基本类型
如果数组的元素类型是基本类型（如整数类型 i32、浮点类型 f64 等），这些类型实现了 Copy trait，因此不会涉及到所有权的转移。
     */
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("arr1 = {:?}, arr2 = {:?}", arr1, arr2);

    /*
数组中的非基本类型
如果数组中的元素类型是非基本类型（如 String），这些类型涉及到所有权管理。
     */
    let arr3 = [String::from("hello"), String::from("world")];
    let arr4 = arr3; // 赋值完成，arr3将不再有效
    println!("arr4 = {:?}", arr4);
    // println!("arr3 = {:?}", arr3); // 报错

    /*
    数组的整体所有权
Rust 中的数组本质上是一个拥有固定大小、且元素类型相同的集合。对于数组来说，数组本身拥有其所有元素的所有权。数组的所有权也会影响它的可变性、生命周期和如何在函数间传递。

当你将一个数组传递给函数时，如果数组类型涉及到所有权管理（如包含 String 类型的数组），那么函数会获得数组的所有权，原数组不再有效。
如果你希望共享数组的所有权，而不是转移所有权，可以使用数组的引用 & 或可变引用 &mut。
     */
    fn take_ownership(arr: [String; 2]) {
        println!("{:?}", arr);
    }

    let arr5 = [String::from("hello"), String::from("world")];
    take_ownership(arr5);

    // 这里再试图使用 `arr5` 会报错，因为所有权已经转移
    // println!("{:?}", arr5); // 编译错误：borrow of moved value
}