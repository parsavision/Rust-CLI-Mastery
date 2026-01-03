# Exercise 84: Split Input

## 🎯 Goal
Learn to parse multiple values from a single line of input by splitting a string and parsing each part separately.

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, Simple)
```rust
let input = "5 3";
let parts: Vec<&str> = input.trim().split(' ').collect();
let num1 = parts[0].parse::<i32>().expect("Not a number");
let num2 = parts[1].parse::<i32>().expect("Not a number");
println!("Numbers: {} and {}", num1, num2);
```

**Why this works:**
- Direct indexing `parts[0]` and `parts[1]` gets the values
- `.expect()` handles parse errors with a message
- Easy to understand the flow

**Trade-offs:**
- ⚠️ **Panics if user enters fewer than 2 values** (e.g., just "5")
- ⚠️ **Panics if split produces less than expected** (empty input)
- ⚠️ **Panics if values aren't valid numbers**
- ⚠️ **Assumes exactly one space** (multiple spaces break it)
- No recovery possible when errors occur

### Idiomatic Approach (Safe, Professional)
```rust
fn parse_two_numbers(input: &str) -> Option<(i32, i32)> {
    let mut parts = input.trim().split_whitespace();
    let num1 = parts.next()?.parse().ok()?;
    let num2 = parts.next()?.parse().ok()?;
    Some((num1, num2))
}

// Usage:
match parse_two_numbers(&input) {
    Some((a, b)) => println!("Numbers: {} and {}", a, b),
    None => println!("Error: Please enter two valid numbers"),
}
```

**Why this is better:**
- ✅ **No panic risk** - returns `Option` for missing/invalid data
- ✅ **Handles multiple spaces** - `.split_whitespace()` is smarter
- ✅ **Composable** - can use `?` operator to chain operations
- ✅ **Self-documenting** - function name explains intent
- ✅ **Graceful error handling** - caller decides what to do on failure
- ✅ **Returns tuple** - natural pairing of related values

**Key Insight:**
String splitting is inherently fallible (wrong format, missing values). Use iterators and `Option` to handle this gracefully instead of assuming input is always correct.

---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern #1: Direct Indexing After Split
```rust
let parts: Vec<&str> = input.split(' ').collect();
let num1 = parts[0];  // ⚠️ Panics if empty input!
let num2 = parts[1];  // ⚠️ Panics if only one value!
```

**Why it's dangerous:**
1. **Index out of bounds** - User enters "" → `parts` is empty → panic
2. **Insufficient values** - User enters "5" → only `parts[0]` exists → `parts[1]` panics
3. **No error message** - Just crashes with cryptic "index out of bounds"
4. **Hidden assumption** - Code assumes user always enters correct format

### ⚠️ Risky Pattern #2: Chained `.expect()` Calls
```rust
let num1 = input.split(' ').nth(0).expect("Missing first").parse::<i32>().expect("Not a number");
```

**Why it's dangerous:**
- Multiple panic points in one line
- Hard to tell which `.expect()` failed
- No way to recover from any error
- Creates fragile code that crashes easily

### ⚠️ Risky Pattern #3: Assuming Single Space
```rust
let parts: Vec<&str> = input.split(' ').collect();
// User enters "5  3" (two spaces) → creates empty string element!
```

**Why it's dangerous:**
- Multiple consecutive spaces create empty strings
- Leading/trailing spaces create empty elements
- Parsing empty string fails
- Creates subtle bugs hard to debug

### ✅ Safe Alternative #1: Iterator with Pattern Matching
```rust
let mut parts = input.trim().split_whitespace();
match (parts.next(), parts.next()) {
    (Some(a), Some(b)) => {
        match (a.parse::<i32>(), b.parse::<i32>()) {
            (Ok(num1), Ok(num2)) => println!("Numbers: {} and {}", num1, num2),
            _ => println!("Error: Both values must be numbers"),
        }
    }
    _ => println!("Error: Please enter two numbers"),
}
```

**Why this is better:**
- No indexing - no panic risk
- Explicitly checks for presence of values
- Handles all error cases gracefully
- User gets helpful error messages

### ✅ Safe Alternative #2: Using `?` Operator (Most Idiomatic)
```rust
fn parse_two_numbers(input: &str) -> Result<(i32, i32), String> {
    let mut parts = input.trim().split_whitespace();
    
    let num1 = parts.next()
        .ok_or("Missing first number")?
        .parse::<i32>()
        .map_err(|_| "First value is not a number")?;
    
    let num2 = parts.next()
        .ok_or("Missing second number")?
        .parse::<i32>()
        .map_err(|_| "Second value is not a number")?;
    
    Ok((num1, num2))
}
```

**Why this is better:**
- Returns specific error messages
- No panic points
- Early return on any error
- Composable with other Result-returning functions
- Professional Rust pattern

### Common Mistakes to Avoid
- ❌ Using `.split(' ')` instead of `.split_whitespace()` (doesn't handle multiple spaces)
- ❌ Collecting to `Vec` before checking length (unnecessary allocation)
- ❌ Indexing with `[0]`, `[1]` instead of using iterator methods
- ❌ Using `.expect()` in production code (only for prototyping)

---

## 🔍 Modern Rust: Clippy Insights

Run `cargo clippy` on beginner code with indexing:

```bash
$ cargo clippy

warning: indexing may panic
  --> src/main.rs:15:18
   |
15 |     let num1 = parts[0].parse::<i32>().expect("Not a number");
   |                ^^^^^^^^
   |
   = note: `#[warn(clippy::indexing_slicing)]` on by default
   = help: Consider using `.get(..)` or `.first()` to avoid panicking

warning: called `.expect()` on a `Result` value
  --> src/main.rs:15:33
   |
   = help: Consider using `match` or `if let` for more explicit error handling
   = note: `#[warn(clippy::expect_used)]` on by default
```

**What Clippy teaches:**
- **`indexing_slicing`**: Array/vector indexing can panic. Use `.get(index)` which returns `Option<&T>` for safe access, or `.first()` for the first element
- **`expect_used`**: While `.expect()` provides error messages, it still panics. Consider `match` or `if let` for graceful handling in user-facing code
- **Why it matters**: CLI tools should never crash on unexpected input. Users expect helpful error messages, not panic backtraces

**How to fix:**
```rust
// Clippy-approved approach
let num1 = parts.get(0)
    .and_then(|s| s.parse::<i32>().ok())
    .unwrap_or(0);
```

### Clippy Lints for This Exercise
- `indexing_slicing`: Catches `parts[0]` - suggests `.get(0)`
- `expect_used`: Catches `.expect()` - suggests explicit match
- `unnecessary_to_owned`: Catches unnecessary `.to_string()` or `.collect()`
- `manual_split_once`: If splitting on first occurrence, use `.split_once()` instead

---

## 💪 Progressive Challenges

After completing the basic version, level up your code:

### Challenge 1: Eliminate Panics ⏱️ 5-10 minutes
**Goal:** Make your code panic-free

**Tasks:**
- [ ] Replace `.collect()` + indexing with iterator methods (`.next()`)
- [ ] Replace `.expect()` with `.unwrap_or()` or `match`
- [ ] Use `.split_whitespace()` instead of `.split(' ')` (handles multiple spaces)
- [ ] Test with these inputs:
  - [ ] Empty string: `""`
  - [ ] One number: `"5"`
  - [ ] Three numbers: `"5 3 7"` (what should happen?)
  - [ ] Non-numbers: `"abc def"`
  - [ ] Multiple spaces: `"5    3"`

**Hint:** Use `let mut parts = input.trim().split_whitespace()` then call `.next()` twice.

**Example starter:**
```rust
let mut parts = input.trim().split_whitespace();
let first = parts.next();  // Option<&str>
let second = parts.next(); // Option<&str>

match (first, second) {
    (Some(a), Some(b)) => {
        // Now parse a and b
    }
    _ => {
        println!("Error: Please enter exactly two values");
    }
}
```

### Challenge 2: Better Error Handling ⏱️ 10-15 minutes
**Goal:** Provide helpful, user-friendly errors

**Tasks:**
- [ ] Create a function `parse_two_numbers(input: &str) -> Result<(i32, i32), String>`
- [ ] Return specific error messages:
  - [ ] `"Please enter two numbers"` if wrong count
  - [ ] `"First value must be a number"` if first parse fails
  - [ ] `"Second value must be a number"` if second parse fails
- [ ] Handle all edge cases gracefully
- [ ] Use the function in main with proper error handling

**Example signature:**
```rust
fn parse_two_numbers(input: &str) -> Result<(i32, i32), String> {
    let mut parts = input.trim().split_whitespace();
    
    let first = parts.next().ok_or("Please enter two numbers")?;
    let second = parts.next().ok_or("Please enter two numbers")?;
    
    let num1 = first.parse::<i32>()
        .map_err(|_| "First value must be a number".to_string())?;
    
    let num2 = second.parse::<i32>()
        .map_err(|_| "Second value must be a number".to_string())?;
    
    Ok((num1, num2))
}
```

### Challenge 3: Idiomatic Rust ⏱️ 15-25 minutes
**Goal:** Write code that looks professional

**Tasks:**
- [ ] Use the `?` operator consistently for error propagation
- [ ] Add a retry loop - keep asking until valid input
- [ ] Return `Result<(), Box<dyn std::error::Error>>` from main
- [ ] Add documentation comments (`///`) for your function
- [ ] Ensure `cargo clippy` passes with zero warnings
- [ ] Use `.split_once()` if you need exactly two parts

**Example refactoring:**
```rust
use std::io::{self, Write};

/// Parses two space-separated integers from a string.
///
/// # Errors
/// Returns an error if the input doesn't contain exactly two valid integers.
///
/// # Examples
/// ```
/// let result = parse_two_numbers("5 3");
/// assert_eq!(result, Ok((5, 3)));
/// ```
fn parse_two_numbers(input: &str) -> Result<(i32, i32), String> {
    // Implementation
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Enter two numbers separated by space:");
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match parse_two_numbers(&input) {
            Ok((a, b)) => {
                println!("You entered: {} and {}", a, b);
                println!("Sum: {}", a + b);
                break;
            }
            Err(e) => {
                println!("Error: {}", e);
                println!("Try again...");
            }
        }
    }
    
    Ok(())
}
```

### Challenge 4: Testing & Polish ⏱️ 20-30 minutes (Optional)
**Goal:** Production-ready code

**Tasks:**
- [ ] Add unit tests covering:
  - [ ] Valid input: `"5 3"` → `Ok((5, 3))`
  - [ ] Multiple spaces: `"5    3"` → `Ok((5, 3))`
  - [ ] Leading/trailing spaces: `"  5 3  "` → `Ok((5, 3))`
  - [ ] Empty input: `""` → `Err(...)`
  - [ ] One number: `"5"` → `Err(...)`
  - [ ] Three numbers: `"5 3 7"` → should this be `Ok((5, 3))` or error?
  - [ ] Invalid numbers: `"abc def"` → `Err(...)`
- [ ] Add module-level documentation
- [ ] Run `cargo test` - all tests pass
- [ ] Run `cargo fmt` - code is formatted
- [ ] Run `cargo clippy -- -D warnings` - no warnings

**Test example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        assert_eq!(parse_two_numbers("5 3"), Ok((5, 3)));
    }

    #[test]
    fn test_multiple_spaces() {
        assert_eq!(parse_two_numbers("5    3"), Ok((5, 3)));
    }

    #[test]
    fn test_leading_trailing_spaces() {
        assert_eq!(parse_two_numbers("  5 3  "), Ok((5, 3)));
    }

    #[test]
    fn test_empty_input() {
        assert!(parse_two_numbers("").is_err());
    }

    #[test]
    fn test_one_number() {
        assert!(parse_two_numbers("5").is_err());
    }

    #[test]
    fn test_invalid_first_number() {
        assert!(parse_two_numbers("abc 3").is_err());
    }

    #[test]
    fn test_invalid_second_number() {
        assert!(parse_two_numbers("5 def").is_err());
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(parse_two_numbers("-5 -3"), Ok((-5, -3)));
    }
}
```

---

## 📝 Task

Parse multiple values from a single line of input.

**Requirements:**
- Get user input (e.g., `"5 3"`)
- Split the input into separate parts
- Parse each part as an integer
- Print both numbers

**Example Usage:**
```bash
$ cargo run
Exercise 84: Split Input
Enter two numbers separated by space (e.g., '5 3'):
> 5 3
You entered: 5 and 3

$ cargo run
Enter two numbers separated by space:
> hello world
Error: Both values must be numbers
```

---

## 🧪 Testing Your Solution

```bash
# Build and run
cargo run

# Run with test input (Linux/Mac)
echo "5 3" | cargo run

# Run tests (if you added them)
cargo test

# Check for issues
cargo clippy

# Format code
cargo fmt

# Try these test cases:
# ✅ Valid input: "5 3" → should work
# ✅ Multiple spaces: "5    3" → should work with split_whitespace
# ⚠️ One number: "5" → should handle gracefully (not panic)
# ⚠️ Empty input: "" → should handle gracefully
# ⚠️ Non-numbers: "abc def" → should show error message
# ⚠️ Three numbers: "5 3 7" → decide if error or take first two
```

---

## 💡 Key Takeaways

1. **Iterators > Indexing** - Use `.next()` on iterators instead of collecting and indexing to avoid panic risks
2. **`.split_whitespace()` > `.split(' ')`** - Handles multiple spaces, tabs, and other whitespace correctly
3. **`Option` and `Result` are your friends** - They force you to handle missing/invalid data explicitly
4. **The `?` operator** - Makes error handling concise while remaining safe and explicit
5. **Test edge cases** - Empty input, wrong counts, invalid formats - think like an adversarial user

---

## 📖 Further Reading

- [Rust Book: Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Rust Book: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [String::split_whitespace()](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace)
- [Iterator::next()](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next)
- [Clippy: indexing_slicing](https://rust-lang.github.io/rust-clippy/master/index.html#indexing_slicing)
- [Rust by Example: Error Handling](https://doc.rust-lang.org/rust-by-example/error.html)

---

## 🎓 Concepts Covered

- String splitting (`.split()` vs `.split_whitespace()`)
- Iterator methods (`.next()`, `.collect()`)
- Chaining `Option` with `?` operator
- Error propagation with `Result`
- Tuple return types
- Pattern matching with tuples
- Safe alternatives to indexing

---

**[← Previous Exercise](../ex083_input_with_units/README.md) | [Next Exercise →](../ex085_error_recovery/README.md)**
