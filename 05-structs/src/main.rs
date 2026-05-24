#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    fn square(&self) -> Rectangle {
        return Rectangle {
            width: self.height,
            height: self.height,
        };
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 50,
        height: 20,
    };

    let rectangle_area = rectangle.area();

    println!("{:#?}", rectangle);
    println!("{}", rectangle_area)
}
