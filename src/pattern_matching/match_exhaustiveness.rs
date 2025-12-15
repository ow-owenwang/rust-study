enum Direction {
    Up,
    Down,
    Left,
    Right,
}
/*
match 表达式必须穷尽枚举的所有可能值。如果确实不关心剩余分支时，使用 _ 通配符。
使用 _ 后，如果枚举增加了新值，编译器就不会再提醒你是否处理了所有情况。所以对于 enum，最好不要用 _，除非你真的不关心未来变化。
 */
fn main() {
    let dir = Direction::Left;

    // 编译器会强制你处理所有可能的情况
    match dir {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"), // 注释其中一个就会报错
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }
}
