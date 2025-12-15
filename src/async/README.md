
## 🌀 异步编程（Async Programming）

Rust 没有内建线程池或运行时，它将异步和并发的控制权交给了开发者。这种设计让它的异步编程非常强大也略显复杂。

### 1. async/await 语法

Rust 使用 `async` 和 `await` 实现异步操作：

```rust
async fn say_hello() {
    println!("Hello");
}

#[tokio::main]
async fn main() {
    say_hello().await;
}
```

* `async fn` 返回的是一个 **`Future`**
* `await` 表示等待 Future 完成
* 所有异步代码必须在运行时执行，比如 `tokio`、`async-std`

### 2. Future 和运行时

* Rust 的异步操作是通过 `Future` trait 来实现的，类似于 JavaScript 的 Promise，但是 **惰性（lazy）** 的。
* Future 不会自动运行，必须通过运行时去 **poll（轮询）** 才能推进状态。

### 3. 常用运行时

* **tokio**：功能全面，工业级异步运行时，生态最广。
* **async-std**：类似 `std` API 的异步库，更易上手。

### 4. 异步编程的挑战

* 生命周期 & 所有权管理（如 `Pin`、`Box::pin`）
* 类型复杂（特别是嵌套 `Future`）
* 错误处理（使用 `Result<T, E>` 配合 `.await`）

---

## 🚀 总结

| 特性  | 宏编程                            | 异步编程                            |
|-----|--------------------------------|---------------------------------|
| 用途  | 代码生成、DSL、抽象                    | 非阻塞 IO、高并发                      |
| 类型  | `macro_rules!`、过程宏             | `async/await`、`Future`          |
| 难点  | 语法复杂、调试难                       | 生命周期、运行时选择                      |
| 常用库 | `serde`, `log`, `tokio-macros` | `tokio`, `async-std`, `reqwest` |

