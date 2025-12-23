use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter your name: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("Hello, {}!", input.trim());
    println!(
        "The first character of your name is : {}",
        input.chars().next().unwrap_or('?')
    );
}
