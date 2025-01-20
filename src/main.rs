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

fn main() {
    println!("practising rust struct !");
    let dimensions = Rectangle{
        length: 30,
        width: 50
    };
    let area = dimensions.area();
    println!("The area is {area}");
    println!("Displaying the fields of original struct : {dimensions:#?}")
}