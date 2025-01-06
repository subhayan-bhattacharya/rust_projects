// Understanding scopes in Rust 


use std::io;

fn take_ownership_string(passed: String) {
    print("I have taken ownership of the string {passed}!");
}

fn main() {
    println!("Understanding scopes!");
    let name: String = String::from("Subhayan");
    take_ownership_string(name);
    // The below should not work
    println!("Can i still access the original variable called name ? {name}")
}