fn main() {
    let x = (-42.0_f32).sqrt(); // 负数的平方根在实数范围内是未定义的（会返回 NaN）。不会 panic。

    // 判断 x 是否是 NaN。
    if x.is_nan() {
        println!("为定义的数学行为");
    }
    assert_eq!(x, x); // NaN != NaN（任何 NaN 和自身都不相等）。
}
