use std::io::{self, Write};

fn main() {
    print!("Hi! What is your name?: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input_name = String::new();
    io::stdin()
        .read_line(&mut input_name)
        .expect("Failed to read line");
    println!("Hello, {}!", input_name.trim());
}
