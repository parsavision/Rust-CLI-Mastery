# Exercise 52: Create Empty String ⭐

[← Previous Exercise](../ex051_import_statement/README.md) | [Next Exercise →](../ex053_read_input/README.md)

## 🎯 Goal

Learn how to create an empty String in Rust for storing user input.

## 📝 Task

Create a mutable empty String that can be filled with user input later.

## 🎓 Concept

**String::new() creates an empty String:**

```rust
let mut input = String::new();
```

**Why `mut`?**
- Strings need to grow when we add data
- User input will be added to this String
- Without `mut`, the String is immutable (can't change)

**String vs &str:**
```rust
let string_slice: &str = "Hello";  // Fixed text, can't grow
let owned_string: String = String::new();  // Can grow, owns data
```

## ✅ Solution Approach

### 🟢 Beginner Way - Direct creation:
```rust
use std::io;

fn main() {
    let mut input = String::new();
    println!("Empty string created with capacity: {}", input.capacity());
}
```

*Trade-off:* ✅ Simple and clear  
*Why it works:* String starts with 0 capacity, grows as needed

### 🔵 Idiomatic Way - Pre-allocate if size known:
```rust
use std::io;

fn main() {
    // If you expect ~100 characters of input
    let mut input = String::with_capacity(100);
    println!("String created with capacity: {}", input.capacity());
    
    // Or standard way when size unknown
    let mut user_input = String::new();
}
```

*Why better:* Avoids multiple reallocations for known sizes  
*When to use:* Reading large files or predictable input lengths

## 💡 Why This Matters

**Memory efficiency:**
```rust
// ❌ Creating new String each time
for _ in 0..1000 {
    let temp = String::new();  // 1000 allocations!
}

// ✅ Reusing String with .clear()
let mut buffer = String::new();
for _ in 0..1000 {
    buffer.clear();  // Reuse same allocation
    // Read input into buffer
}
```

**Foundation for input:**
```rust
// This pattern is everywhere in Rust I/O
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read");
println!("You typed: {}", input);
```

## 🔄 Try These

```rust
use std::io;

fn main() {
    // 1. Basic empty String
    let mut text = String::new();
    println!("Length: {}, Capacity: {}", text.len(), text.capacity());
    
    // 2. Pre-allocated String
    let mut buffer = String::with_capacity(50);
    println!("Length: {}, Capacity: {}", buffer.len(), buffer.capacity());
    
    // 3. Add text to see it grow
    text.push_str("Hello");
    println!("After adding 'Hello': len={}, cap={}", text.len(), text.capacity());
    
    // 4. Clear and reuse
    text.clear();
    println!("After clear: len={}, cap={}", text.len(), text.capacity());
    // Capacity stays, length becomes 0!
}
```

## ⚡ Common Mistakes

```rust
// ❌ Forgetting `mut` when you'll add data
let input = String::new();
input.push_str("Hi");  // ERROR: cannot mutate immutable variable

// ❌ Using &str when you need owned String
let input = "";  // This is &str, can't grow
io::stdin().read_line(&mut input);  // ERROR: expected String, found &str

// ❌ Creating String from string literal unnecessarily
let greeting = String::from("Hello");  // Allocates heap memory
let better = "Hello";  // Just stack reference, cheaper

// ✅ Correct empty String creation
let mut input = String::new();  // Can grow
let mut preallocated = String::with_capacity(100);  // Efficient for known size
```

## 🎯 Success Criteria

- [ ] Can create empty String with `String::new()`
- [ ] Understand why `mut` is needed for input
- [ ] Know difference between String and &str
- [ ] Can explain when to use `with_capacity()`
- [ ] Program compiles without errors

---

[← Previous Exercise](../ex051_import_statement/README.md) | [Next Exercise →](../ex053_read_input/README.md)
