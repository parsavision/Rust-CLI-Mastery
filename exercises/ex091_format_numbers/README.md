# Exercise 91: Format Numbers

## 🎯 Goal
Pretty number output

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, Simple)
```rust
let number = 1234567;
println!("{}", number);  // Output: 1234567
```

**Why this works:**
- Simple and direct
- No formatting needed for small numbers

**Trade-offs:**
- ⚠️ Large numbers hard to read
- ⚠️ No thousand separators
- ⚠️ Not user-friendly for big values

### Idiomatic Approach (Safe, Professional)
```rust
fn format_with_commas(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    
    result.chars().rev().collect()
}

let number = 1234567;
println!("{}", format_with_commas(number));  // Output: 1,234,567
```

**Why this is better:**
- ✅ Easy to read large numbers
- ✅ Standard formatting
- ✅ Works for any size

**Key Insight:**
Rust doesn't have built-in thousand separators in format strings, but they're easy to implement and greatly improve readability.
---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern
```rust
// Trying to use format! for separators
println!("{:,}", 1234567);  // ⚠️ Doesn't work! Not valid syntax
```

**Why it's wrong:**
1. Rust doesn't support :, format specifier
2. Will cause compile error
3. Need manual implementation

### ✅ Safe Alternative
```rust
// Manual but correct
fn add_commas(n: i64) -> String {
    n.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<_>>()
        .join(",")
}
```

**Why this is better:**
- Actually works
- Functional style
- Concise
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

Print number with thousand separators

**Requirements:**
- Implement the core functionality
- Handle errors gracefully
- Provide helpful user feedback
- Follow Rust best practices

**Example Usage:**
```bash
$ cargo run
Exercise 91: Format Numbers
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

1. **Format strings** - Core concept for this exercise
2. **Safety first** - Always handle errors explicitly
3. **User-friendly** - Provide clear feedback
4. **Idiomatic patterns** - Follow Rust conventions

---

## 📖 Further Reading

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

---

 **[← Previous Exercise](../ex090_complete_input_library/README.md) | [Next Exercise →](../ex092_decimal_places/README.md)**
