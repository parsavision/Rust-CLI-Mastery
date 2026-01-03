# Exercise 98: Clear Screen Simulation

## 🎯 Goal
Understand screen control

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, Simple)
```rust
// Simple screen "clear" with newlines
fn main() {
    // Print many blank lines to push content up
    for _ in 0..50 {
        println!();
    }
    
    println!("Screen is 'cleared'!");
    println!("This appears at the top");
}
```

**Why this works:**
- No dependencies needed
- Works everywhere
- Simple to understand

**Trade-offs:**
- ⚠️ Not a true clear (old content still in scrollback)
- ⚠️ Hardcoded line count
- ⚠️ Doesn't work well on all terminals

### Idiomatic Approach (Safe, Professional)
```rust
// Detect terminal and use appropriate method
use std::io::{self, IsTerminal, Write};

fn clear_screen() -> io::Result<()> {
    if io::stdout().is_terminal() {
        // On real terminal, use ANSI escape
        print!("\x1b[2J\x1b[H");
        io::stdout().flush()?;
    } else {
        // If piped/redirected, use newlines
        for _ in 0..50 {
            println!();
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    clear_screen()?;
    println!("This works in all environments!");
    Ok(())
}
```

**Why this is better:**
- ✅ Detects terminal capabilities
- ✅ Graceful fallback
- ✅ Works when piped or redirected

**Key Insight:**
Screen control - always detect terminal capabilities and provide fallbacks.

---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern
```rust
// Assuming ANSI clear works everywhere
fn main() {
    // ANSI escape code to clear screen
    print!("\x1b[2J\x1b[H");
    // Might not work on Windows CMD, or if output is redirected
    
    println!("Screen cleared?");
}
```

**Why it's dangerous:**
1. Doesn't work on all terminals
2. Breaks when output is piped/redirected
3. No fallback mechanism

### ✅ Safe Alternative
```rust
// Detect terminal and use appropriate method
use std::io::{self, IsTerminal, Write};

fn clear_screen() -> io::Result<()> {
    if io::stdout().is_terminal() {
        print!("\x1b[2J\x1b[H");
        io::stdout().flush()?;
    } else {
        for _ in 0..50 {
            println!();
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    clear_screen()?;
    println!("This works in all environments!");
    Ok(())
}
```

**Why this is better:**
- Detects terminal capabilities
- Graceful fallback
- Works when piped or redirected
- Proper error handling

---

## 🔍 Modern Rust: Clippy Insights

Run `cargo clippy` to see helpful suggestions.

**Relevant lints for this exercise:**
- `print_stdout`

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

Print many newlines

**Requirements:**
- Implement the core functionality
- Handle errors gracefully
- Provide helpful user feedback
- Follow Rust best practices

**Example Usage:**
```bash
$ cargo run
Screen is 'cleared'!
This appears at the top
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

1. **Screen control** - Core concept for this exercise
2. **Terminal detection** - Check capabilities before using features
3. **User-friendly** - Provide clear feedback
4. **Idiomatic patterns** - Follow Rust conventions

---

## 📖 Further Reading

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

---

**[← Previous Exercise](../ex097_progress_indicator/README.md) | [Next Exercise →](../ex099_input_output_review/README.md)**
