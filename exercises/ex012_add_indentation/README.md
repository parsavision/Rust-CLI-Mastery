# Exercise 12: Add Indentation ✅

[← Previous Exercise](../ex011_use_escape_sequence/README.md) | [Next Exercise →](../ex013_print_quote_marks/README.md)

## 🎯 Goal

Use tab escape sequence to add indentation to your output.

## 📝 Task

Print a formatted list with tab indentation to demonstrate the `\t` escape sequence.

## 🎓 Concept

The tab character (`\t`) creates horizontal spacing, useful for:
- Aligning columns
- Indenting text
- Creating structured output
- Making output more readable

## ✅ Solution Approach

```rust
fn main() {
    println!("Fruits:");
    println!("\tApple");
    println!("\tBanana");
    println!("\tOrange");
    println!();
    println!("Languages:");
    println!("\tRust");
    println!("\tPython");
    println!("\tJavaScript");
}
```

**Output:**
```
Fruits:
        Apple
        Banana
        Orange

Languages:
        Rust
        Python
        JavaScript
```

## 💡 Why Tabs Matter

**Manual spacing:**
```rust
println!("    Apple");      // Hard to count spaces
println!("      Rust");      // Inconsistent
```

**With tabs:**
```rust
println!("\tApple");       // Clear intention
println!("\tRust");        // Consistent
```

## 🔄 Try These

```rust
// Nested indentation
println!("Level 1");
println!("\tLevel 2");
println!("\t\tLevel 3");

// Column alignment
println!("Name\t\tAge");
println!("Alice\t\t25");
println!("Bob\t\t30");

// Mixed with newlines
println!("Items:");
println!("\t1. Item one");
println!("\t2. Item two");
```

## ⚡ Common Mistakes

```rust
// ❌ Using letter 't' instead of \t
println!("tItem");

// ❌ Forward slash instead of backslash
println!("/tItem");

// ✅ Correct tab escape
println!("\tItem");
```

## 🎯 Success Criteria

- [ ] Program uses `\t` escape sequence
- [ ] Output shows clear indentation
- [ ] Multiple lines formatted consistently
- [ ] Compiles without errors
- [ ] Can explain difference between `\t` and spaces

---

[← Previous Exercise](../ex011_use_escape_sequence/README.md) | [Next Exercise →](../ex013_print_quote_marks/README.md)