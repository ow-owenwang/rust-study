//! ```cargo
//! [dependencies]
//! itertools = "0.14"
//! ```

use itertools::Itertools;

/*
笛卡尔积
 */
fn main() {
    let a = [1, 2];
    let b = ['a', 'b'];

    for pair in a.iter().cartesian_product(&b) {
        println!("{:?}", pair);
    }
}
