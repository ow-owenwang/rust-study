fn main() {
    /*
    Rust 允许在同一作用域内使用 let 重新声明同名变量，新的变量会遮蔽旧的变量。
    遮蔽会生成一个新的变量绑定，它与前一个变量仅名字相同，实际是完全不同的绑定，通常会导致新的内存分配（视类型而定）。
    遮蔽的主要用途是，在你不再需要原变量时，可以使用相同的名字绑定新值，避免引入额外变量名，使代码更简洁。
    使用 mut 可以在不更换变量绑定的情况下修改值，对于栈上数据不会发生内存重新分配；而堆分配类型（如 Vec、String）是否重新分配取决于操作本身（如扩容）。

         */

    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "    "; // spaces 是 &str 类型
    let spaces = spaces.len(); // 新的 spaces 遮蔽了旧的，是 usize 类型
    println!("{}", spaces); // 打印的是 usize 类型
}
