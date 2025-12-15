use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    // 将 &MyBox<T> 自动转换为 &T。
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {

}