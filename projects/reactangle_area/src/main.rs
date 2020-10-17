#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuples(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 50,
        height: 30,
    };

    println!(
        "The area of the rectangle {:?} is {} square pixels.",
        rect1,
        area_struct(&rect1)
    );

    println!(
        "The area of the rectangle {:?} is {} square pixels. Can r2 hold r1? {}",
        rect1,
        rect1.area(),
        rect2.can_hold(&rect1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn area_tuples(reactangle: (u32, u32)) -> u32 {
    reactangle.0 * reactangle.1
}
