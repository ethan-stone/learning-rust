fn main() {
    let rect = Rectangle {
        length: 12,
        width: 12,
    };

    println!("rectangle is {rect:#?}");

    let area = rect.area();

    println!("The area is: {area}");
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.length * self.width;
    }
}
