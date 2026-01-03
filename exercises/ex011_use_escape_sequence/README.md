# Exercise 11: Print Special Characters ✅

[← Previous Exercise](../ex010_run_without_cargo/README.md) | [Next Exercise →](../ex012_add_indentation/README.md)

## 🎯 Goal

Use escape sequences to print special characters like newlines and tabs.

## 📝 Task

Print "Hello\nWorld" to get two lines, demonstrating escape sequences.

## 🎓 Concept

Escape sequences start with a backslash (`\`) and represent special characters:

- `\n` - Newline character
- `\t` - Tab character  
- `\"` - Literal quote mark
- `\\` - Literal backslash

## ✅ Solution Approach

```rust
// println! with escape sequences
fn main() {
    println!("Hello\nWorld");
}
```

**Output:**
```
Hello
World
```

## 💡 Why Escape Sequences?

Without escape sequences, we can't represent:
- Line breaks within strings
- Tab indentation
- Quote marks inside quoted text
- Backslash characters themselves

**Instead of:** Multiple println! calls  
**Use:** Escape sequences for cleaner code

## 🔄 Try These

```rust
println!("Line 1\nLine 2");           // Two lines
println!("Col1\tCol2\tCol3");         // Tabs for alignment
println!("He said \"Hello!\"");       // Quoted text
println!("Path: C:\\\\Windows\\\\");   // Backslashes
```

## ⚡ Common Mistakes

```rust
// ❌ Missing backslash
println!("HellonWorld");

// ❌ Forward slash instead of backslash  
println!("Hello/nWorld");

// ✅ Correct escape sequence
println!("Hello\nWorld");
```

## 🎯 Success Criteria

- [ ] Program prints "Hello" and "World" on separate lines
- [ ] Uses `\n` escape sequence
- [ ] Compiles without errors
- [ ] Understands at least 2 escape sequences

---

[← Previous Exercise](../ex010_run_without_cargo/README.md) | [Next Exercise →](../ex012_add_indentation/README.md)