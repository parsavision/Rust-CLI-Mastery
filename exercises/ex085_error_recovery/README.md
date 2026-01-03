# Exercise 85: Error Recovery

## 🎯 Goal
Learn to handle parse errors gracefully using `match` instead of panicking with `.expect()`.

## 📚 The "Why" and Trade-offs

### Beginner Approach (Works, But Crashes)
```rust
let input = "abc";
let number = input.parse::<i32>().expect("Not a number!");
// Panics! Thread 'main' panicked at 'Not a number!'
```

**Why this works (for valid input):**
- Simple, one line
- Provides error message
- Fine for prototypes

**Trade-offs:**
- ⚠️ **Crashes the entire program on invalid input**
- ⚠️ **No recovery possible** - user must restart
- ⚠️ **Shows panic backtrace** - looks unprofessional
- ⚠️ **Bad user experience** - hostile to mistakes

### Idiomatic Approach (Safe, User-Friendly)
```rust
match input.parse::<i32>() {
    Ok(num) => println!("You entered: {}", num),
    Err(_) => println!("That's not a valid number. Try again."),
}
```

**Why this is better:**
- ✅ **Never crashes** - handles errors gracefully
- ✅ **User-friendly** - shows helpful message instead of panic
- ✅ **Allows retry** - program continues running
- ✅ **Professional** - no cryptic backtraces
- ✅ **Explicit** - forces you to think about error cases

**Key Insight:**
`.parse()` returns `Result<T, E>` because parsing can fail. Use `match` to handle both success (`Ok`) and failure (`Err`) cases explicitly. This is safer and more user-friendly than `.expect()` or `.unwrap()`.

---

## 🛡️ Safety First: Common Pitfalls

### ⚠️ Risky Pattern #1: Using `.expect()` for User Input
```rust
let number = input.parse::<i32>().expect("Not a number!");
// ⚠️ User types "hello" → PANIC!
```

**Why it's dangerous:**
1. **User input is untrusted** - users make typos, enter wrong formats
2. **Expect is for impossible errors** - not for expected failures
3. **Kills the program** - no chance to retry
4. **Bad UX** - users see scary error messages

**When `.expect()` IS appropriate:**
```rust
// System operations that "shouldn't" fail
let file = File::open("config.txt").expect("Config must exist");
// Developer error if config is missing - panic is acceptable
```

### ⚠️ Risky Pattern #2: Using `.unwrap()`
```rust
let number = input.parse::<i32>().unwrap();
// Even worse! No error message, just panic
```

**Why it's dangerous:**
- Same as `.expect()` but NO custom message
- Generic panic: "called `Option::unwrap()` on a `None` value"
- Even harder to debug

### ✅ Safe Alternative #1: Basic `match`
```rust
match input.trim().parse::<i32>() {
    Ok(num) => {
        println!("Success! You entered: {}", num);
        // Continue with valid number
    }
    Err(e) => {
        println!("Error: '{}' is not a valid number", input.trim());
        println!("Please try again.");
        // Program continues, can ask for input again
    }
}
```

**Why this is better:**
- Handles both cases explicitly
- User sees helpful message
- Program doesn't crash
- Can implement retry logic

### ✅ Safe Alternative #2: Using `if let` for Simple Cases
```rust
if let Ok(num) = input.trim().parse::<i32>() {
    println!("You entered: {}", num);
} else {
    println!("That's not a valid number!");
}
```

**Why this works:**
- Simpler than match when you don't need the error details
- Still handles errors gracefully
- No panic risk

### ✅ Safe Alternative #3: Providing Default with `.unwrap_or()`
```rust
let number = input.trim().parse::<i32>().unwrap_or(0);
println!("Number (or 0 if invalid): {}", number);
```

**Why this works:**
- Never panics - provides fallback value
- Good for non-critical input
- Silently handles errors (use with caution - users might not know input was invalid)

### Common Mistakes to Avoid
- ❌ Using `.expect()` or `.unwrap()` for any user input
- ❌ Ignoring error details (sometimes you want to show why parsing failed)
- ❌ Not trimming input before parsing (newline from `read_line` breaks parsing)
- ❌ Silent failures with `.unwrap_or()` when user should know about errors

---

## 🔍 Modern Rust: Clippy Insights

Run `cargo clippy` on code using `.expect()`:

```bash
$ cargo clippy

warning: used `expect()` which can panic
  --> src/main.rs:15:42
   |
15 |     let number = input.parse::<i32>().expect("Not a number!");
   |                                       ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(clippy::expect_used)]` on by default
   = help: Consider using `match` or `if let` for explicit error handling

warning: you should consider adding a `Default` implementation
   |
   = help: or use `unwrap_or_default()` instead of `expect()`
```

**What Clippy teaches:**
- **`expect_used`**: `.expect()` can panic. For user input, use `match` to handle errors gracefully
- **`unwrap_used`**: Similar warning for `.unwrap()` - even more dangerous as it has no error message
- **Why it matters**: Professional Rust code handles errors explicitly rather than panicking

**How to fix:**
```rust
// Clippy-approved
match input.trim().parse::<i32>() {
    Ok(num) => println!("Valid: {}", num),
    Err(_) => println!("Invalid input"),
}
```

### Clippy Lints for This Exercise
- `expect_used`: Warns about `.expect()` calls - suggests match
- `unwrap_used`: Warns about `.unwrap()` calls - more dangerous
- `match_wild_err_arm`: Warns if you ignore error details with `Err(_)` when you should log them

---

## 💪 Progressive Challenges

### Challenge 1: Replace `.expect()` with `match` ⏱️ 5-10 minutes
**Goal:** Handle parse errors without panicking

**Tasks:**
- [ ] Replace `.expect("Not a number")` with proper `match` statement
- [ ] Print helpful error message on invalid input
- [ ] Test with valid input: `"42"`
- [ ] Test with invalid input: `"hello"`
- [ ] Test with empty input: `""`
- [ ] Ensure program never panics

### Challenge 2: Add Retry Loop ⏱️ 10-15 minutes
**Goal:** Keep asking until user enters valid number

**Tasks:**
- [ ] Wrap input/parse in a `loop`
- [ ] `break` out of loop only on valid input
- [ ] Show error and continue loop on invalid input
- [ ] Test by intentionally entering bad input first

**Example:**
```rust
loop {
    println!("Enter a number:");
    // ... get input ...
    
    match input.trim().parse::<i32>() {
        Ok(num) => {
            println!("Got it: {}", num);
            break;
        }
        Err(_) => {
            println!("Invalid! Try again...");
            input.clear(); // Clear for next iteration
        }
    }
}
```

### Challenge 3: Show Specific Error Details ⏱️ 15-25 minutes
**Goal:** Tell users exactly what went wrong

**Tasks:**
- [ ] Instead of `Err(_)`, capture the error: `Err(e)`
- [ ] Print what the error was: `println!("Error: {}", e)`
- [ ] Create function `get_number_from_user() -> i32` that loops until valid
- [ ] Add support for different number types (i32, f64) with generic function

**Example:**
```rust
fn get_number_from_user() -> i32 {
    loop {
        print!("Enter number: ");
        io::stdout().flush().ok();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        
        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(e) => println!("Error: {}. Try again.", e),
        }
    }
}
```

### Challenge 4: Full Error Handling ⏱️ 20-30 minutes
**Goal:** Professional error handling and testing

**Tasks:**
- [ ] Create `parse_number(input: &str) -> Result<i32, String>` function
- [ ] Return custom error messages (not just ParseIntError)
- [ ] Add unit tests for valid and invalid inputs
- [ ] Handle edge cases: very large numbers, negative numbers
- [ ] Add range validation (e.g., only accept 1-100)

**Test example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_number() {
        assert_eq!(parse_number("42"), Ok(42));
    }

    #[test]
    fn test_invalid_number() {
        assert!(parse_number("abc").is_err());
    }

    #[test]
    fn test_negative() {
        assert_eq!(parse_number("-5"), Ok(-5));
    }

    #[test]
    fn test_leading_zeros() {
        assert_eq!(parse_number("007"), Ok(7));
    }
}
```

---

## 📝 Task

Handle parse errors gracefully instead of panicking.

**Requirements:**
- Get user input
- Parse as i32
- Use `match` to handle Ok/Err cases
- Don't use `.expect()` or `.unwrap()`
- Show helpful error message on invalid input

**Example Usage:**
```bash
$ cargo run
Enter a number:
> 42
You entered: 42

$ cargo run
Enter a number:
> hello
That's not a valid number!
```

---

## 🧪 Testing Your Solution

```bash
# Test with valid input
echo "42" | cargo run

# Test with invalid input
echo "abc" | cargo run

# Check for panics (should be none)
cargo clippy

# Valid inputs to try:
# ✅ "42" → should work
# ✅ "-5" → should work (negative)
# ✅ "0" → should work

# Invalid inputs to try (should NOT crash):
# ⚠️ "hello" → should show error message
# ⚠️ "" → should show error message
# ⚠️ "12.5" → i32 can't parse decimals
# ⚠️ "999999999999999" → too large for i32
```

---

## 💡 Key Takeaways

1. **`.parse()` returns `Result`** - always handle both Ok and Err cases
2. **Use `match` for user input** - never `.expect()` or `.unwrap()` on untrusted data
3. **Errors are not failures** - they're expected scenarios to handle gracefully
4. **Trim before parsing** - `read_line()` includes newline character
5. **Loop for retry** - good UX lets users correct mistakes without restarting

---

## 📖 Further Reading

- [Rust Book: Result and Error Handling](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [std::result::Result](https://doc.rust-lang.org/std/result/enum.Result.html)
- [str::parse()](https://doc.rust-lang.org/std/primitive.str.html#method.parse)
- [Clippy: expect_used](https://rust-lang.github.io/rust-clippy/master/index.html#expect_used)

---

## 🎓 Concepts Covered

- `Result<T, E>` enum
- Pattern matching on Result
- `Ok(value)` and `Err(error)` variants
- Graceful error handling
- User input validation
- Retry loops

---

**[← Previous Exercise](../ex084_split_input/README.md) | [Next Exercise →](../ex086_confirm_action/README.md)**
