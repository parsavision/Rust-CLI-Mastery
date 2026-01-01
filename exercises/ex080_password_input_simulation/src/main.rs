use std::io::{self, Write};

const MAX_ATTEMPTS: u8 = 3;

fn main() {
    println!("Hi, Welcome!");
    let mut counter = 0;
    loop {
        print!("Enter your password :");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read line");
        if password.trim() == "secret" {
            println!("Access granted!");
            break;
        } else if password.trim() == "admin" {
            println!("Admin access granted!");
            break;
        } else {
            counter += 1;
            if password.trim() == "" {
                println!("Password cannot be empty!");
            }
            if counter == MAX_ATTEMPTS {
                println!("Too many attempts!");
                break;
            }

            println!("Access denied!");
            if counter == MAX_ATTEMPTS {
                println!("Too many attempts!");
                break;
            }
            println!("remaining attempts {}", MAX_ATTEMPTS - counter);
        }
    }
}
