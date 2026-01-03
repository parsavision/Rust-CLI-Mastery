# Exercise 74: Default with unwrap_or ⭐

[← Previous Exercise](../ex072_character_input/README.md) | [Next Exercise →](../ex075_compare_user_input/README.md)

## 🎯 Goal

Provide fallback values when parsing or extracting data that might not exist.

## 📝 Task

Use `.unwrap_or()` to handle `Option` and `Result` types with safe default values.

## 🎓 Concept

**`.unwrap_or()` provides a fallback value** when Option is None or Result is Err:

```rust
let option_value: Option<i32> = None;
let number = option_value.unwrap_or(0);  // Returns 0 if None
// number is 0
```

**Compared to `.unwrap()`:**
```rust
let none_value: Option<i32> = None;
let x = none_value.unwrap();      // ❌ PANIC!
let y = none_value.unwrap_or(42); // ✅ Returns 42, no panic
```

**Works with both Option and Result:**
```rust
// Option<T>
Some(5).unwrap_or(0)  // 5
None.unwrap_or(0)     // 0

// Result<T, E>
Ok(5).unwrap_or(0)    // 5
Err("error").unwrap_or(0)  // 0
```

## ✅ Solution Approach

### 🟢 Beginner Way - Basic fallback:
```rust
use std::io;

fn main() {
    println!("Enter a number (or press Enter for default):");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    let number = input.trim().parse::<i32>().unwrap_or(42);
    println!("Number: {}", number);
}
```

*Trade-off:* ✅ Never panics, always provides sensible default  
*When to use:* When any value is acceptable, not just valid input

### 🔵 Idiomatic Way - Conditional defaults:
```rust
use std::io;

fn main() {
    println!("Enter a number (or press Enter for default 100):");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    let trimmed = input.trim();
    
    let number = if trimmed.is_empty() {
        100  // Explicit default for empty input
    } else {
        trimmed.parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Invalid number, using default 100");
            100
        })
    };
    
    println!("Using number: {}", number);
}
```

*Why better:* Distinguishes between empty input and invalid input  
*When to use:* When you need different handling for different error cases

## 💡 Why This Matters

**Graceful degradation:**
```rust
// Config file might not have this setting
let max_retries = config.get("max_retries")
    .and_then(|s| s.parse().ok())
    .unwrap_or(3);  // Default to 3 retries

println!("Max retries: {}", max_retries);
```

**Safe character extraction:**
```rust
println!("Enter text:");
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");

let first_char = input.trim().chars().next().unwrap_or('?');
println!("First character: {}", first_char);  // '?' if empty
```

**Optional user preferences:**
```rust
println!("Enter your preferred theme (or press Enter for 'light'):");
let mut theme = String::new();
io::stdin().read_line(&mut theme).expect("Failed");

let selected_theme = if theme.trim().is_empty() {
    "light"
} else {
    theme.trim()
};

println!("Using theme: {}", selected_theme);
```

## 🔄 Try These

```rust
use std::io;

fn get_number_or_default(prompt: &str, default: i32) -> i32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    input.trim().parse().unwrap_or(default)
}

fn main() {
    // 1. Parse with fallback
    let age = get_number_or_default("Enter age (default 18):", 18);
    println!("Age: {}", age);
    
    // 2. Character with fallback
    println!("Enter grade (A-F):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let grade = input.trim().chars().next().unwrap_or('F');
    println!("Grade: {}", grade);
    
    // 3. Multiple fallbacks
    println!("Enter first number:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    let num1 = input.trim().parse::<f64>().unwrap_or(0.0);
    
    println!("Enter second number:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    let num2 = input.trim().parse::<f64>().unwrap_or(1.0);  // Default to 1 to avoid division by zero
    
    println!("{} / {} = {}", num1, num2, num1 / num2);
    
    // 4. Vec access with fallback
    let numbers = vec![10, 20, 30];
    let first = numbers.get(0).copied().unwrap_or(0);
    let tenth = numbers.get(10).copied().unwrap_or(0);  // Index out of bounds
    println!("First: {}, Tenth: {}", first, tenth);
}
```

## ⚡ Common Mistakes

```rust
// ❌ Using unwrap when you should use unwrap_or
let input = "";
let num = input.parse::<i32>().unwrap();  // PANIC on empty input!

// ❌ Wrong default type
let input = "not a number";
let num = input.parse::<i32>().unwrap_or(3.14);  // ERROR: expected i32, found float

// ❌ Expensive operation as default
let result = calculate().unwrap_or(expensive_fallback());  
// expensive_fallback() ALWAYS runs, even if Ok!

// ❌ Not considering what default means
let port = input.parse().unwrap_or(0);  // Port 0 is usually invalid!

// ✅ Correct patterns
// Right default type:
let num = input.parse::<i32>().unwrap_or(0);

// Lazy evaluation:
let result = calculate().unwrap_or_else(|| expensive_fallback());
// Only runs if Err

// Meaningful defaults:
let port = input.parse().unwrap_or(8080);  // Valid default port

// Check before defaulting:
if input.trim().is_empty() {
    println!("Using default");
}
let num = input.trim().parse().unwrap_or(42);
```

## 🎓 Advanced: unwrap_or_else

**For lazy/expensive defaults:**
```rust
use std::io;

fn expensive_default() -> i32 {
    println!("Computing expensive default...");
    // Imagine complex calculation here
    42
}

fn main() {
    println!("Enter number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    // unwrap_or: ALWAYS evaluates default (even if Ok)
    let num1 = input.trim().parse::<i32>()
        .unwrap_or(expensive_default());  // Runs every time!
    
    // unwrap_or_else: Only evaluates if Err
    let num2 = input.trim().parse::<i32>()
        .unwrap_or_else(|_| expensive_default());  // Only on error!
    
    println!("Number: {}", num2);
}
```

## 🔄 Real-World Use Cases

**Configuration with defaults:**
```rust
struct Config {
    port: u16,
    host: String,
    timeout: u64,
}

impl Config {
    fn from_env() -> Self {
        use std::env;
        
        Config {
            port: env::var("PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8080),
            host: env::var("HOST")
                .unwrap_or_else(|_| "localhost".to_string()),
            timeout: env::var("TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
        }
    }
}
```

**Safe array access:**
```rust
fn get_user_selection(options: &[String]) -> String {
    println!("Enter selection (1-{}):", options.len());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    let index = input.trim()
        .parse::<usize>()
        .ok()
        .and_then(|i| i.checked_sub(1))  // Convert 1-based to 0-based
        .unwrap_or(0);  // Default to first option
    
    options.get(index)
        .cloned()
        .unwrap_or_else(|| options[0].clone())
}
```

**User preferences:**
```rust
fn get_preference(prompt: &str, default: &str) -> String {
    println!("{} (default: {})", prompt, default);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    let trimmed = input.trim();
    if trimmed.is_empty() {
        default.to_string()
    } else {
        trimmed.to_string()
    }
}

fn main() {
    let username = get_preference("Username:", "guest");
    let language = get_preference("Language:", "en");
    println!("User: {}, Lang: {}", username, language);
}
```

## 🎯 Success Criteria

- [ ] Understand difference between `unwrap()` and `unwrap_or()`
- [ ] Can provide appropriate default values
- [ ] Know when to use `unwrap_or` vs `unwrap_or_else`
- [ ] Can use with both Option and Result types
- [ ] Avoid panics in production code

---

[← Previous Exercise](../ex072_character_input/README.md) | [Next Exercise →](../ex075_compare_user_input/README.md)
