use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter a number : ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!(
        "You entered: {}",
        input.trim().parse::<i32>().expect("Failed to parse input")
    );
}
