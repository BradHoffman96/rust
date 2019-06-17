struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50
    };

    let rect2 = Rectangle {
        height: 20,
        width: 20
    };

    println!("The area of the rectangle is: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

}