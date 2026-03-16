#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 32,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} in square pixels.",
        rect1.area()
    );

    // println!("rect1 is {rect1:#?}");
    dbg!(&rect1);
}
