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
