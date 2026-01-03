# Exercise 95: Print Percentage

## 🎯 Goal
Format as percent

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, Simple)
```rust
let ratio = 0.85;
println!("{}%", ratio * 100.0);  // 85%
```

### Idiomatic Approach (Safe, Professional)
```rust
let ratio = 0.8533;
println!("{:.1}%", ratio * 100.0);  // 85.3%
println!("{:.0}%", ratio * 100.0);  // 85%
```

**Key Insight:**
Multiply by 100, format with desired precision, add % symbol.
---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern
```rust
println!("{}%", 0.85);  // "0.85%" (wrong!)
```

### ✅ Safe Alternative
```rust
println!("{:.0}%", 0.85 * 100.0);  // "85%"
```
---

## 🔍 Modern Rust: Clippy Insights

Run `cargo clippy` to see helpful suggestions.

**Relevant lints for this exercise:**
- `print_stdout`
- `use_debug`

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

Convert 0.85 to 85%

**Requirements:**
- Implement the core functionality
- Handle errors gracefully
- Provide helpful user feedback
- Follow Rust best practices

**Example Usage:**
```bash
$ cargo run
Exercise 95: Print Percentage
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

1. **Math and formatting** - Core concept for this exercise
2. **Safety first** - Always handle errors explicitly
3. **User-friendly** - Provide clear feedback
4. **Idiomatic patterns** - Follow Rust conventions

---

## 📖 Further Reading

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

---

 **[← Previous Exercise](../ex094_left_right_alignment/README.md) | [Next Exercise →](../ex096_color_output_preview/README.md)**
