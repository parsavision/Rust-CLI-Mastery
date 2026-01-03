# Exercise 15: Unicode Characters ✅⭐

[← Previous Exercise](../ex014_print_backslash/README.md) | [Next Exercise →](../ex016_comments_single_line/README.md)

## 🎯 Goal

Print Unicode characters (like emojis) that go beyond basic ASCII.

## 📝 Task

Print: "🦀 Rust" using Unicode emoji characters.

## 🎓 Concept

Unicode is Rust's default text encoding:
- **Supports:** All world languages, emojis, symbols
- **String type:** UTF-8 encoded by default
- **No special handling:** Just type or paste Unicode!

## ✅ Solution Approach

```rust
fn main() {
    println!("🦀 Rust");
}
```

**Output:**
```
🦀 Rust
```

## 💡 Why Unicode Matters

**Before Unicode:**
- Limited to English characters (ASCII)
- No emojis or special symbols
- Language barriers

**With Unicode:**
- Global applications
- Rich text expressions
- Cultural inclusivity
- Modern user experience

## 🔄 Try These

```rust
// Different emojis
println!("❤️ Heart");
println!("🌍 Earth");
println!("🚀 Rocket");
println!("⭐ Star");

// International text
println!("Привет"); // Russian "Hello"
println!("こんにちは"); // Japanese "Hello"
println!("🌮 Święcon"); // Polish tradition

// Math symbols
println!("∞ Infinity");
println!("∑ Summation");
println!("≠ Not equal");

// Mixed Unicode
println!("🎓 Learning 🦀 Rust: 10/10 ⭐");
```

## ⚡ Common Issues

**Works:** Most modern terminals support Unicode
```bash
# Terminal support check
echo "🦀"  # If you see crab, you're good!
```

**May not work:** Very old terminals, some IDE consoles
**Solution:** Use modern terminal or web-based REPL

## 🎯 Success Criteria

- [ ] Program prints with Unicode characters
- [ ] Can use emojis directly in code
- [ ] Output displays correctly in terminal
- [ ] Understands Unicode is default in Rust
- [ ] Can print at least 3 different Unicode types

---

[← Previous Exercise](../ex014_print_backslash/README.md) | [Next Exercise →](../ex016_comments_single_line/README.md)