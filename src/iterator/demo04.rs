/*
创建自定义迭代器
 */
struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= 5 {
            Some(self.count)
        } else {
            None
        }
    }
}
fn main() {
    let sum: usize = Counter::new() // 1, 2, 3, 4, 5
        .filter(|x| x % 2 == 1) // 过滤奇数：1, 3, 5
        .map(|x| x * x) // 平方：1, 9, 25
        .sum(); // 求和：35

    println!("Result: {}", sum);

    let vec: Vec<_> = Counter::new().collect();
    println!("{:?}", vec); // 输出: [1, 2, 3, 4, 5]

    let mut counter = Counter::new();

    while let Some(val) = counter.next() {
        println!("Got: {}", val);
    }
}
