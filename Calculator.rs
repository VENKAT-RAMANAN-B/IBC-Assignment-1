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
        let choice: u32 = choice.trim().parse().expect("Invalid choice");

        if choice == 5 {
            println!("Exiting the calculator.");
            break;
        }

        println!("Enter the first number: ");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Failed to read line");
        let num1: f64 = num1.trim().parse().expect("Invalid number");

        println!("Enter the second number: ");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Failed to read line");
        let num2: f64 = num2.trim().parse().expect("Invalid number");

        match choice {
            1 => println!("Result: {}", num1 + num2),
            2 => println!("Result: {}", num1 - num2),
            3 => println!("Result: {}", num1 * num2),
            4 => {
                if num2 == 0.0 {
                    println!("Error: Division by zero.");
                } else {
                    println!("Result: {}", num1 / num2);
                }
            }
            _ => println!("Invalid choice. Please select a valid operation."),
        }
    }
}
