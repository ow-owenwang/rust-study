//! ```cargo
//! [dependencies]
//! rayon = "1.10"
//! ```

use rayon::prelude::*;

/*
并行求平方和

.par_iter() 是并行版本的 .iter()，会自动分配到多个线程处理。
其它还有：.par_chunks(), .par_map(), .par_sort(), .par_bridge() 等等。
自动使用线程池，不需手动管理线程，适合 CPU 密集型任务。
 */
fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 并行迭代，求每个元素的平方后再求和
    let sum: i32 = data.par_iter().map(|x| x * x).sum();

    println!("Parallel sum of squares: {}", sum);
}
