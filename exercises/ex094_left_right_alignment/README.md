# Exercise 94: Left Right Alignment

## 🎯 Goal
Format tables

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, Simple)
```rust
println!("Apple 1.50");
println!("Banana 0.75");
// Not aligned
```

### Idiomatic Approach (Safe, Professional)
```rust
println!("{:<10} {:>6.2}", "Apple", 1.50);
println!("{:<10} {:>6.2}", "Banana", 0.75);
// Output:
// Apple       $1.50
// Banana      $0.75
```

**Key Insight:**
`{:<N}` = left align, `{:>N}` = right align, `{:^N}` = center
---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern
```rust
println!("{} {}", item, price);  // Jagged
```

### ✅ Safe Alternative
```rust
println!("{:<15} ${:>8.2}", item, price);  // Aligned
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

Use {:<10} and {:>10}

**Requirements:**
- Implement the core functionality
- Handle errors gracefully
- Provide helpful user feedback
- Follow Rust best practices

**Example Usage:**
```bash
$ cargo run
Exercise 94: Left Right Alignment
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

1. **Text alignment in output** - Core concept for this exercise
2. **Safety first** - Always handle errors explicitly
3. **User-friendly** - Provide clear feedback
4. **Idiomatic patterns** - Follow Rust conventions

---

## 📖 Further Reading

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

---

 **[← Previous Exercise](../ex093_padding_numbers/README.md) | [Next Exercise →](../ex095_print_percentage/README.md)**
