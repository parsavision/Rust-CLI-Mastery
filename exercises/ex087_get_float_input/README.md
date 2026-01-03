# Exercise 87: Get Float Input

## 🎯 Goal
Parse decimal numbers

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, Simple)
```rust
use std::io;

let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read");

let number = input.trim().parse::<f64>().expect("Not a number");
println!("You entered: {}", number);
```

**Why this works:**
- Direct parsing with `parse::<f64>()`
- `.expect()` provides error message if parsing fails
- Easy to understand the flow

**Trade-offs:**
- ⚠️ **Panics on invalid input** - "abc" crashes the program
- ⚠️ **Panics on empty input** - just hitting Enter crashes
- ⚠️ **No recovery** - user has to restart the program
- ⚠️ **Poor UX** - crashes instead of asking again

### Idiomatic Approach (Safe, Professional)
```rust
use std::io::{self, Write};

fn get_float(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().ok();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid decimal number"),
        }
    }
}

// Usage:
let value = get_float("Enter a decimal number: ");
println!("You entered: {:.2}", value);
```

**Why this is better:**
- ✅ **Never panics** - uses `match` to handle errors
- ✅ **Retry loop** - keeps asking until valid input
- ✅ **Reusable function** - works for any float input
- ✅ **User-friendly** - clear error messages
- ✅ **Production-ready** - handles all edge cases

**Key Insight:**
Parsing user input is inherently fallible. Always use `match` or `Result` handling, never `.unwrap()` or `.expect()` in production code that processes user input.

---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern
```rust
let input = "3.14abc";
let number = input.parse::<f64>().expect("Not a number");
// ⚠️ PANICS: thread 'main' panicked at 'Not a number: ParseFloatError'
```

**Why it's dangerous:**
1. **Crashes on any invalid input** - "abc", "", "3.14.15" all panic
2. **No recovery** - program terminates immediately
3. **Poor user experience** - users see scary error messages
4. **Can't retry** - user must restart entire program

**Real-world impact:**
```rust
// User types "3,14" (European decimal notation)
let num = input.trim().parse::<f64>().expect("Invalid");
// Program crashes! Should accept or guide user instead.
```

### ✅ Safe Alternative
```rust
match input.trim().parse::<f64>() {
    Ok(num) => {
        println!("Valid number: {}", num);
        // Continue processing
    }
    Err(e) => {
        println!("Error: '{}' is not a valid decimal number", input.trim());
        println!("Please try again (examples: 3.14, -2.5, 0.001)");
        // Ask again instead of crashing
    }
}
```

**Why this is better:**
- Returns control to caller on error
- Provides specific, helpful feedback
- Never crashes unexpectedly
- User can retry without restarting

---

## 🔍 Modern Rust: Clippy Insights

Run `cargo clippy` to see helpful suggestions.

**Relevant lints for this exercise:**
- `unwrap_used`
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

Parse input as f64

**Requirements:**
- Implement the core functionality
- Handle errors gracefully
- Provide helpful user feedback
- Follow Rust best practices

**Example Usage:**
```bash
$ cargo run
Exercise 87: Get Float Input
# Expected output based on task
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

1. **parse::<f64>()** - Core concept for this exercise
2. **Safety first** - Always handle errors explicitly
3. **User-friendly** - Provide clear feedback
4. **Idiomatic patterns** - Follow Rust conventions

---

## 📖 Further Reading

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

---

 **[← Previous Exercise](../ex086_confirm_action/README.md) | [Next Exercise →](../ex088_multiple_float_inputs/README.md)**
