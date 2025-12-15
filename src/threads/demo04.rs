/*
异步编程（Async / Await）

除了传统线程外，Rust 也支持异步并发，通过 async/await 关键字 + 运行时（如 tokio、async-std）实现轻量协程。
 */

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Async task done");
    });

    println!("Main running");
    handle.await.unwrap();
}
