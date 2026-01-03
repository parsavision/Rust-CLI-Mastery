# Exercise 86: Confirm Action ✅

[← Previous Exercise](../ex085_error_recovery/README.md) | [Next Exercise →](../ex087_get_float_input/README.md)

## 🎯 Goal

Get user confirmation (yes/no) before proceeding with an action.

## 📝 Task

Ask user "Are you sure? (y/n)" and only continue if they confirm.

## 🎓 Concept

User confirmation patterns:
- **Safety first:** Prevent accidental actions
- **User control:** Let user decide
- **Clear options:** Simple y/n or yes/no

**Common use cases:**
- File deletion
- Configuration changes
- Destructive operations
- Payment confirmations

## ✅ Solution Approach

```rust
// Basic y/n confirmation
fn main() {
    println!("Are you sure? (y/n)");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    
    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => println!("Action confirmed!"),
        "n" | "no" => println!("Action cancelled."),
        _ => println!("Invalid response. Please enter 'y' or 'n'."),
    }
}
```

**Examples:**
```bash
$ ./confirm_action
Are you sure? (y/n)
y
Action confirmed!

$ ./confirm_action  
Are you sure? (y/n)
n
Action cancelled.
```

## 💡 Why Confirmation Matters

**Without confirmation:**
```rust
delete_file("important.dat");  // Deletes immediately!
```

**With confirmation:**
```rust
if confirm_action("Delete file?") {
    delete_file("important.dat");  // User chose to delete
}
```

## 🔄 Try These

```rust
// Reusable confirmation function
fn confirm_action(prompt: &str) -> bool {
    println!("{} (y/n)", prompt);
    // ... get input and return true/false
}

// Multiple confirmations
if confirm_action("Delete all files?") {
    if confirm_action("This is permanent! Continue?") {
        println!("Deleting files...");
        // Perform deletion
    }
}

// Custom responses
match input.trim().to_lowercase().as_str() {
    "yes" | "y" | "confirm" => println!("Confirmed"),
    "no" | "n" | "cancel" => println!("Cancelled"),
    _ => println!("Please enter 'yes' or 'no'"),
}
```

## ⚡ Common Patterns

**Case insensitive:**
```rust
match input.trim().to_lowercase().as_str() {
    "y" | "yes" => true,     // Accepts y, Y, yes, YES, etc.
    "n" | "no" => false,      // Accepts n, N, no, NO, etc.
    _ => {
        println!("Please enter 'y' or 'n'");
        false  // Try again or default to false
    }
}
```

**Default to safe option:**
```rust
let response = input.trim().to_lowercase();
let confirmed = match response.as_str() {
    "y" | "yes" => true,
    _ => false,  // Anything other than explicit 'yes' = no
};
```

## 🎯 Success Criteria

- [ ] Program asks user for confirmation
- [ ] Accepts y/n or yes/no variations
- [ ] Only continues on explicit confirmation
- [ ] Handles invalid input gracefully
- [ ] Program explains the action being confirmed

---

[← Previous Exercise](../ex085_error_recovery/README.md) | [Next Exercise →](../ex087_get_float_input/README.md)