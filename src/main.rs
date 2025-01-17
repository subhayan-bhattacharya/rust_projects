// building structs in Rust

struct Rectangle {
    length: u32,
    width: u32
}

fn calculate_area(dimensions: &Rectangle) -> u32 {
    dimensions.length * dimensions.width
}

fn main() {
    println!("practising rust struct !");
    let dimensions = Rectangle{
        length: 30,
        width: 50
    };
    let area = calculate_area(&dimensions);
    println!("The area is {area}");
}