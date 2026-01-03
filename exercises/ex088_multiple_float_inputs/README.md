# Exercise 88: Multiple Float Inputs

## 🎯 Goal
Handle several decimals

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, Simple)
```rust
use std::io;

fn get_float() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().parse::<f64>().expect("Not a number")
}

// Get three values
let num1 = get_float();
let num2 = get_float();
let num3 = get_float();
println!("Numbers: {}, {}, {}", num1, num2, num3);
```

**Why this works:**
- Reuses a simple function
- Direct approach
- Minimal code

**Trade-offs:**
- ⚠️ **Panics on any invalid input**
- ⚠️ **No prompt** - user doesn't know what to enter
- ⚠️ **Can't recover from errors**
- ⚠️ **Not user-friendly**

### Idiomatic Approach (Safe, Professional)
```rust
use std::io::{self, Write};

fn get_float(prompt: &str) -> Result<f64, String> {
    print!("{}", prompt);
    io::stdout().flush().ok();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;
    
    input.trim().parse::<f64>()
        .map_err(|_| format!("'{}' is not a valid number", input.trim()))
}

// Usage with proper error handling
fn main() -> Result<(), String> {
    let nums = vec![
        get_float("Enter first number: ")?,
        get_float("Enter second number: ")?,
        get_float("Enter third number: ")?,
    ];
    
    println!("You entered: {}, {}, {}", nums[0], nums[1], nums[2]);
    println!("Average: {:.2}", nums.iter().sum::<f64>() / nums.len() as f64);
    Ok(())
}
```

**Why this is better:**
- ✅ **Returns Result** - caller handles errors
- ✅ **Clear prompts** - user knows what to enter
- ✅ **Specific error messages** - helpful feedback
- ✅ **Composable** - can use `?` operator
- ✅ **Reusable pattern** - works for any number of inputs

**Key Insight:**
When you need multiple inputs, a well-designed helper function with `Result` return makes your code clean and safe. The `?` operator handles errors elegantly.

---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern
```rust
// No prompts, user confused
let mut input = String::new();
io::stdin().read_line(&mut input).ok();
let n1 = input.trim().parse::<f64>().expect("Bad input");

input.clear();
io::stdin().read_line(&mut input).ok();
let n2 = input.trim().parse::<f64>().expect("Bad input");
// ⚠️ Any invalid input crashes with cryptic error
```

**Why it's dangerous:**
1. **User doesn't know what to enter** - no prompts
2. **Cryptic error on invalid input** - unhelpful messages  
3. **Crashes immediately** - no retry opportunity
4. **Tedious to repeat** - lots of duplicated code

### ✅ Safe Alternative
```rust
fn get_floats(count: usize) -> Result<Vec<f64>, String> {
    let mut numbers = Vec::new();
    
    for i in 1..=count {
        print!("Enter number {}: ", i);
        io::stdout().flush().ok();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .map_err(|e| format!("Read error: {}", e))?;
        
        let num = input.trim().parse::<f64>()
            .map_err(|_| format!("'{}' is not valid", input.trim()))?;
        
        numbers.push(num);
    }
    
    Ok(numbers)
}

// Usage:
match get_floats(3) {
    Ok(nums) => println!("Sum: {:.2}", nums.iter().sum::<f64>()),
    Err(e) => println!("Error: {}", e),
}
```

**Why this is better:**
- Clear numbered prompts
- All errors handled gracefully
- Clean, reusable code
- User-friendly experience

---

## 🔍 Modern Rust: Clippy Insights

Run `cargo clippy` to see helpful suggestions.

**Relevant lints for this exercise:**
- `unwrap_used`
- `expect_used`

**What Clippy teaches:**
Use safe patterns and explicit error handling throughout your code.

---

## 💪 Progressive Challenges

### Challenge 1: Eliminate Panics ⏱️ 5-10 minutes
**Goal:** Make your code panic-free

**Tasks:**
- [ ] Replace .unwrap() with .unwrap_or() or match
- [ ] Handle all error cases explicitly
- [ ] Test with edge cases

### Challenge 2: Better Error Handling ⏱️ 10-15 minutes
**Goal:** Provide helpful error messages

**Tasks:**
- [ ] Return Result<T, String> from functions
- [ ] Provide specific error messages
- [ ] Handle edge cases gracefully

### Challenge 3: Idiomatic Rust ⏱️ 15-25 minutes
**Goal:** Write professional code

**Tasks:**
- [ ] Use ? operator for error propagation
- [ ] Follow clippy suggestions
- [ ] Add documentation comments
- [ ] Pass cargo clippy with zero warnings

### Challenge 4: Testing & Polish ⏱️ 20-30 minutes (Optional)
**Goal:** Production-ready code

**Tasks:**
- [ ] Add unit tests for valid inputs
- [ ] Test error cases
- [ ] Test edge cases
- [ ] Achieve >80% code coverage

---

## 📝 Task

Get three f64 values

**Requirements:**
- Implement the core functionality
- Handle errors gracefully
- Provide helpful user feedback
- Follow Rust best practices

**Example Usage:**
```bash
$ cargo run
Exercise 88: Multiple Float Inputs
# Expected output based on task
```

---

## 🧪 Testing Your Solution

```bash
# Build and run
cargo run

# Run tests
cargo test

# Check for issues
cargo clippy

# Format code
cargo fmt
```

---

## 💡 Key Takeaways

1. **Reusing float input pattern** - Core concept for this exercise
2. **Safety first** - Always handle errors explicitly
3. **User-friendly** - Provide clear feedback
4. **Idiomatic patterns** - Follow Rust conventions

---

## 📖 Further Reading

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

---

 **[← Previous Exercise](../ex087_get_float_input/README.md) | [Next Exercise →](../ex089_input_with_default/README.md)**
