//! ```cargo
//! [dependencies]
//! itertools = "0.14"
//! ```

use itertools::Itertools;

/*
分组
 */
fn main() {
    let data = vec!["apple", "apricot", "banana", "blueberry"];

    let grouped = &data.iter().group_by(|word| word.chars().next().unwrap());

    for (key, group) in grouped.into_iter() {
        println!("{}: {:?}", key, group.collect::<Vec<_>>());
    }
}
