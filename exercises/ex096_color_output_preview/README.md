# Exercise 96: Color Output Preview

## 🎯 Goal
See ANSI colors

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, Simple)
```rust
// Simple colored output (hardcoded ANSI codes)
fn main() {
    // Red text
    println!("\x1b[31mError: Something went wrong\x1b[0m");
    
    // Green text
    println!("\x1b[32mSuccess!\x1b[0m");
    
    // Blue text
    println!("\x1b[34mInfo: Processing...\x1b[0m");
    
    // Bold yellow text
    println!("\x1b[1;33mWarning: Check this\x1b[0m");
}
```

**Why this works:**
- Direct ANSI escape codes
- Simple and immediate
- No dependencies

**Trade-offs:**
- ⚠️ Doesn't check if terminal supports color
- ⚠️ No fallback for non-color terminals
- ⚠️ Manual escape codes are error-prone

### Idiomatic Approach (Safe, Professional)
```rust
// Check terminal support before using colors
fn supports_color() -> bool {
    // Check if stdout is a terminal (not piped/redirected)
    std::io::IsTerminal::is_terminal(&std::io::stdout())
}

fn print_colored(color: &str, msg: &str) {
    if supports_color() {
        println!("{}{}\x1b[0m", color, msg);
    } else {
        println!("{}", msg); // Plain text fallback
    }
}

fn main() {
    print_colored("\x1b[31m", "Error: This degrades gracefully");
    print_colored("\x1b[32m", "Success!");
}
```

**Why this is better:**
- ✅ Detects terminal capabilities
- ✅ Graceful fallback to plain text
- ✅ Works in all environments

**Key Insight:**
Terminal colors - always provide graceful degradation for unsupported terminals.

---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern
```rust
// Assuming all terminals support color
fn print_error(msg: &str) {
    println!("\x1b[31m{}\x1b[0m", msg);
    // No check if terminal supports ANSI codes!
}

fn main() {
    print_error("This might not work in all terminals");
    // Windows CMD, piped output, etc. might show escape codes as text
}
```

**Why it's dangerous:**
1. Output is garbled on unsupported terminals
2. No fallback mechanism
3. Poor user experience in some environments

### ✅ Safe Alternative
```rust
// Check terminal support before using colors
fn supports_color() -> bool {
    std::io::IsTerminal::is_terminal(&std::io::stdout())
}

fn print_colored(color: &str, msg: &str) {
    if supports_color() {
        println!("{}{}\x1b[0m", color, msg);
    } else {
        println!("{}", msg);
    }
}

fn main() {
    print_colored("\x1b[31m", "Error: This degrades gracefully");
    print_colored("\x1b[32m", "Success!");
}
```

**Why this is better:**
- Detects terminal capabilities
- Graceful fallback to plain text
- Works in all environments
- Professional output handling

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

Print colored text (just concept)

**Requirements:**
- Implement the core functionality
- Handle errors gracefully
- Provide helpful user feedback
- Follow Rust best practices

**Example Usage:**
```bash
$ cargo run
Exercise 96: Color Output Preview
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

1. **Terminal colors** - Core concept for this exercise
2. **Safety first** - Always handle errors explicitly
3. **User-friendly** - Provide clear feedback
4. **Idiomatic patterns** - Follow Rust conventions

---

## 📖 Further Reading

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

---

**[← Previous Exercise](../ex095_print_percentage/README.md) | [Next Exercise →](../ex097_progress_indicator/README.md)**
