fn main() {
    // 元组结构体
    // 元组结构体在你希望有一个整体名称，但是又不关心里面字段的名称时将非常有用。
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 单元结构体
    // 如果你定义一个类型，但是不关心该类型的内容, 只关心它的行为时，就可以使用 单元结构体
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    impl SomeTrait for AlwaysEqual {}
}