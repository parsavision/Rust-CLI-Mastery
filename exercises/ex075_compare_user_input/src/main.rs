use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Hi! Do you want to talk to me? (yes/no)");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if input.trim().to_lowercase() == "yes" {
        println!("Great! Let's talk!");
    } else {
        println!("Okay, maybe next time!");
    }
}
