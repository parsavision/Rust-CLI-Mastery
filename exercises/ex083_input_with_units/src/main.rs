use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("How much do you weigh?\n(example: 5kg)\nEnter your weight: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!(
        "You weigh {} kilograms",
        &input.trim()[0..input.trim().len() - 2]
    );
}
