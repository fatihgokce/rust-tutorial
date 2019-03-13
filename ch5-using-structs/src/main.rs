#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

//Multiple impl Blocks
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is  {:?}", rect1);
    println!("{}", rect1.area());
    //Associated Functions
    let sq = Rectangle::square(5);
}
