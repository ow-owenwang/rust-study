fn main() {
    let mut x = 5;
    x = x + 1;
    println!("mut x = {}", x); // 输出 6

    let x = x * 2; // 遮蔽了前一个 x
    println!("shadowed x = {}", x); // 输出 12


    // 遮蔽可多次连续使用
    let y = "42";
    let y = y.parse::<i32>().unwrap();
    let y = y + 1;
    println!("y = {}", y); // 输出 43

}
