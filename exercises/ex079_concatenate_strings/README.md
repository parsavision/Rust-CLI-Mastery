# Exercise 79: Concatenate Strings ⭐

[← Previous Exercise](../ex078_check_if_empty/README.md) | [Next Exercise →](../ex080_password_input_simulation/README.md)

## 🎯 Goal

Combine multiple strings together to create formatted output.

## 📝 Task

Learn different ways to join strings in Rust using `format!`, `+`, and `.push_str()`.

## 🎓 Concept

**String concatenation** joins text together:

```rust
let first = "Hello";
let second = "World";
let combined = format!("{} {}", first, second);
// combined is "Hello World"
```

**Multiple approaches:**
1. `format!()` - Most flexible, creates new String
2. `+` operator - Consumes first operand
3. `.push_str()` - Mutates existing String
4. `.join()` - For collections

## ✅ Solution Approach

### 🟢 Beginner Way - format! macro:
```rust
use std::io;

fn main() {
    println!("Enter your first name:");
    let mut first_name = String::new();
    io::stdin().read_line(&mut first_name).expect("Failed");
    
    println!("Enter your last name:");
    let mut last_name = String::new();
    io::stdin().read_line(&mut last_name).expect("Failed");
    
    let full_name = format!("{} {}", first_name.trim(), last_name.trim());
    println!("Full name: {}", full_name);
}
```

*Trade-off:* ✅ Readable, flexible, handles any type  
*When to use:* Most situations, especially with formatting

### 🔵 Idiomatic Way - Choose method based on use case:
```rust
use std::io;

fn main() {
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed");
    
    // format! for complex formatting
    let greeting = format!("Hello, {}! Welcome to Rust.", name.trim());
    println!("{}", greeting);
    
    // + for simple concatenation
    let message = "Your name is: ".to_string() + name.trim();
    println!("{}", message);
    
    // push_str for building incrementally
    let mut builder = String::from("User: ");
    builder.push_str(name.trim());
    builder.push_str(" (verified)");
    println!("{}", builder);
}
```

*Why better:* Uses most efficient method for each case  
*When to use:* Performance-sensitive code, building strings in loops

## 💡 Why This Matters

**Building messages:**
```rust
let username = "alice";
let score = 100;
let message = format!("Player {} scored {} points!", username, score);
println!("{}", message);
```

**File paths:**
```rust
let directory = "/home/user";
let filename = "document.txt";
let full_path = format!("{}/{}", directory, filename);
println!("Saving to: {}", full_path);
```

**User-friendly output:**
```rust
println!("Enter quantity:");
let mut qty = String::new();
io::stdin().read_line(&mut qty).expect("Failed");

println!("Enter price:");
let mut price = String::new();
io::stdin().read_line(&mut price).expect("Failed");

let qty_num: i32 = qty.trim().parse().unwrap();
let price_num: f64 = price.trim().parse().unwrap();

let summary = format!("{} items × ${:.2} = ${:.2}", 
    qty_num, price_num, qty_num as f64 * price_num);
println!("{}", summary);
```

## 🔄 Try These

```rust
use std::io;

fn main() {
    // 1. format! with multiple values
    let name = "Alice";
    let age = 30;
    let city = "Boston";
    let info = format!("{} is {} years old and lives in {}", name, age, city);
    println!("{}", info);
    
    // 2. + operator (consumes first string)
    let hello = String::from("Hello");
    let world = " World";
    let combined = hello + world;  // hello is moved!
    println!("{}", combined);
    // println!("{}", hello);  // ERROR: hello was moved
    
    // 3. push_str (mutates in place)
    let mut message = String::from("Count: ");
    for i in 1..=5 {
        message.push_str(&i.to_string());
        message.push(' ');
    }
    println!("{}", message);  // "Count: 1 2 3 4 5 "
    
    // 4. join for collections
    let words = vec!["Rust", "is", "awesome"];
    let sentence = words.join(" ");
    println!("{}", sentence);  // "Rust is awesome"
    
    // 5. Building from user input
    println!("\nEnter city:");
    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("Failed");
    
    println!("Enter country:");
    let mut country = String::new();
    io::stdin().read_line(&mut country).expect("Failed");
    
    let location = format!("{}, {}", city.trim(), country.trim());
    println!("Location: {}", location);
    
    // 6. Repeating strings
    let stars = "*".repeat(10);
    println!("{}", stars);  // "**********"
    
    let border = format!("{}\n{}\n{}", stars, "  Title  ", stars);
    println!("{}", border);
}
```

## ⚡ Common Mistakes

```rust
// ❌ Using + with &str on left
let result = "Hello" + " World";  // ERROR: can't add &str + &str

// ❌ Trying to use moved value
let s1 = String::from("Hello");
let s2 = s1 + " World";
println!("{}", s1);  // ERROR: s1 was moved

// ❌ Not converting to string first
let number = 42;
let text = "Answer: " + number;  // ERROR: can't add &str + i32

// ❌ Inefficient concatenation in loop
let mut result = String::new();
for i in 0..1000 {
    result = result + &i.to_string();  // Creates new String each time!
}

// ✅ Correct patterns
// Use format! for &str concatenation:
let result = format!("{} {}", "Hello", "World");

// Clone if you need to keep original:
let s1 = String::from("Hello");
let s2 = s1.clone() + " World";
println!("{} and {}", s1, s2);  // Both work

// Convert to String first:
let number = 42;
let text = format!("Answer: {}", number);

// Use push_str in loops:
let mut result = String::new();
for i in 0..1000 {
    result.push_str(&i.to_string());  // Mutates in place
}
```

## 🎓 Performance Comparison

```rust
// FAST: format! - allocates once with known size
let s = format!("{} {}", "Hello", "World");

// MEDIUM: + operator - may need reallocation
let s = String::from("Hello") + " " + "World";

// FAST: push_str - mutates in place (if capacity exists)
let mut s = String::from("Hello");
s.push_str(" World");

// SLOW: + in loop - creates new String each time
let mut s = String::new();
for word in vec!["a", "b", "c"] {
    s = s + word;  // Allocates new String each iteration!
}

// FAST: Loop with push_str - reuses allocation
let mut s = String::new();
for word in vec!["a", "b", "c"] {
    s.push_str(word);  // Mutates in place
}
```

## 🔄 Real-World Use Cases

**Building SQL queries:**
```rust
fn build_query(table: &str, columns: &[&str], condition: &str) -> String {
    let cols = columns.join(", ");
    format!("SELECT {} FROM {} WHERE {}", cols, table, condition)
}

let query = build_query("users", &["id", "name", "email"], "active = true");
println!("{}", query);
// "SELECT id, name, email FROM users WHERE active = true"
```

**Generating HTML:**
```rust
fn create_link(url: &str, text: &str) -> String {
    format!("<a href=\"{}\">{}</a>", url, text)
}

let link = create_link("https://rust-lang.org", "Learn Rust");
println!("{}", link);
```

**Building file names:**
```rust
use std::io;

fn main() {
    println!("Enter base filename:");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("Failed");
    
    println!("Enter extension:");
    let mut ext = String::new();
    io::stdin().read_line(&mut ext).expect("Failed");
    
    let filename = format!("{}.{}", base.trim(), ext.trim());
    println!("Full filename: {}", filename);
    
    // Add timestamp
    let timestamped = format!("{}_{}.{}", 
        base.trim(), 
        "20240103", 
        ext.trim()
    );
    println!("With timestamp: {}", timestamped);
}
```

**Log messages:**
```rust
fn log_event(level: &str, module: &str, message: &str) {
    let timestamp = "2024-01-03 10:30:00";  // Would use real timestamp
    let log_line = format!("[{}] [{}] {}: {}", 
        timestamp, level, module, message);
    println!("{}", log_line);
}

log_event("INFO", "auth", "User logged in successfully");
// [2024-01-03 10:30:00] [INFO] auth: User logged in successfully
```

**Building prompts:**
```rust
fn create_prompt(field_name: &str, required: bool) -> String {
    let req_marker = if required { " (required)" } else { "" };
    format!("Enter {}{}: ", field_name, req_marker)
}

print!("{}", create_prompt("username", true));
// "Enter username (required): "
```

## 🎯 Success Criteria

- [ ] Can use `format!()` macro for string building
- [ ] Understand `+` operator and ownership implications
- [ ] Know when to use `push_str()` for efficiency
- [ ] Can use `.join()` with collections
- [ ] Understand performance trade-offs of each method

---

[← Previous Exercise](../ex078_check_if_empty/README.md) | [Next Exercise →](../ex080_password_input_simulation/README.md)
