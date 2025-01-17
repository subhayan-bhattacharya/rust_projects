// building structs in Rust
fn calculate_area(length: u32, width: u32) -> u32 {
    length * width
}

fn main() {
    println!("practising rust struct !");
    let length = 30;
    let width = 50;
    let area = calculate_area(length, width);
    println!("The area is {area}");
}