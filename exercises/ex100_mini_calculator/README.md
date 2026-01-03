# Exercise 100: Mini Calculator

## 🎯 Goal
Complete beginner project

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, Simple)
```rust
// Simple calculator with basic operations
use std::io;

fn main() {
    println!("=== Mini Calculator ===");
    
    // Get first number
    println!("Enter first number:");
    let mut num1_str = String::new();
    io::stdin().read_line(&mut num1_str).expect("Failed to read");
    let num1: f64 = num1_str.trim().parse().expect("Not a number");
    
    // Get operator
    println!("Enter operator (+, -, *, /):");
    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Failed to read");
    let op = op.trim();
    
    // Get second number
    println!("Enter second number:");
    let mut num2_str = String::new();
    io::stdin().read_line(&mut num2_str).expect("Failed to read");
    let num2: f64 = num2_str.trim().parse().expect("Not a number");
    
    // Calculate
    let result = match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => {
            println!("Unknown operator!");
            return;
        }
    };
    
    println!("\n{} {} {} = {}", num1, op, num2, result);
}
```

**Why this works:**
- Covers all basic operations
- Simple control flow
- Clear output

**Trade-offs:**
- ⚠️ Multiple expect() calls can panic
- ⚠️ No division by zero check
- ⚠️ Basic error handling

### Idiomatic Approach (Safe, Professional)
```rust
// Production-ready calculator with full error handling
use std::io::{self, Write};

fn get_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn get_number(prompt: &str) -> Result<f64, String> {
    let input = get_input(prompt)
        .map_err(|e| format!("Input error: {}", e))?;
    input.parse()
        .map_err(|_| format!("'{}' is not a valid number", input))
}

fn calculate(a: f64, op: &str, b: f64) -> Result<f64, String> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err("Cannot divide by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err(format!("Unknown operator: '{}'. Use +, -, *, or /", op))
    }
}

fn main() -> Result<(), String> {
    println!("=== Mini Calculator (Phase 2 Complete!) ===\n");
    
    let num1 = get_number("Enter first number: ")?;
    let operator = get_input("Enter operator (+, -, *, /): ")?;
    let num2 = get_number("Enter second number: ")?;
    
    let result = calculate(num1, &operator, num2)?;
    
    println!("\n✅ {} {} {} = {:.2}", num1, operator, num2, result);
    println!("\n🎉 Congratulations! You've completed Phase 2!");
    println!("You now know: variables, input, output, operators, and formatting!");
    
    Ok(())
}
```

**Why this is better:**
- ✅ Zero panic risk
- ✅ Division by zero handled explicitly  
- ✅ Helpful error messages for all cases
- ✅ Input validation

**Key Insight:**
Calculators - always validate operations (like division by zero) and handle all error cases.

---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern
```rust
// Calculator without validation
use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let a: f64 = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let op = input.trim();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let b: f64 = input.trim().parse().unwrap();
    
    let result = match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b, // DANGER: Division by zero!
        _ => panic!("Bad operator"), // DANGER: Explicit panic!
    };
    
    println!("{}", result);
}
```

**Why it's dangerous:**
1. Division by zero causes panic
2. Invalid operators cause panic
3. Invalid numbers cause panic
4. No error messages for users
5. Poor user experience

### ✅ Safe Alternative
```rust
// Production-ready calculator
use std::io::{self, Write};

fn get_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn get_number(prompt: &str) -> Result<f64, String> {
    let input = get_input(prompt)
        .map_err(|e| format!("Input error: {}", e))?;
    input.parse()
        .map_err(|_| format!("'{}' is not a valid number", input))
}

fn calculate(a: f64, op: &str, b: f64) -> Result<f64, String> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err("Cannot divide by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err(format!("Unknown operator: '{}'. Use +, -, *, or /", op))
    }
}

fn main() -> Result<(), String> {
    println!("=== Mini Calculator (Phase 2 Complete!) ===\n");
    
    let num1 = get_number("Enter first number: ")?;
    let operator = get_input("Enter operator (+, -, *, /): ")?;
    let num2 = get_number("Enter second number: ")?;
    
    let result = calculate(num1, &operator, num2)?;
    
    println!("\n✅ {} {} {} = {:.2}", num1, operator, num2, result);
    println!("\n🎉 Congratulations! You've completed Phase 2!");
    
    Ok(())
}
```

**Why this is better:**
- Zero panic risk
- Division by zero handled explicitly
- Helpful error messages for all cases
- Input validation
- Proper error propagation with ?
- Clean separation of concerns
- Professional user experience

---

## 🔍 Modern Rust: Clippy Insights

Run `cargo clippy` to see helpful suggestions.

**Relevant lints for this exercise:**
- `expect_used`
- `unwrap_used`

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

Get two numbers and operator (+, -, *, /), show result

**Requirements:**
- Implement the core functionality
- Handle errors gracefully
- Provide helpful user feedback
- Follow Rust best practices

**Example Usage:**
```bash
$ cargo run
=== Mini Calculator ===
Enter first number: 10
Enter operator (+, -, *, /): +
Enter second number: 5

10 + 5 = 15
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

1. **Calculator logic** - Core concept for this exercise
2. **Safety first** - Always handle errors explicitly
3. **User-friendly** - Provide clear feedback
4. **Idiomatic patterns** - Follow Rust conventions

---

## 📖 Further Reading

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

---

**[← Previous Exercise](../ex099_input_output_review/README.md) | [Next Exercise →](../ex101_first_function/README.md)**
