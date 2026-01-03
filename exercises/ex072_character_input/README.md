# Exercise 72: Character Input ⭐

[← Previous Exercise](../ex071_boolean_from_string/README.md) | [Next Exercise →](../ex074_default_with_unwrap_or/README.md)

## 🎯 Goal

Extract and work with individual characters from user input.

## 📝 Task

Read user input and get the first character using `.chars().next()`.

## 🎓 Concept

**Character extraction** from strings uses the `.chars()` iterator:

```rust
let text = "Hello";
let first_char = text.chars().next();  // Returns Option<char>
// first_char is Some('H')
```

**Key points:**
- `.chars()` returns an iterator over Unicode characters
- `.next()` gets the first item from the iterator
- Returns `Option<char>` (Some if exists, None if empty)

**Why Option?**
```rust
let empty = "";
let first = empty.chars().next();  // None - no characters!
```

## ✅ Solution Approach

### 🟢 Beginner Way - Direct extraction:
```rust
use std::io;

fn main() {
    println!("Enter some text:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    let first_char = input.chars().next().expect("No input!");
    println!("First character: {}", first_char);
}
```

*Trade-off:* ✅ Simple and straightforward  
⚠️ *Caveat:* Panics if input is empty, includes newline character

### 🔵 Idiomatic Way - Safe with trimming:
```rust
use std::io;

fn main() {
    println!("Enter some text:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    match input.trim().chars().next() {
        Some(ch) => {
            println!("First character: '{}'", ch);
            println!("Is alphabetic: {}", ch.is_alphabetic());
            println!("Is numeric: {}", ch.is_numeric());
            println!("Is uppercase: {}", ch.is_uppercase());
        }
        None => {
            println!("No input provided!");
        }
    }
}
```

*Why better:* Safe, trims whitespace, provides character info  
*When to use:* Production code, robust input handling

## 💡 Why This Matters

**Menu selection:**
```rust
println!("Choose an option:");
println!("(A) Add item");
println!("(D) Delete item");
println!("(Q) Quit");

let mut choice = String::new();
io::stdin().read_line(&mut choice).expect("Failed");

match choice.trim().chars().next() {
    Some('A') | Some('a') => println!("Adding item..."),
    Some('D') | Some('d') => println!("Deleting item..."),
    Some('Q') | Some('q') => println!("Goodbye!"),
    _ => println!("Invalid option!"),
}
```

**Password strength checking:**
```rust
println!("Enter password:");
let mut password = String::new();
io::stdin().read_line(&mut password).expect("Failed");

let first = password.trim().chars().next();
if let Some(ch) = first {
    if ch.is_numeric() {
        println!("⚠️ Password shouldn't start with a number!");
    }
}
```

**Initial validation:**
```rust
println!("Enter your name:");
let mut name = String::new();
io::stdin().read_line(&mut name).expect("Failed");

if let Some(first) = name.trim().chars().next() {
    if first.is_uppercase() {
        println!("✅ Name properly capitalized");
    } else {
        println!("⚠️ Name should start with capital letter");
    }
}
```

## 🔄 Try These

```rust
use std::io;

fn get_first_char(text: &str) -> Option<char> {
    text.trim().chars().next()
}

fn main() {
    // 1. Get first character safely
    println!("Enter text:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    if let Some(ch) = get_first_char(&input) {
        println!("First: '{}'", ch);
    } else {
        println!("Empty input!");
    }
    
    // 2. Character properties
    println!("\nEnter a character:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    
    if let Some(ch) = input.trim().chars().next() {
        println!("Character: '{}'", ch);
        println!("  Alphabetic: {}", ch.is_alphabetic());
        println!("  Numeric: {}", ch.is_numeric());
        println!("  Alphanumeric: {}", ch.is_alphanumeric());
        println!("  Whitespace: {}", ch.is_whitespace());
        println!("  Uppercase: {}", ch.is_uppercase());
        println!("  Lowercase: {}", ch.is_lowercase());
        println!("  ASCII: {}", ch.is_ascii());
    }
    
    // 3. Get nth character
    println!("\nEnter a word:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    
    let second = input.trim().chars().nth(1);  // Zero-based: 0, 1, 2...
    match second {
        Some(ch) => println!("Second character: '{}'", ch),
        None => println!("Input too short!"),
    }
    
    // 4. Count characters
    let char_count = input.trim().chars().count();
    println!("Total characters: {}", char_count);
}
```

## ⚡ Common Mistakes

```rust
// ❌ Not trimming - includes newline!
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");
let first = input.chars().next();  // First char might be ' ' or '\n'!

// ❌ Using indexing instead of chars
let input = "Hello";
let first = input[0];  // ERROR: can't index str by integer!

// ❌ Not handling empty input
let first = input.chars().next().unwrap();  // PANIC if empty!

// ❌ Assuming bytes equal characters
let text = "🦀";  // Crab emoji
let first_byte = text.as_bytes()[0];  // Just part of UTF-8 encoding!
let first_char = text.chars().next();  // Correctly gets '🦀'

// ✅ Correct patterns
// Always trim:
let first = input.trim().chars().next();

// Safe handling:
match input.trim().chars().next() {
    Some(ch) => println!("Got: {}", ch),
    None => println!("Empty!"),
}

// Or with if let:
if let Some(ch) = input.trim().chars().next() {
    println!("Got: {}", ch);
}

// With default:
let first = input.trim().chars().next().unwrap_or('?');
```

## 🎓 Working with Multiple Characters

```rust
use std::io;

fn main() {
    println!("Enter text:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    // Get first 3 characters
    let first_three: Vec<char> = input.trim()
        .chars()
        .take(3)
        .collect();
    println!("First 3 chars: {:?}", first_three);
    
    // Check if starts with specific character
    let starts_with_h = input.trim()
        .chars()
        .next()
        .map(|ch| ch.to_lowercase().next() == Some('h'))
        .unwrap_or(false);
    println!("Starts with 'h': {}", starts_with_h);
    
    // Get last character
    let last = input.trim().chars().last();
    if let Some(ch) = last {
        println!("Last character: '{}'", ch);
    }
}
```

## 🔄 Real-World Use Cases

**Single-key menu:**
```rust
fn show_menu() -> char {
    println!("\n=== Menu ===");
    println!("(N)ew");
    println!("(O)pen");
    println!("(S)ave");
    println!("(Q)uit");
    print!("Choice: ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    input.trim()
        .chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .next()
        .unwrap()
}
```

**Y/N confirmation:**
```rust
fn confirm(prompt: &str) -> bool {
    println!("{} (y/n)", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    matches!(
        input.trim().chars().next().map(|c| c.to_lowercase().next()),
        Some(Some('y'))
    )
}
```

**Grade input:**
```rust
println!("Enter grade (A-F):");
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");

match input.trim().chars().next() {
    Some('A') => println!("Excellent! 90-100%"),
    Some('B') => println!("Good! 80-89%"),
    Some('C') => println!("Average. 70-79%"),
    Some('D') => println!("Below average. 60-69%"),
    Some('F') => println!("Failing. <60%"),
    _ => println!("Invalid grade!"),
}
```

## 🎯 Success Criteria

- [ ] Can extract first character with `.chars().next()`
- [ ] Understand why it returns Option<char>
- [ ] Always trim input before extracting characters
- [ ] Can handle empty input gracefully
- [ ] Know difference between chars() and bytes()

---

[← Previous Exercise](../ex071_boolean_from_string/README.md) | [Next Exercise →](../ex074_default_with_unwrap_or/README.md)
