//! ```cargo
//! [dependencies]
//! itertools = "0.14"
//! ```

use itertools::Itertools;

/*
滑动窗口
 */
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    for win in v.iter().copied().tuple_windows::<(_, _, _)>() {
        println!("{:?}", win);
    }
}
