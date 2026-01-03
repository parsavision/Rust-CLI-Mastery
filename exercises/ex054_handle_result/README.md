# Exercise 54: Handle Result ⭐

[← Previous Exercise](../ex053_read_input/README.md) | [Next Exercise →](../ex055_print_user_input/README.md)

## 🎯 Goal

Learn how to handle Result types from I/O operations safely.

## 📝 Task

Handle the Result returned by `read_line()` using `.expect()`.

## 🎓 Concept

**Result<T, E> type:**

```rust
// read_line returns Result<usize, io::Error>
let result = io::stdin().read_line(&mut input);

// Result is either:
Ok(bytes_read)   // Success: number of bytes read
Err(error)       // Failure: I/O error occurred
```

**Why Result?**
- I/O operations can fail (disk full, permission denied, etc.)
- Rust forces you to handle potential errors
- Makes failures explicit and visible

## ✅ Solution Approach

### 🟢 Beginner Way - Using `.expect()`:
```rust
use std::io;

fn main() {
    let mut input = String::new();
    
    // expect() panics with custom message on error
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    println!("Input: {}", input);
}
```

*Trade-off:* ⚠️ Program crashes on error (acceptable for learning)  
*Why it works:* `.expect()` unwraps Ok value or panics with message

### 🔵 Idiomatic Way - Using `match`:
```rust
use std::io;

fn main() {
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(bytes) => {
            println!("Read {} bytes: {}", bytes, input.trim());
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
            std::process::exit(1);
        }
    }
}
```

*Why better:* Explicit error handling, can recover or exit gracefully  
*When to use:* Production code, reusable functions

### 🟣 Alternative - Using `?` operator:
```rust
use std::io;

fn get_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;  // Returns error if fails
    Ok(input.trim().to_string())
}

fn main() {
    match get_input() {
        Ok(text) => println!("Got: {}", text),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

*Why better:* Clean error propagation, composable functions  
*When to use:* Functions that can return errors

## 💡 Why This Matters

**Rust won't let you ignore errors:**
```rust
// ❌ Compiler warning - unused Result
io::stdin().read_line(&mut input);

// ✅ Must handle explicitly
io::stdin().read_line(&mut input).expect("Failed");
```

**Different error handling strategies:**
```rust
// 1. Crash with custom message (learning, prototyping)
.expect("Failed to read input");

// 2. Crash with error details (debugging)
.unwrap();

// 3. Provide default on error (graceful degradation)
.unwrap_or(0);

// 4. Match and handle (production)
match result {
    Ok(val) => use_value(val),
    Err(e) => handle_error(e),
}

// 5. Propagate error up (library functions)
result?;
```

## 🔄 Try These

```rust
use std::io;

fn main() {
    let mut input = String::new();
    
    // 1. expect() with custom message
    println!("Enter your name:");
    io::stdin().read_line(&mut input).expect("Failed to read name");
    println!("Name: {}", input.trim());
    
    // 2. unwrap() - crashes without custom message
    input.clear();
    println!("Enter your age:");
    let bytes = io::stdin().read_line(&mut input).unwrap();
    println!("Read {} bytes", bytes);
    
    // 3. match for explicit handling
    input.clear();
    println!("Enter your city:");
    match io::stdin().read_line(&mut input) {
        Ok(n) => println!("Read {} bytes: {}", n, input.trim()),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // 4. if let for Ok case only
    input.clear();
    println!("Enter your country:");
    if let Ok(bytes) = io::stdin().read_line(&mut input) {
        println!("Success: read {} bytes", bytes);
    }
}
```

## ⚡ Common Mistakes

```rust
use std::io;

// ❌ Ignoring Result (compiler warns)
io::stdin().read_line(&mut input);
// warning: unused `Result` that must be used

// ❌ Using expect without descriptive message
io::stdin().read_line(&mut input).expect("Error");
// Better: .expect("Failed to read user input")

// ❌ Using unwrap in production code
io::stdin().read_line(&mut input).unwrap();
// Better: Use match or ? operator

// ❌ Not differentiating error types
match io::stdin().read_line(&mut input) {
    Ok(_) => {},
    Err(_) => println!("Something failed"),  // Too vague!
}

// ✅ Correct Result handling
io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line from stdin");
```

## 🛡️ Safety First

**When to use each approach:**

```rust
// .expect() - Good for:
// - Learning/practice exercises
// - Prototype/quick scripts
// - Cases where error = unrecoverable
io::stdin().read_line(&mut input).expect("Failed to read");

// match - Good for:
// - Production code
// - Recoverable errors
// - Different handling per error type
match io::stdin().read_line(&mut input) {
    Ok(_) => continue_program(),
    Err(e) => {
        log_error(e);
        retry_or_exit();
    }
}

// ? operator - Good for:
// - Functions returning Result
// - Error propagation
// - Composing fallible operations
fn read_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
```

**Error message best practices:**
```rust
// ❌ Bad: Vague
.expect("Error")

// ❌ Bad: Restates the obvious
.expect("read_line failed")

// ✅ Good: Specific and actionable
.expect("Failed to read user input from stdin")

// ✅ Good: Context-aware
.expect("Failed to read filename (needed for save operation)")
```

## 🎯 Success Criteria

- [ ] Can handle Result with `.expect()`
- [ ] Understand when program will panic
- [ ] Know difference between expect() and unwrap()
- [ ] Can write helpful error messages
- [ ] Aware of match and ? alternatives

---

[← Previous Exercise](../ex053_read_input/README.md) | [Next Exercise →](../ex055_print_user_input/README.md)
