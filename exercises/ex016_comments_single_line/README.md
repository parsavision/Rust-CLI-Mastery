# Exercise 16: Comments - Single Line ✅

[← Previous Exercise](../ex015_unicode_characters/README.md) | [Next Exercise →](../ex017_comments_multi_line/README.md)

## 🎯 Goal

Add single-line comments to explain your code.

## 📝 Task

Add `//` comments to explain what each part of your program does.

## 🎓 Concept

Single-line comments start with `//` and continue to end of line:

```rust
// This is a comment - compiler ignores it
let x = 5; // Comment after code also works
```

**Purpose:**
- Explain code logic
- Leave notes for future you
- Communicate with other developers
- Document intent and reasoning

## ✅ Solution Approach

```rust
// Calculate the area of a rectangle
fn main() {
    // Define dimensions
    let width = 10;
    let height = 5;
    
    // Calculate area (width × height)
    let area = width * height;
    
    // Display result
    println!("Area: {}", area);
}
```

## 💡 Why Comments Matter

**Without comments:**
```rust
let x = 5;
let y = 10;
let z = x + y * 2;
```

**With comments:**
```rust
// Base value for calculation
let x = 5;  
// Multiplier factor
let y = 10;
// Complex formula: base + (multiplier × 2)
let z = x + y * 2;
```

## 🔄 Try These

```rust
// Comment types
// TODO: Implement error handling
// FIXME: This is a temporary solution
// NOTE: Performance could be optimized
// WARNING: Don't use with negative numbers

// Explaining algorithms
fn fibonacci(n: u32) -> u32 {
    // Base case: first two numbers
    if n <= 1 { return n; }
    
    // Recursive case: sum of previous two
    fibonacci(n - 1) + fibonacci(n - 2)
}

// Inline vs separate
let result = calculate(); // Good for short explanations
let result = calculate();
// Better for longer explanations
// about what the function does
// and why we're calling it here
```

## ⚡ Comment Guidelines

**Good comments:**
```rust
// Validate user input before processing
if input.is_empty() { return; }

// Parse string to integer, handle potential errors
let number = input.parse::<i32>().unwrap_or(0);
```

**Poor comments:**
```rust
let x = 5; // Set x to 5 (obvious)
let name = "John"; // name is John (redundant)
// Add 2 to x (doesn't explain WHY)
let result = x + 2;
```

## 🎯 Success Criteria

- [ ] Program contains at least 3 `//` comments
- [ ] Comments explain what the code does
- [ ] Comments provide value beyond obvious
- [ ] Program still compiles and runs
- [ ] Can distinguish between good and poor comments

---

[← Previous Exercise](../ex015_unicode_characters/README.md) | [Next Exercise →](../ex017_comments_multi_line/README.md)