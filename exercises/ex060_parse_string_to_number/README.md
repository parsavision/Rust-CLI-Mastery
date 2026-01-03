# Exercise 60: Parse String to Number ⭐

[← Previous Exercise](../ex059_complete_input_flow/README.md) | [Next Exercise →](../ex061_handle_parse_errors/README.md)

## 🎯 Goal

Convert String input to numeric types using `.parse()`.

## 📝 Task

Take user input as a String and convert it to an integer (i32).

## 🎓 Concept

**Parsing with turbofish syntax:**

```rust
let number: i32 = input.trim().parse().expect("Not a number");
// Or with turbofish:
let number = input.trim().parse::<i32>().expect("Not a number");
```

**Why .trim() first?**
```rust
let input = "42\n";  // User pressed Enter
let bad = input.parse::<i32>();  // Err - newline breaks parsing
let good = input.trim().parse::<i32>();  // Ok(42)
```

**parse() works for many types:**
```rust
"42".parse::<i32>()     // Ok(42)
"3.14".parse::<f64>()   // Ok(3.14)
"true".parse::<bool>()  // Ok(true)
```

## ✅ Solution Approach

### 🟢 Beginner Way - Using expect():
```rust
use std::io;

fn main() {
    println!("Enter a number:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    
    let number: i32 = input.trim().parse().expect("Please enter a number");
    
    println!("Your number doubled: {}", number * 2);
}
```

*Trade-off:* ⚠️ Crashes on invalid input  
*Why it works:* `.expect()` unwraps Result, panics on parse error

### 🔵 Idiomatic Way - Handle parse errors gracefully:
```rust
use std::io;

fn main() {
    println!("Enter a number:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    
    match input.trim().parse::<i32>() {
        Ok(number) => {
            println!("Your number doubled: {}", number * 2);
        }
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number", input.trim());
        }
    }
}
```

*Why better:* Doesn't crash, explains error to user  
*When to use:* Any user-facing input

## 💡 Why This Matters

**Type safety at parse time:**
```rust
// ❌ String math doesn't work
let input = "5";
let result = input + 3;  // ERROR: can't add integer to &str

// ✅ Parse to number first
let number: i32 = input.parse().expect("Not a number");
let result = number + 3;  // Works!
```

## 🔄 Try These

```rust
use std::io;

fn main() {
    let mut input = String::new();
    
    // Parse to integer
    println!("Enter an integer:");
    io::stdin().read_line(&mut input).expect("Failed");
    let int: i32 = input.trim().parse().expect("Not an int");
    println!("As integer: {}", int);
    
    // Parse to float
    input.clear();
    println!("Enter a decimal:");
    io::stdin().read_line(&mut input).expect("Failed");
    let float: f64 = input.trim().parse().expect("Not a float");
    println!("As float: {}", float);
}
```

## ⚡ Common Mistakes

```rust
// ❌ Forgetting to trim (newline breaks parsing)
let input = "42\n";
let number: i32 = input.parse().expect("Failed");  // ERROR!

// ❌ Not handling parse failure
let number: i32 = input.parse().unwrap();  // Crashes on bad input

// ✅ Correct parsing
let number: i32 = input.trim().parse().expect("Please enter a number");
```

## 🎯 Success Criteria

- [ ] Can parse String to i32 using `.parse()`
- [ ] Always `.trim()` before parsing
- [ ] Understand turbofish syntax: `parse::<i32>()`
- [ ] Handle parse errors with match or expect
- [ ] Know why parse() returns Result

---

[← Previous Exercise](../ex059_complete_input_flow/README.md) | [Next Exercise →](../ex061_handle_parse_errors/README.md)
