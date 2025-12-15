#[derive(Debug)]
struct Dog {
    name: String,
    weight: f32,
    height: f32,
}

impl Dog {
    fn get_name(&self) -> &str {
        &(self.name[..])
    }

    fn get_height(&self) -> f32 {
        self.height
    }
}

impl Dog {
    fn get_weight(&self) -> f32 {
        self.weight
    }
}
fn main() {
    let d = Dog {
        name: String::from("wangcai"),
        weight: 3.0,
        height: 4.0,
    };

    println!("dog = {:#?}", d);
    println!("name = {}", d.get_name());
    println!("weight = {}", d.get_weight());
}
