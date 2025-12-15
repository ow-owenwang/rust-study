fn main() {
    let use_closure = || {
        println!("hello");
    };

    use_closure();

    // 写法1
    let add_one = |x: u32| -> u32 { x + 1 };
    // 写法2：为什么报错
    // let add_one_v1 = |x| x + 1;

    // 不能推导两次
    let foo = |x| x;
    let s = foo(String::from("hello"));
    println!("{}", s);

    // let i = foo(4);
    // println!("{}", i);
}
