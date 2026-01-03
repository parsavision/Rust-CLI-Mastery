# Exercise 22: Integer Types ⭐✅

[← Previous Exercise](../ex021_variable_naming_rules/README.md) | [Next Exercise →](../ex023_different_integer_sizes/README.md)

## 🎯 Goal

Understand different integer types and why they matter.

## 📝 Task

Create variables with different integer types to see the differences.

## 🎓 Concept

Rust has many integer types with different sizes:

### Signed Integers (can be negative)
- `i8`  - -128 to 127
- `i16` - -32,768 to 32,767  
- `i32` - -2,147,483,648 to 2,147,483,647 *(default)*
- `i64` - -9×10¹⁸ to 9×10¹⁸
- `i128` - Very large range

### Unsigned Integers (only positive)
- `u8`  - 0 to 255
- `u16` - 0 to 65,535
- `u32` - 0 to 4,294,967,295
- `u64` - 0 to 18×10¹⁸
- `u128` - Very large positive range

### Why Different Sizes?

**Memory vs Range:**
- **Smaller types** = less memory, smaller range
- **Larger types** = more memory, larger range
- **Choose wisely** based on expected values

## ✅ Solution Approach

```rust
fn main() {
    // Different integer types with annotations
    let small_number: i8 = 42;
    let medium_number: i32 = 1000;
    let big_number: i64 = 1000000;
    let unsigned_small: u8 = 200;
    let unsigned_big: u64 = 4000000000;
    
    println!("i8: {}", small_number);
    println!("i32: {}", medium_number);  
    println!("i64: {}", big_number);
    println!("u8: {}", unsigned_small);
    println!("u64: {}", unsigned_big);
}
```

**Output:**
```
i8: 42
i32: 1000
i64: 1000000
u8: 200
u64: 4000000000
```

## 💡 Why Type Annotations Matter

**Without annotation:**
```rust
let x = 42;        // Rust infers i32 (default)
let y = 3000000000; // Rust infers i64
```

**With annotation:**
```rust
let x: i8 = 42;           // We specify the type
let y: u32 = 4000000000;   // We choose unsigned 32-bit
```

## 🔄 Try These

```rust
// Test range limits
let max_i8: i8 = 127;      // Maximum i8 value
let min_i8: i8 = -128;     // Minimum i8 value

// Try exceeding range (will overflow!)
// let too_big_i8: i8 = 128;  // Compile error!

// Choose type based on use case
let age: u8 = 25;           // Age won't be negative, < 255
let population: u32 = 1000000; // Large positive number
let temperature: i32 = -10;    // Can be negative

// See type sizes
println!("i8 size: {} bytes", std::mem::size_of::<i8>());
println!("i32 size: {} bytes", std::mem::size_of::<i32>());
println!("i64 size: {} bytes", std::mem::size_of::<i64>());
```

## ⚡ Common Mistakes

```rust
// ❌ Type mismatch
let x: i8 = 300;    // 300 doesn't fit in i8!

// ❌ Wrong type for data
let age: u8 = -5;      // Can't store negative in unsigned!

// ❌ Ignoring overflow
let small: i8 = 100;   
let result = small + 50;  // 150 > 127, overflow!

// ✅ Choose appropriate type
let age: u8 = 25;      // Positive, small range
let temperature: i32 = -10; // Can be negative, larger range
let big_count: u64 = 1000000000; // Very large positive
```

## 📊 Type Selection Guide

| Use Case | Recommended Type | Why? |
|----------|-------------------|--------|
| Age (0-120) | `u8` | Small, positive |
| Temperature | `i32` | Can be negative |
| Population count | `u64` | Large positive numbers |
| Small counts (0-255) | `u8` | Efficient memory use |
| General purpose | `i32` | Good balance, default |

## 🎯 Success Criteria

- [ ] Create variables with different integer types
- [ ] Use type annotations (`: i8`, `: u32`, etc.)
- [ ] Program compiles without type errors
- [ ] Can explain why different types exist
- [ ] Can choose appropriate type for a given scenario

---

[← Previous Exercise](../ex021_variable_naming_rules/README.md) | [Next Exercise →](../ex023_different_integer_sizes/README.md)