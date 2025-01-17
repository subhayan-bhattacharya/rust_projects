// building structs in Rust
fn calculate_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn main() {
    println!("practising rust struct !");
    let dimensions = (50, 30);
    let area = calculate_area(dimensions);
    println!("The area is {area}");
}