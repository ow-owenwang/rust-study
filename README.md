#    

需要安装 `rust-script`：

`rust-script` 不会自动解析 `use rand::Rng` 并帮你下载依赖，你需要显式声明依赖。
`rust-script` 默认 不会像 `Cargo` 那样自动读 `Cargo.toml`。

在文件头部声明依赖，示例：

```rust
#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! rand = "0.8"
//! ```

use rand::Rng;

fn main() {
    let x = rand::thread_rng().gen_range(1..=10);
    println!("random: {}", x);
}
```


以脚本方式运行：rust-script src/main.rs


## Cargo 常用命令部分



* `cargo new`：生成新的项目模板
* `cargo build`：构建项目，生成可执行文件或库
* `cargo run`：构建并运行项目
* `cargo test`：运行测试用例
* `cargo doc`：生成项目文档
* `cargo publish`：发布库到 crates.io（仅限库）


* `cargo check` **会进行语法和类型检查**
* **不会生成可执行文件或库**
* 在日常开发中常用来快速发现错误

`cargo check` 只进行编译检查（不生成产物），速度比 cargo build 快，常用于开发阶段


* rustfmt 是 Rust 官方格式化工具
* 安装方式：

  ```bash
  rustup component add rustfmt
  ```
* 在项目根目录执行：

  ```bash
  cargo fmt
  ```

  会自动格式化整个项目

* rustfmt 的配置可以通过 `rustfmt.toml` 文件自定义


## `cargo new hello --lib` 与 `--bin` 的区别


### `cargo new hello --bin`（默认）

生成结构类似：

```
hello/
├── Cargo.toml
└── src/
    └── main.rs
```

* 是一个 **可执行程序**
* 入口是 `fn main()`
* `cargo run` 会直接运行

### `cargo new hello --lib`

生成结构类似：

```
hello/
├── Cargo.toml
└── src/
    └── lib.rs
```

* 是一个 **库项目**
* 没有 `main` 函数
* 主要用于被其他 crate 依赖
* 默认生成一个测试模块
