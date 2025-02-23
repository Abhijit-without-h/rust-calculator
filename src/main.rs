use std::io;

fn main() {
    println!("Welcome to Rust Calculator!");
    println!("Enter calculations like '2 + 3' or 'exit' to quit\n");
    
    loop {
        let mut input = String::new();
        
        // Read input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input = input.trim().to_lowercase();
        
        // Check for exit command
        if input == "exit" {
            println!("Goodbye!");
            break;
        }
        
        // Split into parts
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        // Validate input format
        if parts.len() != 3 {
            println!("Invalid input! Please use format: 'number operator number'\n");
            continue;
        }
        
        // Parse numbers
        let num1: f64 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid first number\n");
                continue;
            }
        };
        
        let num2: f64 = match parts[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid second number\n");
                continue;
            }
        };
        
        let operator = parts[1];
        
        // Perform calculation
        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Error: Division by zero\n");
                    continue;
                }
                num1 / num2
            }
            _ => {
                println!("Invalid operator. Use +, -, *, or /\n");
                continue;
            }
        };
        
        println!("Result: {}\n", result);
    }
}
