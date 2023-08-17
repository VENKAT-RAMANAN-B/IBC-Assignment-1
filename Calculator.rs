use std::io;

fn main() {
    loop {
        println!("Calculator Menu:");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit");
        println!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Please enter a number.");
                continue;
            }
        };

        if choice == 5 {
            println!("Exiting the calculator.");
            break;
        }

        println!("Enter the first number: ");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Failed to read line");
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Please enter a valid number.");
                continue;
            }
        };

        println!("Enter the second number: ");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Failed to read line");
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Please enter a valid number.");
                continue;
            }
        };

        match choice {
            1 => println!("Result: {}", add(&num1, &num2)),
            2 => println!("Result: {}", subtract(&num1, &num2)),
            3 => println!("Result: {}", multiply(&num1, &num2)),
            4 => println!("Result: {}", divide(&num1, &num2)),
            _ => println!("Invalid choice. Please select a valid operation."),
        }
    }
}

fn add(a: &f64, b: &f64) -> f64 {
    *a + *b
}

fn subtract(a: &f64, b: &f64) -> f64 {
    *a - *b
}

fn multiply(a: &f64, b: &f64) -> f64 {
    *a * *b
}

fn divide(a: &f64, b: &f64) -> f64 {
    if *b == 0.0 {
        println!("Error: Division by zero.");
        std::f64::NAN
    } else {
        *a / *b
    }
}
