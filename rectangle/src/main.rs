#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

fn main() {
    let p = Rectangle {
        width: 10,
        height: 20,
    };

    println!("The area is {}", p.area());
}
