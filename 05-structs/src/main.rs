#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
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
