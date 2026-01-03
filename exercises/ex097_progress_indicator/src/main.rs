use std::thread;
use std::time::Duration;

fn main() {
    println!("Exercise 97: Progress Indicator");
    println!("Show progress text");
    println!();
    
    // TODO: Print dots while "processing"
    print!("Processing");
    for _ in 0..5 {
        print!(".");
        std::io::Write::flush(&mut std::io::stdout()).ok();
        thread::sleep(Duration::from_millis(500));
    }
    println!(" Done!");
}
