// Simple rust code to check if a number is even or odd

use std::io;

fn check_if_even(value: i32) -> bool {
    let remainder = value % 2;
    if remainder == 0 {
        true
    } else {
        false
    }
}

fn main() {
    println!("Please input a number to check if it is even or odd!");
    loop {
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        let number: i32 = match number.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        
        if number == 0 {
            println!("Okay quitting!");
            break;
        }
        let even = check_if_even(number);
        if even {
            println!("The number {number} is even");
        } else {
            println!("The number {number} is odd");
        }

    }
    
}