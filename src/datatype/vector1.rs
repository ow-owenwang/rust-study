fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<i32> = Vec::new();
    v2.push(4);

    {
        let v3 = vec![1, 2, 3];
    }
    let one: &i32 = &v2[0];
    println!("{}", one); // 4
    println!("{}", *one); // 4
    println!("{:?}", v2); // [4]

    // 推荐
    match v2.get(4) {
        Some(value) => println!("值 {}", value),
        _ => println!("None"),
    }

    // 不可变遍历
    for i in &v {
        println!("{}", i);
    }

    // 可变遍历
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    #[derive(Debug)]
    enum Context {
        Text(String),
        Float(f32),
        Int(i32)
    }

    let c = vec![
        Context::Text(String::from("Hello")),
        Context::Float(3.14),
        Context::Int(-1),
    ];
    println!("{:?}", c);
}
