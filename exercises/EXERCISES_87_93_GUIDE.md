# Exercises 87-93: Quick Reference Guide

This document provides key teaching points for exercises 87-93. Full READMEs following the template can be generated from these summaries.

## Exercise 87: Get Float Input

**Key Concept:** Parsing floating-point numbers with f64

**Beginner Trap:**
```rust
let num = input.parse::<f64>().unwrap(); // Panics on "abc"
```

**Idiomatic:**
```rust
match input.trim().parse::<f64>() {
    Ok(num) => println!("Float: {}", num),
    Err(_) => println!("Not a valid decimal"),
}
```

**Safety Focus:**
- f64 can parse "inf", "NaN", "-inf" - might need validation
- Very large/small numbers might lose precision
- Use `.is_finite()` to check for inf/NaN

**Clippy Lints:**
- `unwrap_used`
- `cast_precision_loss`

---

## Exercise 88: Multiple Float Inputs

**Key Concept:** Reusing float input pattern multiple times

**Beginner Trap:**
```rust
let a = input1.parse::<f64>().expect("Invalid");
let b = input2.parse::<f64>().expect("Invalid");
let c = input3.parse::<f64>().expect("Invalid");
// Repetitive, panic-prone
```

**Idiomatic:**
```rust
fn get_float(prompt: &str) -> Result<f64, String> {
    print!("{}: ", prompt);
    // ... implementation
}

let a = get_float("Enter first number")?;
let b = get_float("Enter second number")?;
let c = get_float("Enter third number")?;
```

**Safety Focus:**
- DRY principle - Don't Repeat Yourself
- Extract common patterns to functions
- Use `?` operator to propagate errors

---

## Exercise 89: Input with Default

**Key Concept:** Fallback values for empty input

**Beginner Trap:**
```rust
if input.is_empty() {
    value = 0; // Hardcoded default
} else {
    value = input.parse().unwrap();
}
```

**Idiomatic:**
```rust
let value = if input.trim().is_empty() {
    default_value
} else {
    input.trim().parse().unwrap_or(default_value)
};
```

**Safety Focus:**
- Empty input is valid user choice (pressing Enter)
- Always trim before checking `is_empty()`
- Consider `.unwrap_or()` for clean fallbacks

**Clippy Lints:**
- `if_then_some_else_none` - suggests using `.then_some()`
- `manual_unwrap_or` - simpler patterns available

---

## Exercise 90: Complete Input Library

**Key Concept:** Building reusable input utilities

**Functions to Implement:**
```rust
fn get_string(prompt: &str) -> String
fn get_i32(prompt: &str) -> Result<i32, String>
fn get_f64(prompt: &str) -> Result<f64, String>
fn get_bool(prompt: &str) -> bool  // y/n confirmation
fn get_char(prompt: &str) -> Result<char, String>
```

**Safety Focus:**
- Generic error handling across all types
- Consistent API design
- Consider adding retry loops
- Make functions composable

**This is a milestone exercise** - students create utilities they'll use for remaining 500+ exercises!

---

## Exercise 91: Format Numbers

**Key Concept:** Pretty-printing numbers with separators

**Beginner Trap:**
```rust
println!("{}", 1000000); // Hard to read: 1000000
```

**Idiomatic:**
```rust
// Manual formatting (Rust std doesn't have built-in)
fn format_with_commas(n: i32) -> String {
    let s = n.to_string();
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    
    for (i, c) in chars.iter().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, *c);
    }
    result
}

println!("{}", format_with_commas(1000000)); // 1,000,000
```

**Safety Focus:**
- String manipulation requires care
- Consider using external crates (num-format) for production
- Handle negative numbers correctly

**Clippy Lints:**
- `manual_string_new` - prefer `String::new()`
- `needless_range_loop` - use iterators

---

## Exercise 92: Decimal Places

**Key Concept:** Controlling float precision in output

**Beginner Trap:**
```rust
let pi = 3.14159;
println!("{}", pi); // 3.14159 - too many digits
```

**Idiomatic:**
```rust
let pi = 3.14159;
println!("{:.2}", pi); // 3.14 - exactly 2 decimal places
```

**Format Specifiers:**
- `{:.2}` - 2 decimal places
- `{:.0}` - no decimal places (rounds)
- `{:8.2}` - width 8, 2 decimals (right-aligned)
- `{:<8.2}` - width 8, 2 decimals (left-aligned)

**Safety Focus:**
- Formatting doesn't change the value, only display
- Rounding behavior: uses "round half to even"
- Precision limits for f64: ~15-17 decimal digits

---

## Exercise 93: Padding Numbers

**Key Concept:** Alignment and zero-padding

**Beginner Trap:**
```rust
for i in 1..100 {
    println!("{}", i); // Misaligned:
}
// 1
// 10
// 100
```

**Idiomatic:**
```rust
for i in 1..100 {
    println!("{:03}", i); // Zero-padded to width 3:
}
// 001
// 010
// 100
```

**Format Specifiers:**
- `{:05}` - width 5, zero-padded
- `{:5}` - width 5, space-padded
- `{:<5}` - width 5, left-aligned
- `{:>5}` - width 5, right-aligned (default)
- `{:^5}` - width 5, center-aligned

**Safety Focus:**
- Padding doesn't change the numeric value
- Useful for tables, lists, file names
- Common in CLI tools for aligned output

---

## Common Patterns Across All Exercises

### Error Handling Progression:
1. **Ex 84-85**: Basic `match` on `Result`
2. **Ex 86**: Boolean returns with validation
3. **Ex 87-88**: `Result<T, String>` with custom errors
4. **Ex 89**: Fallback values with `.unwrap_or()`
5. **Ex 90**: Complete reusable library

### Code Quality Principles:
- **DRY**: Extract repeated code to functions
- **Single Responsibility**: Each function does one thing
- **Explicit Error Handling**: No hidden `.unwrap()` calls
- **User-Friendly**: Clear prompts and error messages

### Testing Strategy:
- Valid inputs (happy path)
- Invalid inputs (error cases)
- Edge cases (empty, very large, special values)
- Boundary conditions

---

## Next Steps

After completing exercises 84-93, students have mastered:
- ✅ Input parsing and validation
- ✅ Error handling with Result and Option
- ✅ String manipulation and formatting
- ✅ Building reusable utilities
- ✅ Professional CLI patterns

**Ready for Phase 3 (Functions & Reusability)**!
