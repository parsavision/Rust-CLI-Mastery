# Exercise 17: Comments - Multi Line ✅

[← Previous Exercise](../ex016_comments_single_line/README.md) | [Next Exercise →](../ex018_first_variable/README.md)

## 🎯 Goal

Use multi-line comments for longer explanations and documentation.

## 📝 Task

Add block comments using `/* */` to explain your program.

## 🎓 Concept

Multi-line (block) comments use `/*` to start and `*/` to end:

```rust
/* This is a multi-line comment
   that can span many lines
   and include detailed explanations
*/
let x = 5;
```

**Use cases:**
- Long explanations
- Temporary code disabling
- Complex algorithm documentation
- License headers
- Module documentation

## ✅ Solution Approach

```rust
/*
 * Exercise 17: Multi-line Comments
 * 
 * This program demonstrates how to write
 * block comments that span multiple lines.
 * 
 * Block comments are useful for:
 * - Detailed function explanations
 * - Disabling multiple lines temporarily
 * - License information
 * - Complex algorithm documentation
 */
fn main() {
    /*
     * Calculate and print a simple result
     * Using block comments to explain each step
     */
    let base = 10;     /* Starting value */
    let multiplier = 2;  /* Factor to multiply by */
    let result = base * multiplier;  /* Final calculation */
    
    println!("Result: {}", result);  /* Display output */
}
```

## 💡 When to Use Block Comments

**Multi-line (`/* */`):**
```rust
/*
 * Calculate the greatest common divisor (GCD)
 * using Euclidean algorithm
 * 
 * The algorithm works by repeatedly:
 * 1. Dividing larger number by smaller
 * 2. Replacing larger with remainder
 * 3. Repeating until remainder is 0
 * 4. GCD is the last non-zero remainder
 */
fn gcd(a: u32, b: u32) -> u32 {
    // Implementation here...
}
```

**Single-line (`//`):**
```rust
// Quick inline explanation
let result = calculate(); // Inline is fine here
```

## ⚡ Common Patterns

**License headers:**
```rust
/*
 * Copyright (c) 2024 Your Name
 * 
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation files
 */
```

**Documentation blocks:**
```rust
/*
 * ========================================
 * FUNCTION: calculate_area
 * ========================================
 * 
 * PARAMETERS:
 *   width  (f64) - rectangle width
 *   height (f64) - rectangle height
 * 
 * RETURNS:
 *   (f64) - calculated area
 * 
 * DESCRIPTION:
 *   Calculates rectangle area using formula:
 *   area = width × height
 * ========================================
 */
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}
```

## ⚠️ Nested Comment Warning

**Nested block comments don't work:**
```rust
/*
 * Outer comment starts here
   /*
    * This inner comment will cause an error
    */
   Outer comment continues
 */
// ❌ ERROR: This won't compile!
```

**Instead, use single-line comments inside:**
```rust
/*
 * Outer comment starts here
   // Single-line comment inside works fine
   Outer comment continues
 */
// ✅ CORRECT: This compiles!
```

## 🎯 Success Criteria

- [ ] Program contains multi-line comments (`/* */`)
- [ ] Comments span at least 2 lines
- [ ] Program compiles and runs
- [ ] Can explain when to use block vs single-line
- [ ] Avoids nested comment mistakes

---

[← Previous Exercise](../ex016_comments_single_line/README.md) | [Next Exercise →](../ex018_first_variable/README.md)