use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    loop {
        print!("Enter a positive number: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim().parse::<u32>().is_ok() {
            println!("You entered: {}", input.trim());
            break;
        }
        println!("Invalid input. Please enter a positive number.");
        input.clear();
    }
}
