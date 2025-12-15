fn main() {
    println!("{1}{0}{1}", 4, 2);
    // 命名参数
    println!("name={name} age={age}", name = "jack", age = 6);

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("{number:>width$}", number = 1, width = 6);

    println!("{number:>0width$}", number = 1, width = 6);

    println!("My name is {0}, {1} {0}", "Bond"); // 编译将会报错，补上漏掉的参数："James"
}
