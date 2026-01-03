# Exercise 75: Compare User Input ⭐

[← Previous Exercise](../ex074_default_with_unwrap_or/README.md) | [Next Exercise →](../ex078_check_if_empty/README.md)

## 🎯 Goal

Compare user input against expected values to make decisions.

## 📝 Task

Read user input and compare it to specific strings for validation and control flow.

## 🎓 Concept

**String comparison** checks if input matches expected values:

```rust
let input = "yes";
if input == "yes" {
    println!("Match!");
}
```

**Key considerations:**
- String comparison is case-sensitive by default
- Whitespace matters (always trim!)
- `==` compares entire strings, not substrings

**Common pattern:**
```rust
if input.trim().to_lowercase() == "yes" {
    // Case-insensitive comparison
}
```

## ✅ Solution Approach

### 🟢 Beginner Way - Direct comparison:
```rust
use std::io;

fn main() {
    println!("Do you agree? (yes/no)");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    if input.trim() == "yes" {
        println!("You agreed!");
    } else {
        println!("You declined.");
    }
}
```

*Trade-off:* ✅ Simple and clear  
⚠️ *Caveat:* Case-sensitive, only matches exact "yes"

### 🔵 Idiomatic Way - Flexible comparison:
```rust
use std::io;

fn main() {
    println!("Do you agree? (yes/no)");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    match input.trim().to_lowercase().as_str() {
        "yes" | "y" => println!("✅ You agreed!"),
        "no" | "n" => println!("❌ You declined."),
        _ => println!("Invalid input. Please enter yes or no."),
    }
}
```

*Why better:* Case-insensitive, accepts multiple formats, clear error handling  
*When to use:* User-facing applications, robust input validation

## 💡 Why This Matters

**Menu navigation:**
```rust
println!("Choose: (add/remove/quit)");
let mut choice = String::new();
io::stdin().read_line(&mut choice).expect("Failed");

match choice.trim() {
    "add" => println!("Adding..."),
    "remove" => println!("Removing..."),
    "quit" => std::process::exit(0),
    _ => println!("Unknown command"),
}
```

**Password verification:**
```rust
let correct_password = "secret123";
println!("Enter password:");

let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");

if input.trim() == correct_password {
    println!("Access granted!");
} else {
    println!("Access denied!");
}
```

**Validation:**
```rust
println!("Type 'CONFIRM' to proceed:");
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");

if input.trim() == "CONFIRM" {
    println!("Proceeding with operation...");
} else {
    println!("Operation cancelled.");
}
```

## 🔄 Try These

```rust
use std::io;

fn main() {
    // 1. Case-insensitive comparison
    println!("Enter 'start' to begin:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    if input.trim().to_lowercase() == "start" {
        println!("Starting...");  // Matches "START", "Start", "start"
    }
    
    // 2. Multiple valid inputs
    println!("\nContinue? (y/yes/ok)");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    
    let answer = input.trim().to_lowercase();
    if answer == "y" || answer == "yes" || answer == "ok" {
        println!("Continuing!");
    }
    
    // 3. Using match for clarity
    println!("\nSelect action: (save/load/exit)");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    
    match input.trim() {
        "save" => println!("Saving..."),
        "load" => println!("Loading..."),
        "exit" => println!("Goodbye!"),
        other => println!("Unknown: '{}'", other),
    }
    
    // 4. Substring checking
    println!("\nEnter command:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    
    if input.trim().starts_with("delete ") {
        println!("Delete command detected!");
    }
    
    // 5. Contains check
    if input.contains("help") {
        println!("Help requested!");
    }
}
```

## ⚡ Common Mistakes

```rust
// ❌ Not trimming input (newline included)
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");
if input == "yes" {  // Never matches! input is "yes\n"
    println!("Match!");
}

// ❌ Case sensitivity oversight
if input.trim() == "yes" {  // Doesn't match "Yes" or "YES"
    // ...
}

// ❌ Comparing wrong types
let input = "42";
if input == 42 {  // ERROR: can't compare &str with i32
    // ...
}

// ❌ Partial match when full match needed
let input = "yes please";
if input.contains("yes") {  // Matches! But might not be intended
    // User might have typed something else
}

// ✅ Correct patterns
// Always trim:
if input.trim() == "yes" {
    // ...
}

// Case-insensitive:
if input.trim().to_lowercase() == "yes" {
    // ...
}

// Compare same types:
let number: i32 = input.trim().parse().expect("Invalid");
if number == 42 {
    // ...
}

// Exact match, not substring:
if input.trim() == "yes" {  // Not contains!
    // ...
}
```

## 🎓 Advanced Comparison Techniques

**Starts with / Ends with:**
```rust
let input = "delete file.txt";

if input.starts_with("delete ") {
    let filename = &input[7..];  // Skip "delete "
    println!("Deleting: {}", filename);
}

if input.ends_with(".txt") {
    println!("Text file detected");
}
```

**Multiple conditions:**
```rust
let answer = input.trim().to_lowercase();

if matches!(answer.as_str(), "yes" | "y" | "yeah" | "yep") {
    println!("Affirmative!");
}
```

**Fuzzy matching (Levenshtein distance):**
```rust
// For typo tolerance - requires external crate like `strsim`
// Example concept:
fn similar_enough(input: &str, target: &str) -> bool {
    // Check if strings are similar (1-2 character difference)
    // Real implementation would use proper algorithm
    input.len() == target.len()
}
```

## 🔄 Real-World Use Cases

**Login system:**
```rust
fn login() -> bool {
    let correct_username = "admin";
    let correct_password = "secret";
    
    println!("Username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed");
    
    println!("Password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed");
    
    username.trim() == correct_username && password.trim() == correct_password
}
```

**Command parser:**
```rust
fn parse_command(input: &str) -> Option<String> {
    let input = input.trim().to_lowercase();
    
    match input.as_str() {
        "help" | "h" | "?" => Some("help".to_string()),
        "quit" | "q" | "exit" => Some("quit".to_string()),
        "save" | "s" => Some("save".to_string()),
        _ => None,
    }
}
```

**Yes/No confirmation:**
```rust
fn confirm(prompt: &str) -> bool {
    loop {
        println!("{} (yes/no)", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        
        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Please answer yes or no."),
        }
    }
}

fn main() {
    if confirm("Delete all files?") {
        println!("Deleting...");
    } else {
        println!("Cancelled.");
    }
}
```

## 🎯 Success Criteria

- [ ] Can compare strings with `==` operator
- [ ] Always trim whitespace before comparing
- [ ] Understand case sensitivity issues
- [ ] Can implement case-insensitive comparison
- [ ] Know when to use exact match vs contains/starts_with

---

[← Previous Exercise](../ex074_default_with_unwrap_or/README.md) | [Next Exercise →](../ex078_check_if_empty/README.md)
