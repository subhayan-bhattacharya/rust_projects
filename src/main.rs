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
}

impl Rectangle {
    fn area_increased(&self) -> u32 {
        self.length * self.width * 10
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
    println!("Displaying the fields of original struct : {dimensions:#?}");
    let new_area = dimensions.area_increased();
    println!("What is the changed area calculation {new_area}");
}