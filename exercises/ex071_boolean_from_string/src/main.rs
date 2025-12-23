use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    loop {
        print!("Do you want to talk with me?(true/false): ");
        io::stdout().flush().expect("Failed to flush stdout");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<bool>() {
            Ok(true) => {
                println!("No, I'm busy plotting my next evil scheme! Muahaha, get out!");
                std::process::exit(0);
            }
            Ok(false) => {
                println!("Okay, maybe later.");
                std::process::exit(0);
            }
            _ => println!("I didn't understand that."),
        }
    }
}
