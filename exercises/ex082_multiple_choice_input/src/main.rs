use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    loop {
        print!(
            "What is the best programming language?\n1) python\n2) cpp\n3) rust\n4) javascript\nWhich one?(1,2,3,4)"
        );
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<u8>() {
            Ok(choice) => match choice {
                1 => {
                    println!();
                    println!("You mean rust? answer again.");
                    println!();
                    input.clear();
                }
                2 => {
                    println!();
                    println!("You mean rust? answer again.");
                    println!();
                    input.clear();
                }
                3 => {
                    println!();
                    println!("You finally make a right choice!");
                    break;
                }
                4 => {
                    println!();
                    println!("You mean rust? answer again.");
                    println!();
                    input.clear();
                }
                _ => {
                    println!();
                    println!("Invalid choice!");
                    println!();
                    input.clear();
                }
            },
            Err(_) => println!("Invalid input!"),
        }
    }
}
