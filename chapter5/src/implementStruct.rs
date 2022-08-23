#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, otherRectangle: &Rectangle) -> bool{
        if self.width > otherRectangle.width && self.height > otherRectangle.height {
            return true
        }
        false
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height:size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rec1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rec1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(3);
    println!("rect4 is: {:?}", rect4);
}