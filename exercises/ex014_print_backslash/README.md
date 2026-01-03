# Exercise 14: Print Backslash ✅

[← Previous Exercise](../ex013_print_quote_marks/README.md) | [Next Exercise →](../ex015_unicode_characters/README.md)

## 🎯 Goal

Print a single backslash character using escape sequences.

## 📝 Task

Print a single backslash: `\`

## 🎓 Concept

The backslash character is special in Rust strings because it starts escape sequences. To print a literal backslash, you need to escape it: `\\`

- `\` - Escape sequence starter (special character)
- `\\` - Literal backslash character

## ✅ Solution Approach

```rust
fn main() {
    println!("\\");
}
```

**Output:**
```
\
```

## 💡 Why Double Backslash?

**The Logic:**
1. First `\` tells Rust: "Here comes an escape sequence"
2. Second `\` tells Rust: "The escape sequence is 'backslash'"

**Common escape sequences:**
```rust
println!("\\");   // Prints: \
println!("\"");   // Prints: "
println!("\n");   // Prints: newline
println!("\t");   // Prints: tab
```

## 🔄 Try These

```rust
// Backslash in text
println!("Path: C:\\Users\\John");

// Multiple backslashes
println!("\\\\\\\\"); // Prints: \\\\

// Mixed escapes
println!("Quote: \" and Backslash: \\");

// File path example
println!("Config file: C:\\\\config.ini");
```

## ⚡ Common Mistakes

```rust
// ❌ Single backslash (syntax error)
println!("\");

// ❌ Forward slash
println!("/");

// ❌ Triple backslash
println!("\\\");

// ✅ Correct: double backslash
println!("\\");
```

## 🎯 Success Criteria

- [ ] Program prints a single backslash
- [ ] Uses `\\` escape sequence correctly
- [ ] Output shows only one backslash
- [ ] Compiles without errors
- [ ] Understands why double backslash is needed

---

[← Previous Exercise](../ex013_print_quote_marks/README.md) | [Next Exercise →](../ex015_unicode_characters/README.md)