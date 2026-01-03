# Exercise 97: Progress Indicator

## 🎯 Goal
Show progress text

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, Simple)
```rust
// Simple progress text indicator
use std::thread;
use std::time::Duration;

fn main() {
    println!("Processing...");
    
    for i in 1..=10 {
        print!("."); // Print dot for each step
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(200));
    }
    
    println!("\nDone!");
}
```

**Why this works:**
- Simple visual feedback
- No dependencies
- Easy to understand

**Trade-offs:**
- ⚠️ No percentage or time estimate
- ⚠️ Unwrap can panic
- ⚠️ Basic user experience

### Idiomatic Approach (Safe, Professional)
```rust
// Progress with percentage and proper flushing
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn show_progress(current: usize, total: usize) -> io::Result<()> {
    let percent = (current * 100) / total;
    print!("\rProgress: {}% [{}/{}]", percent, current, total);
    io::stdout().flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let total = 10;
    
    for i in 1..=total {
        show_progress(i, total)?;
        thread::sleep(Duration::from_millis(200));
    }
    
    println!("\nComplete!");
    Ok(())
}
```

**Why this is better:**
- ✅ Real-time progress updates
- ✅ Percentage and fraction shown
- ✅ Proper error handling

**Key Insight:**
Progress indicators - always flush output and provide helpful feedback.

---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern
```rust
// Progress without flushing output
use std::thread;
use std::time::Duration;

fn main() {
    for i in 1..=10 {
        print!("."); // No flush! Won't show until newline
        thread::sleep(Duration::from_millis(200));
    }
    println!("Done!");
    // User sees nothing until very end - looks frozen!
}
```

**Why it's dangerous:**
1. Output is buffered, not shown to user
2. Program appears frozen
3. Poor user experience

### ✅ Safe Alternative
```rust
// Progress with percentage and proper flushing
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn show_progress(current: usize, total: usize) -> io::Result<()> {
    let percent = (current * 100) / total;
    print!("\rProgress: {}% [{}/{}]", percent, current, total);
    io::stdout().flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let total = 10;
    
    for i in 1..=total {
        show_progress(i, total)?;
        thread::sleep(Duration::from_millis(200));
    }
    
    println!("\nComplete!");
    Ok(())
}
```

**Why this is better:**
- Real-time progress updates
- Percentage and fraction shown
- Proper error handling
- Professional user feedback

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

Show progress text

**Requirements:**
- Implement the core functionality
- Handle errors gracefully
- Provide helpful user feedback
- Follow Rust best practices

**Example Usage:**
```bash
$ cargo run
Processing...
..........
Done!
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

1. **Progress indicators** - Core concept for this exercise
2. **Flush output** - Always flush for real-time updates
3. **User-friendly** - Provide clear feedback
4. **Idiomatic patterns** - Follow Rust conventions

---

## 📖 Further Reading

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

---

**[← Previous Exercise](../ex096_color_output_preview/README.md) | [Next Exercise →](../ex098_clear_screen_simulation/README.md)**
