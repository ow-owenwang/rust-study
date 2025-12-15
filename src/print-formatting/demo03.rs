use std::fmt;

struct Person {
    name: String,
    age: u8,
}

// 控制输出格式
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 直接使用 f 控制格式
        if f.alternate() {  // 如果使用了 `#` 进行格式化
            write!(f, "Name: {}\nAge: {}", self.name, self.age)
        } else {
            write!(f, "{} ({})", self.name, self.age)
        }
    }
}

// 检查格式化选项
/*impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 检查宽度是否设置，若设置则调整输出宽度
        let width = f.width().unwrap_or(0);
        write!(f, "{:width$} is {} years old", self.name, self.age, width=width)
    }
}*/

// 嵌套格式化输出
// 有时候，你可能需要在实现 Display 时调用其他类型的 Display 或 Debug 实现。这时，你可以使用 Formatter 的方法来完成这些嵌套的格式化输出。
/*impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person: {}", self.name)?;
        fmt::Debug::fmt(&self.age, f)  // 调用 Debug 特征来格式化 age
    }
}*/


// 实现自定义的格式化逻辑
/*impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用精度选项
        if let Some(precision) = f.precision() {
            write!(f, "{:.precision$} (age: {})", self.name, self.age, precision=precision)
        } else {
            write!(f, "{} (age: {})", self.name, self.age)
        }
    }
}*/

fn main() {
    let p = Person {
        name: "surface".to_string(),
        age: 18,
    };
    println!("{}", p);       // 普通输出
    println!("{:#}", p);    // 使用 `#` 来切换格式
}
