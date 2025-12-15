
// 发散函数
fn foo() -> ! {
    panic!("this shouldn't panic!");
}

fn main() {
    foo();

}