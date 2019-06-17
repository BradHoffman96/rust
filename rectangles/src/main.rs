struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50
    };

    println!("The area of the rectangle is: {}", rect1.area());

}