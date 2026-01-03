# Exercise 71: Boolean from String ⭐

[← Previous Exercise](../ex063_use_user_number/README.md) | [Next Exercise →](../ex072_character_input/README.md)

## 🎯 Goal

Parse boolean values (true/false) from user input strings.

## 📝 Task

Read user input and convert it to a boolean value for yes/no decisions.

## 🎓 Concept

**Boolean parsing** converts strings to true/false:

```rust
let input = "true";
let bool_value: bool = input.parse().expect("Not a boolean");
// bool_value is true
```

**Valid boolean strings:**
- `"true"` → `true`
- `"false"` → `false`
- Anything else → Parse error!

**Case sensitivity:**
```rust
"true".parse::<bool>()   // ✅ Ok(true)
"True".parse::<bool>()   // ❌ Err - case matters!
"TRUE".parse::<bool>()   // ❌ Err - must be lowercase
```

## ✅ Solution Approach

### 🟢 Beginner Way - Direct parsing:
```rust
use std::io;

fn main() {
    println!("Enter true or false:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    let answer: bool = input.trim().parse().expect("Not a boolean");
    
    if answer {
        println!("You said true!");
    } else {
        println!("You said false!");
    }
}
```

*Trade-off:* ✅ Simple and direct  
⚠️ *Caveat:* Crashes if user types anything other than "true" or "false"

### 🔵 Idiomatic Way - Flexible yes/no parsing:
```rust
use std::io;

fn main() {
    println!("Do you agree? (yes/no):");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    let answer = input.trim().to_lowercase();
    
    match answer.as_str() {
        "yes" | "y" | "true" | "1" => {
            println!("✅ You agreed!");
        }
        "no" | "n" | "false" | "0" => {
            println!("❌ You declined.");
        }
        _ => {
            println!("Invalid input. Please enter yes or no.");
        }
    }
}
```

*Why better:* User-friendly, accepts multiple formats  
*When to use:* Real applications where users don't know exact format

## 💡 Why This Matters

**Confirmation prompts:**
```rust
println!("Delete all files? (yes/no)");
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");

if input.trim().to_lowercase() == "yes" {
    println!("Deleting...");
} else {
    println!("Cancelled.");
}
```

**Feature flags:**
```rust
println!("Enable debug mode? (true/false)");
let debug: bool = input.trim().parse().unwrap_or(false);

if debug {
    println!("Debug mode ON");
}
```

**Configuration:**
```rust
// Config file: "auto_save=true"
let auto_save: bool = config_value.parse().unwrap_or(false);
if auto_save {
    save_automatically();
}
```

## 🔄 Try These

```rust
use std::io;

fn get_confirmation(prompt: &str) -> bool {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    match input.trim().to_lowercase().as_str() {
        "yes" | "y" | "true" | "1" => true,
        _ => false,
    }
}

fn main() {
    // 1. Strict boolean parsing
    println!("Enter 'true' or 'false':");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    match input.trim().parse::<bool>() {
        Ok(b) => println!("Parsed: {}", b),
        Err(_) => println!("Must be exactly 'true' or 'false'"),
    }
    
    // 2. Flexible yes/no
    if get_confirmation("Continue? (yes/no)") {
        println!("Continuing...");
    } else {
        println!("Stopped.");
    }
    
    // 3. Default to false
    println!("Enable feature? (yes/no)");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    let enabled = input.trim().to_lowercase() == "yes";
    println!("Feature enabled: {}", enabled);
    
    // 4. Multiple conditions
    let agree = get_confirmation("Agree to terms?");
    let subscribe = get_confirmation("Subscribe to newsletter?");
    
    if agree && subscribe {
        println!("Both confirmed!");
    } else if agree {
        println!("Terms agreed, no subscription.");
    } else {
        println!("Terms not agreed.");
    }
}
```

## ⚡ Common Mistakes

```rust
// ❌ Case sensitivity not handled
let input = "True";
let bool_val: bool = input.parse().expect("Parse");  // PANIC! Must be lowercase

// ❌ Not trimming whitespace
let input = "true \n";  // From read_line
let bool_val: bool = input.parse().expect("Parse");  // PANIC! Whitespace matters

// ❌ Expecting 1/0 to work
let input = "1";
let bool_val: bool = input.parse().expect("Parse");  // PANIC! Only "true"/"false"

// ❌ Assuming any non-empty string is true
let input = "hello";
if !input.is_empty() {  // This is true, but...
    // User might have meant "no"!
}

// ✅ Correct patterns
// Always trim:
let bool_val: bool = input.trim().parse().expect("Parse");

// Case insensitive comparison:
let answer = input.trim().to_lowercase();
let is_yes = answer == "yes" || answer == "true";

// Safe with default:
let bool_val = input.trim().parse::<bool>().unwrap_or(false);

// Explicit matching:
match input.trim().to_lowercase().as_str() {
    "yes" | "y" | "true" => true,
    "no" | "n" | "false" => false,
    _ => {
        eprintln!("Invalid input, defaulting to false");
        false
    }
}
```

## 🎓 Advanced Pattern: Loop Until Valid

```rust
use std::io;

fn get_bool_from_user(prompt: &str) -> bool {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        
        match input.trim().to_lowercase().as_str() {
            "yes" | "y" | "true" => return true,
            "no" | "n" | "false" => return false,
            _ => println!("Please enter yes or no."),
        }
    }
}

fn main() {
    let confirmed = get_bool_from_user("Proceed? (yes/no)");
    println!("User confirmed: {}", confirmed);
}
```

## 🔄 Real-World Use Cases

**Installation wizard:**
```rust
let install_docs = get_confirmation("Install documentation?");
let create_desktop = get_confirmation("Create desktop shortcut?");
let add_to_path = get_confirmation("Add to PATH?");

println!("\nInstallation options:");
println!("  Documentation: {}", install_docs);
println!("  Desktop shortcut: {}", create_desktop);
println!("  Add to PATH: {}", add_to_path);
```

**Safety check:**
```rust
println!("⚠️  WARNING: This will delete all data!");
if !get_confirmation("Are you SURE? (yes/no)") {
    println!("Operation cancelled.");
    return;
}

println!("Last chance! Type 'yes' to confirm:");
// Second confirmation for dangerous operations
```

**Configuration:**
```rust
struct Config {
    auto_save: bool,
    notifications: bool,
    dark_mode: bool,
}

impl Config {
    fn from_user_input() -> Self {
        Config {
            auto_save: get_confirmation("Enable auto-save?"),
            notifications: get_confirmation("Enable notifications?"),
            dark_mode: get_confirmation("Use dark mode?"),
        }
    }
}
```

## 🎯 Success Criteria

- [ ] Can parse "true"/"false" strings to bool
- [ ] Understand case sensitivity requirement
- [ ] Can implement flexible yes/no parsing
- [ ] Handle invalid input gracefully
- [ ] Know when strict vs flexible parsing is appropriate

---

[← Previous Exercise](../ex063_use_user_number/README.md) | [Next Exercise →](../ex072_character_input/README.md)
