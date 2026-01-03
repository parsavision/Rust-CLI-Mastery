# Exercise 92: Decimal Places

## 🎯 Goal
Control precision

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, Simple)
```rust
let pi = 3.14159265359;
println!("{}", pi);  // Output: 3.14159265359
```

**Why this works:**
- Shows all decimal places
- Simple

**Trade-offs:**
- ⚠️ Too many decimals for display
- ⚠️ Not suitable for currency
- ⚠️ No control over precision

### Idiomatic Approach (Safe, Professional)
```rust
let pi = 3.14159265359;

println!("{:.2}", pi);   // Output: 3.14
println!("{:.4}", pi);   // Output: 3.1416
println!("{:.0}", pi);   // Output: 3

// For currency
let price = 19.995;
println!("${:.2}", price);  // Output: $20.00 (rounds!)
```

**Why this is better:**
- ✅ Precision control
- ✅ Automatic rounding
- ✅ Clean output

**Key Insight:**
The `:.N` format specifier rounds to N decimal places. Essential for displaying currency, percentages, and scientific values.
---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern
```rust
let price = 19.99;
println!("${}", price);  // Output: $19.99 sometimes $19.990000001
```

**Why it's dangerous:**
1. Floating point precision issues
2. Inconsistent output
3. Not suitable for money

### ✅ Safe Alternative
```rust
println!("${:.2}", price);  // Always: $19.99
```

**Why this is better:**
- Consistent precision
- Rounds properly
- Professional display
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

Print float with 2 decimal places

**Requirements:**
- Implement the core functionality
- Handle errors gracefully
- Provide helpful user feedback
- Follow Rust best practices

**Example Usage:**
```bash
$ cargo run
Exercise 92: Decimal Places
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

1. **{:.2} format specifier** - Core concept for this exercise
2. **Safety first** - Always handle errors explicitly
3. **User-friendly** - Provide clear feedback
4. **Idiomatic patterns** - Follow Rust conventions

---

## 📖 Further Reading

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

---

 **[← Previous Exercise](../ex091_format_numbers/README.md) | [Next Exercise →](../ex093_padding_numbers/README.md)**
