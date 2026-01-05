use std::io::{self, Write};

fn main() {
    // TODO: Implement split input exercise
    // Your task: Get input like "5 3" and split into two numbers
    let input = "5 3";
    let parts: Vec<&str> = input.trim().split(" ").collect();
    let num1 = parts[0]
        .parse::<i32>()
        .expect("Failed to parse first number");
    let num2 = parts[1]
        .parse::<i32>()
        .expect("Failed to parse second number");
    println!("Numbers: {}, {}", num1, num2);
    // Hints:
    // 1. Use .split(' ') or .split_whitespace() to separate values
    // 2. You'll need to parse each part separately
    // 3. Think about what happens if user enters wrong format

    println!("Exercise 84: Split Input");
    println!("Enter two numbers separated by space (e.g., '5 3'):");

    print!("> ");
    io::stdout().flush().expect("Failed to flush");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split(" ").collect();
    println!("The numbers you entered are {} and {}", parts[0], parts[1]);
    // TODO: Split the input and parse both numbers
    // TODO: Print the results
}
