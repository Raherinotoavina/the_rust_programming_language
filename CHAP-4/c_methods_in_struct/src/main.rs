#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32
}

impl Rectangle {
    // &self is a short of self:&self
    fn area (&self) -> u32 {
        self.width * self.height
    }

    fn can_hold (&self, other:&Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // Associated function that don't have self methods
    fn square (size:u32) -> Self {
        Self {
            width : size,
            height : size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width : 20,
        height : 40
    };

    let rect1 = Rectangle {
        width : 21,
        height : 20,
    };

    let area = rect.area();
    println!("Our area is : {}", area);
    if rect.can_hold(&rect1) {
        println!("Rect 1 peut contenir rect 2");
    } else {
        println!("Rect 1 ne peut pas contenir rect 2");
    }

    // Usage of the associated function that don't have self parameter
    let square = Rectangle::square(10);
    println!("{:?}", square);
}
