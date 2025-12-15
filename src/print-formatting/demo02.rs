/*
在 Rust 中，很多内置类型都实现了 Display 和 Debug 特征。

实现了 Display 类型：
- 基本数值类型：i32、u32、f64等整数和浮点数类型。
- 字符串类型：String、&str
- 字符类型：char
- 布尔类型：bool


实现了 Debug 类型：
- 所有实现了 Display 的类型：因为 Debug 特征比 Display 更宽泛，通常来说，所有实现了 Display 的类型也实现了 Debug。
- 数组与切片：[i32; 4], [&str]
- 元组：如 (i32, f64)
- 结构体和枚举：在使用 #[derive(Debug)] 派生时，结构体和枚举会自动实现 Debug。
- 集合类型：Vec<T>、HashMap<K, V>、Option<T>、Result<T, E>
 */

fn main() {
    let point = (3, 5);
    println!("{:?}", point); // (3, 5)
}