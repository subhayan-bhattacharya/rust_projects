// building structs in Rust

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.length > rectangle.length && self.width > rectangle.width
    }
}

fn main() {
    println!("practising rust struct !");
    let dimensions = Rectangle{
        length: 30,
        width: 50
    };
    let area = dimensions.area();
    println!("The area is {area}");
    let dimensions2 = Rectangle {
        length: 80,
        width: 90
    };
    let ans = dimensions.can_hold(&dimensions2);
    if ans {
        println!("the rectangle {dimensions:#?} can hold the other rectange {ans}");
    } else {
        println!("This rectangle {dimensions2:#?} is bigger");
    }
}