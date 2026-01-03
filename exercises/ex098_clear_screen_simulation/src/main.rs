fn main() {
    println!("Exercise 98: Clear Screen Simulation");
    println!("Understand screen control");
    println!();
    
    // TODO: "Clear" screen by printing many newlines
    // Real clear uses ANSI codes
    
    for _ in 0..50 {
        println!();
    }
    
    println!("Screen 'cleared'!");
}
