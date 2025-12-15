fn main() {
    /*
    这段代码的问题在于，Rust 在初始化数组时，使用的语法 [value; N] 会尝试复制（clone）value N 次。
    但是对于 String 类型，它并没有实现 Copy trait，因为它是一个堆分配的数据结构，包含所有权，因此你无法简单地通过复制来初始化多个相同的 String。
    因此，直接使用 String::from("rust is good!") 作为数组的元素初始化值会导致编译错误。
     */
    // let array = [String::from("rust is good!"); 8];

    /*
    要解决这个问题，可以使用 String 的 clone() 方法来确保每个数组元素都是独立的堆分配对象。你可以使用迭代器来生成数组，或者直接使用 vec! 宏，然后再将其转换为数组。
     */
    let array: [String; 6] = std::array::from_fn(|_| String::from("rust is good!"));
    println!("{:#?}", array);

    let array1: [String; 3] = vec![String::from("rust is good!"); 8].try_into().unwrap();
    println!("{:#?}", array1);
}