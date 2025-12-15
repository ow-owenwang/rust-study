fn foo() {
    println!("foo");
}

fn bar(a: i32, b: u32) {
    println!("{} {}", a, b);
}

// 有返回值
fn sum(a: i32, b: i32) -> i32 {
    // let result = a + b;
    // return result;

    // 简写
    a + b
}
fn main() {
    foo();
    bar(-1, 2);
    let res = sum(1, 2);
    println!("res: {}", res);
}
