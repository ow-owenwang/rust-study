use std::thread;

fn main() {
    let times3 = |n: u32| -> u32 { n * 3 };

    println!("times3 = {}", times3(10));

    let hello_message = "Hello, World!";
    thread::spawn(move || {
        println!("{}", hello_message);
    })
    .join();
}
