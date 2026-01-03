# Exercise 55: Print User Input ⭐

[← Previous Exercise](../ex054_handle_result/README.md) | [Next Exercise →](../ex060_parse_string_to_number/README.md)

## 🎯 Goal

Echo user input back to the terminal - display what the user typed.

## 📝 Task

Read user input and print it back to demonstrate the complete input-output cycle.

## 🎓 Concept

**Printing user input** combines reading input and displaying it:

```rust
use std::io;

let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read");
println!("You entered: {}", input);
```

**Key components:**
- `read_line()` captures user input **including the newline** (`\n`)
- `println!()` displays the input
- The `{}` placeholder inserts the String value

**Why the newline matters:**
```rust
// User types: Hello<Enter>
// input contains: "Hello\n"
println!("You entered: {}", input);  // Output: "You entered: Hello\n" (extra line!)
```

## ✅ Solution Approach

### 🟢 Beginner Way - Direct echo:
```rust
use std::io;

fn main() {
    println!("Type something and press Enter:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    println!("You entered: {}", input);
    println!("Done!");
}
```

*Trade-off:* ✅ Simple and clear  
⚠️ *Caveat:* Output includes newline from user's Enter key, may look odd

### 🔵 Idiomatic Way - Trimmed output:
```rust
use std::io;

fn main() {
    println!("Type something and press Enter:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    // Trim whitespace for cleaner output
    println!("You entered: {}", input.trim());
    println!("Length: {} characters", input.trim().len());
}
```

*Why better:* Removes newline/whitespace for clean display  
*When to use:* Almost always when displaying user input

## 💡 Why This Matters

**Interactive programs need feedback:**

```rust
// ❌ Bad UX - No feedback
io::stdin().read_line(&mut input).expect("Failed");
// User doesn't know if program received input

// ✅ Good UX - Clear confirmation
io::stdin().read_line(&mut input).expect("Failed");
println!("Got it! You entered: {}", input.trim());
```

**Foundation for CLI tools:**
```rust
// Every interactive CLI follows this pattern:
// 1. Prompt user
// 2. Read input
// 3. Process input
// 4. Show result
println!("Enter your name:");
let mut name = String::new();
io::stdin().read_line(&mut name).expect("Failed");
println!("Hello, {}!", name.trim());
```

**Real-world examples:**
- Password prompts (read + echo as asterisks)
- Confirmation dialogs (read + validate yes/no)
- Form input (read + display summary)

## 🔄 Try These

```rust
use std::io;

fn main() {
    // 1. Simple echo
    println!("Enter text:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    println!("Echo: {}", input);  // Notice extra line!
    
    // 2. Trimmed echo
    println!("\nEnter more text:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed");
    println!("Echo: {}", input2.trim());  // Clean output!
    
    // 3. Show what was captured
    println!("\nRaw input has {} bytes", input2.len());
    println!("Trimmed input has {} chars", input2.trim().len());
    
    // 4. Multiple inputs
    println!("\nEnter first word:");
    let mut word1 = String::new();
    io::stdin().read_line(&mut word1).expect("Failed");
    
    println!("Enter second word:");
    let mut word2 = String::new();
    io::stdin().read_line(&mut word2).expect("Failed");
    
    println!("You entered: {} and {}", word1.trim(), word2.trim());
}
```

## ⚡ Common Mistakes

```rust
// ❌ Forgetting the newline exists
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");
println!("You entered: {}", input);  
// Output has awkward extra blank line!

// ❌ Reusing String without clearing
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");
println!("First: {}", input.trim());

io::stdin().read_line(&mut input).expect("Failed");  
println!("Second: {}", input.trim());  
// BUG: Contains BOTH inputs! "First\nSecond\n"

// ❌ Comparing without trimming
let mut answer = String::new();
io::stdin().read_line(&mut answer).expect("Failed");
if answer == "yes" {  // Never matches! answer is "yes\n"
    println!("Success!");
}

// ✅ Correct patterns
// Always trim for display:
println!("You entered: {}", input.trim());

// Clear before reusing:
input.clear();
io::stdin().read_line(&mut input).expect("Failed");

// Trim before comparing:
if answer.trim() == "yes" {
    println!("Success!");
}
```

## 🔄 Common Use Cases

**Confirmation prompts:**
```rust
println!("Are you sure? (yes/no)");
let mut answer = String::new();
io::stdin().read_line(&mut answer).expect("Failed");
println!("You answered: {}", answer.trim());
```

**Name collection:**
```rust
println!("What's your name?");
let mut name = String::new();
io::stdin().read_line(&mut name).expect("Failed");
println!("Nice to meet you, {}!", name.trim());
```

**Input validation feedback:**
```rust
println!("Enter a number:");
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");

match input.trim().parse::<i32>() {
    Ok(n) => println!("Valid number: {}", n),
    Err(_) => println!("'{}' is not a valid number", input.trim()),
}
```

## 🎯 Success Criteria

- [ ] Can read input with `read_line()`
- [ ] Can display input with `println!`
- [ ] Understand why `.trim()` is usually needed
- [ ] Can explain the newline character issue
- [ ] Program provides clear feedback to user

---

[← Previous Exercise](../ex054_handle_result/README.md) | [Next Exercise →](../ex060_parse_string_to_number/README.md)
