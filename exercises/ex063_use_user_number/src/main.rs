use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    println!("Enter a number and I'll square it (e.g., 2 -> 4, 3 -> 9).");
    print!("Please enter a number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().parse::<i32>() {
        Ok(number) => {
            println!("The square of {} is {}", number, number.pow(2));
            print!("Do you want to enter another number? (y/n): ");
            io::stdout().flush().expect("Failed to flush stdout");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let answer = input.trim().to_lowercase();
            if answer == "y" {
                main();
            } else if answer == "n" {
                println!("Goodbye!");
                std::process::exit(0);
            } else {
                println!("Invalid input. Please enter 'y' or 'n'.");
                main();
            }
        }
        Err(_) => {
            println!("Invalid input. Please type a number!");
            main();
        }
    }
}
