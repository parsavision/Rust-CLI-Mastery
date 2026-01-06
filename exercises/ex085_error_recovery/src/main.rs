use std::io::{self, Write};

fn main() {
    println!("Exercise 85: Error Recovery");
    println!("Enter a number:");

    print!("> ");
    io::stdout().flush().expect("Failed to flush");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // TODO: Instead of using .expect(), use match to handle parse errors
    // match input.trim().parse::<i32>() {
    //     Ok(num) => println!("You entered: {}", num),
    //     Err(_) => println!("That's not a valid number!"),
    // }
    match input.trim().parse::<i32>() {
        Ok(num) => {
            println!("You entered: {}", num);
        }
        Err(_) => {
            println!("That's not a valid number!");
        }
    }
}
