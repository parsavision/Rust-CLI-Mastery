# Exercise 90: Complete Input Library

## 🎯 Goal
Master all input types

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, Simple)
```rust
use std::io;

fn get_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    input.trim().to_string()
}

fn get_i32() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    input.trim().parse().expect("Not a number")
}

// Use them
let name = get_string();
let age = get_i32();
```

**Why this works:**
- Separate function for each type
- Easy to use
- Clear intent

**Trade-offs:**
- ⚠️ get_i32() panics on invalid input
- ⚠️ No prompts or error messages
- ⚠️ No retry logic

### Idiomatic Approach (Safe, Professional)
```rust
use std::io::{self, Write};

pub struct Input;

impl Input {
    pub fn get_string(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().ok();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        input.trim().to_string()
    }
    
    pub fn get<T>(prompt: &str) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        loop {
            print!("{}", prompt);
            io::stdout().flush().ok();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).ok();
            
            match input.trim().parse::<T>() {
                Ok(value) => return value,
                Err(e) => println!("Invalid input: {}. Try again.", e),
            }
        }
    }
}

// Usage
let name = Input::get_string("Name: ");
let age: i32 = Input::get("Age: ");
let price: f64 = Input::get("Price: ");
let active: bool = Input::get("Active (true/false): ");
```

**Why this is better:**
- ✅ Single generic function for all types
- ✅ Never panics - retry loop
- ✅ Clear error messages
- ✅ Clean API
- ✅ Production-ready

**Key Insight:**
A well-designed input library is one of the most useful tools for CLI development. Invest time here and reuse it in every project.
---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern
```rust
fn get_number<T: FromStr>() -> T {
    let mut input = String::new();
    stdin().read_line(&mut input).ok();
    input.trim().parse().unwrap()  // ⚠️ Panics!
}
```

**Why it's dangerous:**
1. Generic but unsafe
2. No error handling
3. No user feedback

### ✅ Safe Alternative
```rust
fn get_number<T: FromStr>(prompt: &str) -> T 
where T::Err: std::fmt::Display 
{
    loop {
        print!("{}", prompt);
        flush();
        
        let mut input = String::new();
        stdin().read_line(&mut input).ok();
        
        match input.trim().parse() {
            Ok(n) => return n,
            Err(e) => println!("Error: {}", e),
        }
    }
}
```

**Why this is better:**
- Generic AND safe
- Retry on error
- Shows parse errors to user
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

Create functions for: get_string, get_i32, get_f64, get_bool, get_char

**Requirements:**
- Implement the core functionality
- Handle errors gracefully
- Provide helpful user feedback
- Follow Rust best practices

**Example Usage:**
```bash
$ cargo run
Exercise 90: Complete Input Library
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

1. **Reusable input helpers** - Core concept for this exercise
2. **Safety first** - Always handle errors explicitly
3. **User-friendly** - Provide clear feedback
4. **Idiomatic patterns** - Follow Rust conventions

---

## 📖 Further Reading

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

---

 **[← Previous Exercise](../ex089_input_with_default/README.md) | [Next Exercise →](../ex091_format_numbers/README.md)**
