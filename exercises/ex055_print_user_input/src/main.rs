use std::io::{self, Write};
fn main() {
    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}
