fn main() {
    // 可恢复错误
    let a: Result<u32, &'static str> = Result::Ok(42);
    println!("a = {:?}", a);

    let b: Result<u32, &'static str> = Result::Err("err");
    println!("b = {:?}", b);
}


