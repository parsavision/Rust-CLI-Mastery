# Exercise 53: Read Input ⭐

[← Previous Exercise](../ex052_create_empty_string/README.md) | [Next Exercise →](../ex054_handle_result/README.md)

## 🎯 Goal

Learn how to read user input from the terminal using stdin.

## 📝 Task

Read a line of text from the user and store it in a String.

## 🎓 Concept

**Reading from stdin (standard input):**

```rust
use std::io;

let mut input = String::new();
io::stdin().read_line(&mut input);
```

**How it works:**
1. `io::stdin()` - Gets handle to standard input
2. `.read_line()` - Reads until user presses Enter
3. `&mut input` - Mutable reference to store the text
4. Returns `Result<usize>` - Number of bytes read, or error

**Why `&mut`?**
```rust
// ❌ Won't work - read_line needs to modify the String
io::stdin().read_line(input);

// ✅ Correct - gives mutable reference
io::stdin().read_line(&mut input);
```

## ✅ Solution Approach

### 🟢 Beginner Way - Using `.expect()`:
```rust
use std::io;

fn main() {
    println!("What is your name?");
    
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    println!("You entered: {}", name);
}
```

*Trade-off:* ⚠️ Program crashes on I/O error (rare but possible)  
*Why it works:* `.expect()` handles Result, panics on Err

### 🔵 Idiomatic Way - Handle Result properly:
```rust
use std::io::{self, BufRead};

fn main() {
    println!("What is your name?");
    
    let stdin = io::stdin();
    let mut input = String::new();
    
    match stdin.read_line(&mut input) {
        Ok(bytes_read) => {
            println!("Read {} bytes: {}", bytes_read, input.trim());
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
        }
    }
}
```

*Why better:* Graceful error handling, doesn't crash  
*When to use:* Production code, user-facing applications

## 💡 Why This Matters

**read_line includes newline:**
```rust
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");

// If user types "Alice" and presses Enter
println!("'{}'", input);  // Prints: 'Alice\n'
println!("'{}'", input.trim());  // Prints: 'Alice'
```

**Multiple reads append:**
```rust
let mut buffer = String::new();

io::stdin().read_line(&mut buffer).expect("Failed");  
println!("First read: {}", buffer);  // "Hello\n"

io::stdin().read_line(&mut buffer).expect("Failed");
println!("Second read: {}", buffer);  // "Hello\nWorld\n"

// Use .clear() to reset:
buffer.clear();
io::stdin().read_line(&mut buffer).expect("Failed");
println!("After clear: {}", buffer);  // "New\n"
```

## 🔄 Try These

```rust
use std::io;

fn main() {
    // 1. Basic input reading
    let mut input = String::new();
    println!("Enter something:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    println!("You entered: {:?}", input);  // Debug print shows \n
    
    // 2. Multiple inputs
    input.clear();
    println!("Enter your age:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    println!("Age string: {:?}", input);
    
    // 3. See bytes read
    input.clear();
    println!("Type a sentence:");
    let bytes = io::stdin().read_line(&mut input).expect("Failed to read");
    println!("Read {} bytes", bytes);
    
    // 4. Without expect (match on Result)
    input.clear();
    println!("One more input:");
    match io::stdin().read_line(&mut input) {
        Ok(n) => println!("Read {} bytes: '{}'", n, input.trim()),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## ⚡ Common Mistakes

```rust
use std::io;

// ❌ Forgetting to import io module
let mut input = String::new();
stdin().read_line(&mut input);  // ERROR: cannot find `stdin`

// ❌ Not passing mutable reference
io::stdin().read_line(input);  // ERROR: expected &mut String

// ❌ Ignoring the Result
io::stdin().read_line(&mut input);  // WARNING: unused Result

// ❌ Not clearing between multiple reads
let mut buffer = String::new();
for _ in 0..3 {
    io::stdin().read_line(&mut buffer).expect("Failed");
    // buffer keeps growing with each read!
}

// ✅ Correct input reading
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read");
println!("Input: {}", input.trim());  // Trim the newline
```

## 🛡️ Safety First

**What can go wrong with read_line?**

```rust
// ⚠️ Rare but possible failures:
// - stdin is closed
// - stdin is redirected and source fails
// - Out of memory (very large input)
// - Invalid UTF-8 (read_line expects valid UTF-8)

// ✅ Handle these gracefully:
match io::stdin().read_line(&mut input) {
    Ok(_) => { /* process input */ },
    Err(e) if e.kind() == io::ErrorKind::InvalidData => {
        eprintln!("Invalid UTF-8 in input");
    },
    Err(e) => {
        eprintln!("Unexpected error: {}", e);
    }
}
```

## 🎯 Success Criteria

- [ ] Can read user input with `read_line()`
- [ ] Understand why `&mut` reference is needed
- [ ] Know that newline is included in result
- [ ] Handle Result with `.expect()` or `match`
- [ ] Can read multiple inputs with `.clear()`

---

[← Previous Exercise](../ex052_create_empty_string/README.md) | [Next Exercise →](../ex054_handle_result/README.md)
