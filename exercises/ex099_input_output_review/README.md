# Exercise 99: Input/Output Review

## 🎯 Goal
Build interactive program

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, Simple)
```rust
// Review of basic I/O patterns
use std::io;

fn main() {
    println!("=== Input/Output Review ===");
    
    // Get string input
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read");
    let name = name.trim();
    
    // Get number input
    println!("Enter your age:");
    let mut age_str = String::new();
    io::stdin().read_line(&mut age_str).expect("Failed to read");
    let age: i32 = age_str.trim().parse().expect("Not a number");
    
    // Formatted output
    println!("\nHello, {}!", name);
    println!("You are {} years old", age);
    println!("In 10 years you'll be {}", age + 10);
}
```

**Why this works:**
- Demonstrates core I/O concepts
- Simple and readable
- Combines string and number input

**Trade-offs:**
- ⚠️ Multiple expect() calls can panic
- ⚠️ No input validation
- ⚠️ Basic error messages

### Idiomatic Approach (Safe, Professional)
```rust
// Comprehensive I/O with proper error handling
use std::io::{self, Write};

fn get_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn get_number(prompt: &str) -> Result<i32, String> {
    let input = get_input(prompt)
        .map_err(|e| format!("Input error: {}", e))?;
    
    input.parse::<i32>()
        .map_err(|_| format!("'{}' is not a valid number", input))
}

fn main() -> Result<(), String> {
    println!("=== Safe Input/Output Review ===");
    
    let name = get_input("Enter your name: ")?;
    
    let age = loop {
        match get_number("Enter your age (1-120): ") {
            Ok(n) if n >= 1 && n <= 120 => break n,
            Ok(n) => println!("Age {} is out of range. Try again.", n),
            Err(e) => println!("{}. Try again.", e),
        }
    };
    
    println!("\n✅ Hello, {}! You are {} years old.", name, age);
    println!("In 10 years you'll be {}", age + 10);
    
    Ok(())
}
```

**Why this is better:**
- ✅ All errors handled explicitly
- ✅ Input validation with retry logic
- ✅ Helpful error messages
- ✅ Never panics on bad input

**Key Insight:**
Input/Output - always validate input and handle all error cases gracefully.

---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern
```rust
// Chaining operations without error handling
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num = input.trim().parse::<i32>().unwrap();
    let result = 100 / num; // Three panic points!
    println!("Result: {}", result);
}
```

**Why it's dangerous:**
1. Three separate panic points (read, parse, divide)
2. No validation of input
3. Division by zero possible
4. Poor error messages

### ✅ Safe Alternative
```rust
// Comprehensive I/O with proper error handling
use std::io::{self, Write};

fn get_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn get_number(prompt: &str) -> Result<i32, String> {
    let input = get_input(prompt)
        .map_err(|e| format!("Input error: {}", e))?;
    
    input.parse::<i32>()
        .map_err(|_| format!("'{}' is not a valid number", input))
}

fn main() -> Result<(), String> {
    println!("=== Safe Input/Output Review ===");
    
    let name = get_input("Enter your name: ")?;
    
    let age = loop {
        match get_number("Enter your age (1-120): ") {
            Ok(n) if n >= 1 && n <= 120 => break n,
            Ok(n) => println!("Age {} is out of range. Try again.", n),
            Err(e) => println!("{}. Try again.", e),
        }
    };
    
    println!("\n✅ Hello, {}! You are {} years old.", name, age);
    println!("In 10 years you'll be {}", age + 10);
    
    Ok(())
}
```

**Why this is better:**
- All errors handled explicitly
- Input validation with retry logic
- Helpful error messages
- Never panics on bad input
- Professional user experience

---

## 🔍 Modern Rust: Clippy Insights

Run `cargo clippy` to see helpful suggestions.

**Relevant lints for this exercise:**
- `expect_used`

**What Clippy teaches:**
Use safe patterns and explicit error handling throughout your code.

---

## 💪 Progressive Challenges

### Challenge 1: Eliminate Panics ⏱️ 5-10 minutes
**Goal:** Make your code panic-free

**Tasks:**
- [ ] Replace .unwrap() with .unwrap_or() or match
- [ ] Handle all error cases explicitly
- [ ] Test with edge cases

### Challenge 2: Better Error Handling ⏱️ 10-15 minutes
**Goal:** Provide helpful error messages

**Tasks:**
- [ ] Return Result<T, String> from functions
- [ ] Provide specific error messages
- [ ] Handle edge cases gracefully

### Challenge 3: Idiomatic Rust ⏱️ 15-25 minutes
**Goal:** Write professional code

**Tasks:**
- [ ] Use ? operator for error propagation
- [ ] Follow clippy suggestions
- [ ] Add documentation comments
- [ ] Pass cargo clippy with zero warnings

### Challenge 4: Testing & Polish ⏱️ 20-30 minutes (Optional)
**Goal:** Production-ready code

**Tasks:**
- [ ] Add unit tests for valid inputs
- [ ] Test error cases
- [ ] Test edge cases
- [ ] Achieve >80% code coverage

---

## 📝 Task

Program that asks 3 questions, does calculation, shows result

**Requirements:**
- Implement the core functionality
- Handle errors gracefully
- Provide helpful user feedback
- Follow Rust best practices

**Example Usage:**
```bash
$ cargo run
=== Input/Output Review ===
Enter your name: Alice
Enter your age: 25

Hello, Alice! You are 25 years old.
In 10 years you'll be 35
```

---

## 🧪 Testing Your Solution

```bash
# Build and run
cargo run

# Run tests
cargo test

# Check for issues
cargo clippy

# Format code
cargo fmt
```

---

## 💡 Key Takeaways

1. **Input/Output** - Core concept for this exercise
2. **Safety first** - Always handle errors explicitly
3. **User-friendly** - Provide clear feedback
4. **Idiomatic patterns** - Follow Rust conventions

---

## 📖 Further Reading

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

---

**[← Previous Exercise](../ex098_clear_screen_simulation/README.md) | [Next Exercise →](../ex100_mini_calculator/README.md)**
