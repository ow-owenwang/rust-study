fn main() {
    // 元组结构体，这种结构体类似于元组，但它有自己的类型名，字段没有名称，只有类型。
    // 元组结构体是一种特殊形式的结构体，字段是按顺序排列的，但没有字段名称。
    // Array 为结构体的名字，它包含了一个 Vec<i32> 作为其唯一的字段。
    struct Array(Vec<i32>);
    let array = Array(vec![1, 2, 3]);
    // 通过索引访问
    println!("array: {:?}", array.0);

    // 普通结构体
    struct Array1 {
        elements: Vec<i32>,
    }

    let array1 = Array1 {
        elements: vec![1, 2, 3],
    };
    // 通过字段名访问
    println!("array1: {:?}", array1.elements);


    /*
    选择使用哪种结构体：
如果结构体需要多个有意义的字段名称，使用普通结构体。
如果结构体只封装了几个字段，而且字段名称不重要，使用元组结构体。
     */
}