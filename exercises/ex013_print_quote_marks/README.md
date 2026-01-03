# Exercise 13: Print Quote Marks ✅

[← Previous Exercise](../ex012_add_indentation/README.md) | [Next Exercise →](../ex014_print_backslash/README.md)

## 🎯 Goal

Print text that includes quotation marks using escape sequences.

## 📝 Task

Print the sentence: `He said, "Hello, World!"` with proper quote marks.

## 🎓 Concept

To include quote marks inside a string, use the escape sequence `\"`:

- `"` - Starts or ends a string
- `\"` - Literal quote character inside string

**Without escape:** `println!("He said, "Hello!"")` ❌ (Syntax error)
**With escape:** `println!("He said, \"Hello!\"")` ✅

## ✅ Solution Approach

```rust
fn main() {
    println!("He said, \"Hello, World!\"");
}
```

**Output:**
```
He said, "Hello, World!"
```

## 💡 Why Escape Quotes Matter

**Problem:** Rust thinks the quote ends the string early
```rust
println!("He said, "Hello, World!");   // ❌ Compiler error
                ^ string ends here
```

**Solution:** Escape the quote so it's treated as text
```rust
println!("He said, \"Hello, World!\"); // ✅ Quote is literal
```

## 🔄 Try These

```rust
// Multiple quotes
println!("\"Hello\" and \"Goodbye\"");

// Nested quotes (quote inside quote)
println!("She replied, \"He said \\\"Hi!\\\" to me.\"");

// Single quotes don't need escaping
println!("Don't forget to say 'Hello'!");

// Mixed quotes
println!("Title: \"The 'Rust' Book\"");
```

## ⚡ Common Mistakes

```rust
// ❌ Missing backslash
println!("He said, "Hello, World!");

// ❌ Forward slash
println!("He said, /"Hello, World!/" );

// ❌ Escaping single quotes (not needed)
println!("It\'s a beautiful day!");

// ✅ Correct usage
println!("He said, \"Hello, World!\"");
```

## 🎯 Success Criteria

- [ ] Program prints quote marks correctly
- [ ] Uses `\"` escape sequence
- [ ] Output matches expected exactly
- [ ] Compiles without syntax errors
- [ ] Can use quotes in different contexts

---

[← Previous Exercise](../ex012_add_indentation/README.md) | [Next Exercise →](../ex014_print_backslash/README.md)