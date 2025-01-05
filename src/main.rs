// Write a function that calculates the sum of all prime numbers below a given number 


use std::io;

fn is_number_prime(number: u32) -> bool {
    for num in 2..number {
        if number % num == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("Please input the under which we need sum of primes!");
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
            for num in 2..value {
                if is_number_prime(num) {
                    answer = answer + num;
                }
            }
            println!("The answer is {answer}");
        };

    }
    
}