/*
Arc<T>：多线程共享数据，带原子引用计数

应用场景：在多线程中共享只读数据，比如缓存、配置对象等

特点
原子引用计数。
多线程共享安全。
如果想要内部可变，还需配合 Mutex<T> 或 RwLock<T> 使用。
 */

use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);

    for _ in 0..3 {
        let data_clone = Arc::clone(&data);
        thread::spawn(move || {
            println!("{:?}", data_clone);
        });
    }
}
