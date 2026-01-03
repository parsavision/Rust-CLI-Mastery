# Exercise 89: Input With Default

## 🎯 Goal
Allow empty input

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, Simple)
```rust
use std::io;

let mut input = String::new();
io::stdin().read_line(&mut input).ok();

let value = if input.trim().is_empty() {
    10  // Default value
} else {
    input.trim().parse::<i32>().expect("Not a number")
};

println!("Value: {}", value);
```

**Why this works:**
- Simple if-else logic
- Checks for empty input
- Provides default

**Trade-offs:**
- ⚠️ Still panics on invalid (non-empty) input
- ⚠️ No user guidance
- ⚠️ Default value hardcoded

### Idiomatic Approach (Safe, Professional)
```rust
use std::io::{self, Write};

fn get_input_or_default<T: std::str::FromStr>(prompt: &str, default: T) -> T 
where
    T: std::fmt::Display,
{
    print!("{} [default: {}]: ", prompt, default);
    io::stdout().flush().ok();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    
    let trimmed = input.trim();
    if trimmed.is_empty() {
        default
    } else {
        trimmed.parse().unwrap_or(default)
    }
}

let count = get_input_or_default("Enter count", 10);
let price = get_input_or_default("Enter price", 9.99);
```

**Why this is better:**
- ✅ Generic function works for any type
- ✅ Shows default in prompt
- ✅ Never panics - returns default on parse error
- ✅ User-friendly and reusable

**Key Insight:**
Defaults make CLI tools more user-friendly. Always show the default value in the prompt so users know what happens if they just press Enter.
---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern
```rust
let input = read_line();
let num = if input.is_empty() {
    42
} else {
    input.parse().expect("Invalid")  // ⚠️ Still panics!
};
```

**Why it's dangerous:**
1. Empty input handled, but invalid input still crashes
2. User doesn't see what the default is
3. Incomplete safety

### ✅ Safe Alternative
```rust
let num = input.trim()
    .parse::<i32>()
    .unwrap_or(42);  // Returns default on ANY error
```

**Why this is better:**
- Handles both empty AND invalid input
- Never panics
- Concise and clear
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

If empty, use default value

**Requirements:**
- Implement the core functionality
- Handle errors gracefully
- Provide helpful user feedback
- Follow Rust best practices

**Example Usage:**
```bash
$ cargo run
Exercise 89: Input With Default
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

1. **Fallback values** - Core concept for this exercise
2. **Safety first** - Always handle errors explicitly
3. **User-friendly** - Provide clear feedback
4. **Idiomatic patterns** - Follow Rust conventions

---

## 📖 Further Reading

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

---

 **[← Previous Exercise](../ex088_multiple_float_inputs/README.md) | [Next Exercise →](../ex090_complete_input_library/README.md)**
