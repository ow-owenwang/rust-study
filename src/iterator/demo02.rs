/*

适配器是对原始迭代器的转换，不会消耗迭代器本身（惰性）。

**常用的迭代器适配器**
| 方法          | 描述                    |
| ----------- | --------------------- |
| `map`       | 对每个元素应用函数并返回新迭代器      |
| `filter`    | 保留满足条件的元素             |
| `enumerate` | 返回 `(index, item)` 元组 |
| `take(n)`   | 只取前 n 个元素             |
| `skip(n)`   | 跳过前 n 个元素             |
| `rev()`     | 反转迭代器（需双向迭代器）         |

 */

fn main() {
    let v = vec![1, 2, 3, 4];
    let result: Vec<_> = v.iter().map(|x| x * 2).collect();
    assert_eq!(result, vec![2, 4, 6, 8]);
}
