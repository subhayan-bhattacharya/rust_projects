// Simple rust code to calculate the nth fibonacci number 
// Questions:
// How to make sure that rust accepts a number from the terminal which is u32


use std::io;

fn nth_fibonacci_number(value: u64) -> u64 {
    if value <= 1 { value } else {
        let mut first: u64 = 0;
        let mut second: u64 = 1;
        let mut fibonacci: u64 = 0;
        for _number in 2..value {
            fibonacci = first + second;
            first = second;
            second = fibonacci;
        }
        return fibonacci
    }
}

fn main() {
    println!("Please input the number for which the the nth Fibonacci is desired");
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
            let value: u64 = number as u64; // explicit conversion between types
            let answer: u64 = nth_fibonacci_number(value);
            println!("The answer is {answer}");
        };

    }
    
}