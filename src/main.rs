// the sum of all multiples of 3 or 5 below a given number 


use std::io;

fn is_a_multiple_of_three_or_five(number: u32) -> bool {
    let remainder_3 = number % 3;
    let remainder_5 = number % 5;
    if remainder_3 == 0 {
        true
    } else if remainder_5 == 0 {
        true
    } else {
        false
    }
}

fn main() {
    println!("Please input the under which multiples of 3 or 5 needs to be added!");
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
        } else if number <= 0 {
            println!("This does not make sense provide a number greater than 0.");
            continue;
        } else {
            let value: u32 = number as u32; // explicit conversion between types
            let mut answer: u32 = 0;
            for num in 1..value {
                if is_a_multiple_of_three_or_five(num) {
                    answer = answer + num;
                }
            }
            println!("The answer is {answer}");
        };

    }
    
}