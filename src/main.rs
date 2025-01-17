// building structs in Rust

struct Rectangle {
    length: u32,
    width: u32
}

fn calculate_area(dimensions: &mut Rectangle) -> u32 {
    dimensions.length = dimensions.length * 5;
    dimensions.length * dimensions.width
}

fn main() {
    println!("practising rust struct !");
    let mut dimensions = Rectangle{
        length: 30,
        width: 50
    };
    let area = calculate_area(&mut dimensions);
    let new_length = dimensions.length;
    println!("The area is {area}");
    println!("Accessing original struct field {new_length}");
}