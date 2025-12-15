/*
const 是在编译期确定值的常量，必须显式指定类型，例如 i32, u32, f64 等，不像 let，const 不能通过类型推导。
*/

fn main() {
    const MAX_POINT: i32 = 100;
    println!("MAX_POINT = {}", MAX_POINT);
}
