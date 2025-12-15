/*fn avg(a: u32, b: u32) -> u32 {
    (a & b) + ((a ^ b) >> 1)
}

fn main() {
    assert_eq!(avg(4294967295, 4294967295), 4294967295);
    assert_eq!(avg(0, 0), 0);
    assert_eq!(avg(10, 20), 30);
    assert_eq!(avg(4294967295, 1), 2147483648);
    println!("passed");
}*/
mod quiz_game;

fn main() {
    // let a: u32 = 4294967295;
    let a: u32 = "4294967295".parse::<u32>().unwrap();
    let b: u32 = 1;

    let sum = a + b; // 溢出
    println!("{:?}", sum);


}
