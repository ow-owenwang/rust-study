/*
? 是一个非常强大、常用的语法糖，主要用于 简化错误处理 或 可选值（Option）处理。

它可以用于两种类型：
Option<T>：用于表示可能没有值
Result<T, E>：用于表示可能出错
 */

fn get_first_char(s: &str) -> Option<char> {
    let first = s.chars().next()?; // 如果是 None，直接返回 None
    Some(first)
}

// 等价于
fn get_first_char1(s: &str) -> Option<char> {
    match s.chars().next() {
        Some(first) => Some(first),
        None => return None,
    }
}
