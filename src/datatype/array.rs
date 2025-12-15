fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr[1] = {}", arr[1]);
    // println!("arr[5] = {}", arr[5]); // 编译失败 error: this operation will panic at runtime

    // let index = "5".parse::<usize>().unwrap(); // cargo build可以成功，但是运行会失败，说明编译期可通过
    // println!("arr[5] = {}", arr[5]);

    let mut buf: [u32; 32 * 1024] = [0; 32 * 1024];
    println!("buf[1020] = {}", buf[1020]);
    buf[1020] = 10;
    println!("buf[1020] = {}", buf[1020]);
}
