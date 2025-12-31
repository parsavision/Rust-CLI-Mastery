use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    loop {
        print!("Do you want to talk to me? (yes/no)");
        io::stdout().flush().expect("Failed to flush stdout");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim().to_lowercase() == "yes" {
            println!("Great! Let's talk!");
            break;
        } else if input.is_empty() {
            println!("Please enter a valid response.");
        } else if input.len() < 3 {
            println!("Please enter a valid response.");
        } else if input.trim().to_lowercase() == "no" {
            println!("You must talk to me!There is no escape!");
            continue;
        } else {
            println!("Invalid response. Please enter 'yes' or 'no'.");
        }
    }
}
