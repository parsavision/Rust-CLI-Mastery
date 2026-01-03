# Exercise 86: Confirm Action

## 🎯 Goal
Learn the Y/N confirmation pattern commonly used in CLI applications for destructive operations.

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, Simple)
```rust
let mut input = String::new();
io::stdin().read_line(&mut input).ok();

if input.trim() == "y" {
    println!("Confirmed!");
}
```

**Why this works:**
- Simple string comparison
- Easy to understand

**Trade-offs:**
- ⚠️ **Case-sensitive** - "Y" won't match "y"
- ⚠️ **Silent on invalid input** - "maybe" does nothing
- ⚠️ **No validation** - unclear what happens otherwise

### Idiomatic Approach (Safe, Professional)
```rust
fn confirm(prompt: &str) -> bool {
    print!("{} (y/n): ", prompt);
    io::stdout().flush().ok();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

// Usage:
if confirm("Delete file?") {
    println!("Deleting...");
} else {
    println!("Cancelled");
}
```

**Why this is better:**
- ✅ **Case-insensitive** - accepts "Y", "y", "Yes", "yes"
- ✅ **Reusable function** - works for any confirmation
- ✅ **Clear return type** - returns bool
- ✅ **Explicit handling** - caller decides what to do with false
- ✅ **Uses `matches!` macro** - idiomatic pattern matching

**Key Insight:**
Confirmation prompts should be user-friendly (case-insensitive, flexible) and reusable (extracted as functions). The `matches!` macro makes checking multiple valid responses concise.

---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern #1: Case-Sensitive Comparison
```rust
if input.trim() == "y" {  // ⚠️ "Y" won't match!
```

**Why it's dangerous:**
- Users naturally type "Y" (capital)
- Creates confusion - confirmation appears to fail
- Inconsistent with user expectations

### ⚠️ Risky Pattern #2: No Validation
```rust
if input.trim() == "y" {
    delete();
}
// What if user types "maybe" or just hits Enter?
// Silent failure - unclear what happened
```

### ✅ Safe Alternative
```rust
match input.trim().to_lowercase().as_str() {
    "y" | "yes" => println!("Confirmed!"),
    "n" | "no" => println!("Cancelled"),
    _ => println!("Please answer y or n"),
}
```

**Why this is better:**
- Accepts multiple valid responses
- Handles invalid input explicitly
- User gets feedback on what went wrong

---

## 🔍 Modern Rust: Clippy Insights

```bash
$ cargo clippy

warning: this match could be written with `matches!`
   |
   = help: consider using `matches!(input, "y" | "yes")`
```

**What Clippy teaches:**
- **`match_like_matches_macro`**: Use `matches!` for simple boolean pattern checks
- Cleaner, more idiomatic code
- Better intent communication

### Clippy Lints for This Exercise
- `match_like_matches_macro`: Suggests using `matches!`
- `case_sensitive_file_extension_comparisons`: Similar concept applies to strings

---

## 💪 Progressive Challenges

### Challenge 1: Case-Insensitive Matching ⏱️ 5-10 minutes
**Tasks:**
- [ ] Accept both "y" and "Y"
- [ ] Use `.to_lowercase()` for normalization
- [ ] Test with: "y", "Y", "n", "N"

### Challenge 2: Accept Multiple Responses ⏱️ 10-15 minutes
**Tasks:**
- [ ] Accept "y", "yes", "Y", "Yes", "YES"
- [ ] Accept "n", "no", "N", "No", "NO"
- [ ] Use `match` with multiple patterns
- [ ] Handle invalid input with helpful message

### Challenge 3: Reusable Function ⏱️ 15-25 minutes
**Tasks:**
- [ ] Create `fn confirm(prompt: &str) -> bool`
- [ ] Add retry loop for invalid input
- [ ] Use `matches!` macro for clean code
- [ ] Add documentation comments

**Example:**
```rust
/// Prompts user for yes/no confirmation.
/// Accepts: y, yes, Y, Yes, YES (true) | n, no, N, No, NO (false)
/// Loops until valid input received.
fn confirm(prompt: &str) -> bool {
    loop {
        print!("{} (y/n): ", prompt);
        io::stdout().flush().ok();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please answer 'y' or 'n'"),
        }
    }
}
```

### Challenge 4: Testing ⏱️ 20-30 minutes
**Tasks:**
- [ ] Add unit tests (need to mock stdin - advanced)
- [ ] Test the normalization logic separately
- [ ] Add examples in documentation

---

## 📝 Task

Create a Y/N confirmation prompt for a CLI action.

**Requirements:**
- Ask "Are you sure? (y/n)"
- Accept user response
- Handle 'y' as confirmed, 'n' as cancelled
- Make it case-insensitive

**Example Usage:**
```bash
$ cargo run
Are you sure? (y/n): y
Confirmed!

$ cargo run
Are you sure? (y/n): N
Cancelled
```

---

## 💡 Key Takeaways

1. **Confirmations prevent mistakes** - essential for destructive operations
2. **Be user-friendly** - accept multiple valid responses, case-insensitive
3. **Extract to functions** - confirmation logic is highly reusable
4. **Use `matches!` macro** - cleaner than verbose match expressions
5. **Provide feedback** - always tell user what happened

---

## 📖 Further Reading

- [std::matches macro](https://doc.rust-lang.org/std/macro.matches.html)
- [String::to_lowercase()](https://doc.rust-lang.org/std/string/struct.String.html#method.to_lowercase)
- [CLI UX best practices](https://clig.dev/)

---

**[← Previous Exercise](../ex085_error_recovery/README.md) | [Next Exercise →](../ex087_get_float_input/README.md)**
