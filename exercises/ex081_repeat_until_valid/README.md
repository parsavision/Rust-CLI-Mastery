# Exercise 81: Repeat Until Valid ⭐

[← Previous Exercise](../ex080_password_input_simulation/README.md) | [Next Exercise →](../ex082_multiple_choice_input/README.md)

## 🎯 Goal

Loop until the user provides valid input, implementing robust input validation.

## 📝 Task

Ask the user to enter a positive number. Keep prompting until they provide valid input, then display the result.

## 🎓 Concept

**Validation loops** are essential for robust CLI applications. They ensure your program only proceeds with valid data.

**Basic validation loop structure:**
```rust
loop {
    // Get input
    // Validate
    if valid {
        break;  // Exit loop
    }
    // Show error, try again
}
```

**Why validation loops matter:**
- **User mistakes:** People mistype, enter wrong format
- **Robust programs:** Never crash on bad input
- **User experience:** Clear feedback helps users succeed
- **Data integrity:** Only process valid data

## ✅ Solution Approach

### 🟢 Beginner Way - Simple loop with manual checks:
```rust
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    loop {
        print!("Enter a positive number: ");
        io::stdout().flush().expect("Failed to flush stdout");
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        // Try to parse as u32 (unsigned = positive only)
        if input.trim().parse::<u32>().is_ok() {
            println!("You entered: {}", input.trim());
            break;  // Valid input, exit loop
        }
        
        println!("Invalid input. Please enter a positive number.");
        input.clear();  // Important: clear for next iteration
    }
}
```

*Trade-off:* Works reliably, uses `is_ok()` to check Result, clears buffer properly.

### 🔵 Idiomatic Way - Match for better control:
```rust
use std::io::{self, Write};

fn main() {
    loop {
        print!("Enter a positive number: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<u32>() {
            Ok(number) => {
                println!("You entered: {}", number);
                break;  // Exit on success
            }
            Err(_) => {
                println!("Invalid input. Please enter a positive number.");
                // Loop continues automatically
            }
        }
    }
}
```

*Why better:* New `String` each iteration (auto-clears), `match` shows intent clearly, handles Ok/Err explicitly.

*When to use:* Production code, when you need to handle errors differently, clearer intent.

## 💡 Why This Matters

Validation loops are **everywhere** in CLI applications:

1. **Login systems** - Retry password until correct or max attempts
2. **Menu selections** - Keep showing menu until valid choice
3. **File operations** - Retry until valid filename provided
4. **Configuration** - Ensure settings are valid before saving
5. **Data entry** - Forms that require specific formats

**Without validation loops:**
```rust
// ❌ Program crashes on bad input
let number: u32 = input.trim().parse().expect("Must be number!");
```

**With validation loops:**
```rust
// ✅ Program guides user to success
loop {
    match input.trim().parse::<u32>() {
        Ok(n) => break,
        Err(_) => println!("Try again with a number"),
    }
}
```

## 🔄 Try These

```rust
// Experiment 1: Validate within range
loop {
    print!("Enter age (1-120): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim().parse::<u32>() {
        Ok(age) if age >= 1 && age <= 120 => {
            println!("Valid age: {}", age);
            break;
        }
        Ok(age) => println!("Age {} out of range!", age),
        Err(_) => println!("Please enter a number"),
    }
}

// Experiment 2: Count attempts
let mut attempts = 0;
loop {
    attempts += 1;
    print!("Attempt {}: Enter positive number: ", attempts);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if let Ok(n) = input.trim().parse::<u32>() {
        println!("Success after {} attempts!", attempts);
        break;
    }
    println!("Try again...");
}

// Experiment 3: Maximum attempts
const MAX_ATTEMPTS: u32 = 3;
for attempt in 1..=MAX_ATTEMPTS {
    print!("Attempt {}/{}: ", attempt, MAX_ATTEMPTS);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if let Ok(n) = input.trim().parse::<u32>() {
        println!("Valid: {}", n);
        break;
    }
    
    if attempt == MAX_ATTEMPTS {
        println!("Too many attempts. Exiting.");
    } else {
        println!("Invalid. {} attempts remaining.", MAX_ATTEMPTS - attempt);
    }
}

// Experiment 4: Different validation
loop {
    print!("Enter email: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let email = input.trim();
    if email.contains('@') && email.contains('.') {
        println!("Email accepted: {}", email);
        break;
    }
    println!("Invalid email format");
}
```

## ⚡ Common Mistakes

```rust
// ❌ Forgetting to clear input buffer (when reusing String)
let mut input = String::new();
loop {
    io::stdin().read_line(&mut input).unwrap();
    // ... validation ...
    // Missing: input.clear()
}
// Bug: Input accumulates each iteration!

// ❌ Infinite loop with no exit
loop {
    println!("Enter number:");
    // Never breaks! User trapped forever
}

// ❌ Not flushing stdout before input
loop {
    print!("Enter: ");  // May not appear before input prompt!
    io::stdin().read_line(&mut input).unwrap();
}

// ✅ Correct pattern
loop {
    print!("Enter: ");
    io::stdout().flush().unwrap();  // Ensure prompt shows
    
    let mut input = String::new();  // Fresh string each time
    io::stdin().read_line(&mut input).unwrap();
    
    if let Ok(value) = input.trim().parse::<u32>() {
        break;  // Exit condition!
    }
}
```

## 🎓 Advanced Patterns

### Validation with custom function:
```rust
fn get_positive_number() -> u32 {
    loop {
        print!("Enter positive number: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<u32>() {
            Ok(n) => return n,  // Return directly on success
            Err(_) => println!("Invalid, try again"),
        }
    }
}

let number = get_positive_number();  // Reusable!
```

### While loop alternative:
```rust
let mut valid = false;
while !valid {
    // Get input
    match input.trim().parse::<u32>() {
        Ok(_) => valid = true,
        Err(_) => println!("Try again"),
    }
}
```

## 🔄 Real-World Use Cases

1. **Login systems** - Retry password until correct or lockout
2. **Configuration wizards** - Each step validates before proceeding
3. **File selectors** - Retry until valid file path provided
4. **Game input** - Ensure valid moves before processing
5. **Form validation** - Don't save until all fields valid

## 🎯 Success Criteria

- [ ] Program loops until user provides valid input
- [ ] Uses `loop` with `break` to exit on success
- [ ] Clears input buffer between attempts (or creates new String)
- [ ] Flushes stdout before reading input
- [ ] Provides clear error messages
- [ ] Can explain when to use validation loops
- [ ] Understands `.is_ok()` vs `match` patterns

---

[← Previous Exercise](../ex080_password_input_simulation/README.md) | [Next Exercise →](../ex082_multiple_choice_input/README.md)
