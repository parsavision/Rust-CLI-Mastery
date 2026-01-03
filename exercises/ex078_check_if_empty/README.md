# Exercise 78: Check if Empty ⭐

[← Previous Exercise](../ex075_compare_user_input/README.md) | [Next Exercise →](../ex079_concatenate_strings/README.md)

## 🎯 Goal

Validate user input by checking if it's empty or contains only whitespace.

## 📝 Task

Use `.is_empty()` and `.trim()` to detect empty input before processing.

## 🎓 Concept

**`.is_empty()` checks if a string has zero length:**

```rust
let text = "";
if text.is_empty() {
    println!("Empty!");  // This prints
}
```

**Important distinction:**
```rust
let spaces = "   ";
spaces.is_empty()        // false - has characters!
spaces.trim().is_empty() // true - only whitespace
```

**Why it matters:**
- Prevents processing invalid input
- Catches accidental Enter presses
- Distinguishes between no input and whitespace

## ✅ Solution Approach

### 🟢 Beginner Way - Basic empty check:
```rust
use std::io;

fn main() {
    println!("Enter your name:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    if input.trim().is_empty() {
        println!("You didn't enter anything!");
    } else {
        println!("Hello, {}!", input.trim());
    }
}
```

*Trade-off:* ✅ Simple validation  
*When to use:* Basic input validation

### 🔵 Idiomatic Way - Loop until valid:
```rust
use std::io;

fn get_non_empty_input(prompt: &str) -> String {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        
        let trimmed = input.trim();
        if trimmed.is_empty() {
            println!("⚠️  Input cannot be empty. Please try again.");
        } else {
            return trimmed.to_string();
        }
    }
}

fn main() {
    let name = get_non_empty_input("Enter your name:");
    println!("Hello, {}!", name);
}
```

*Why better:* Guarantees non-empty input, user-friendly prompts  
*When to use:* Required fields, production applications

## 💡 Why This Matters

**Form validation:**
```rust
println!("Enter username:");
let mut username = String::new();
io::stdin().read_line(&mut username).expect("Failed");

if username.trim().is_empty() {
    eprintln!("Error: Username is required!");
    return;
}
```

**Preventing logic errors:**
```rust
println!("Enter file name:");
let mut filename = String::new();
io::stdin().read_line(&mut filename).expect("Failed");

if filename.trim().is_empty() {
    eprintln!("Cannot save: no filename provided");
} else {
    // Safe to use filename
    save_file(filename.trim());
}
```

**Optional vs required:**
```rust
println!("Enter description (optional):");
let mut description = String::new();
io::stdin().read_line(&mut description).expect("Failed");

let desc = if description.trim().is_empty() {
    "No description"
} else {
    description.trim()
};

println!("Description: {}", desc);
```

## 🔄 Try These

```rust
use std::io;

fn main() {
    // 1. Basic empty check
    println!("Enter text:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    if input.trim().is_empty() {
        println!("Empty input detected!");
    } else {
        println!("You entered: {}", input.trim());
    }
    
    // 2. Length check
    println!("\nEnter at least 5 characters:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed");
    
    let trimmed = input.trim();
    if trimmed.is_empty() {
        println!("Error: Input is required");
    } else if trimmed.len() < 5 {
        println!("Error: Too short (need 5 chars, got {})", trimmed.len());
    } else {
        println!("Valid input: {}", trimmed);
    }
    
    // 3. Required vs optional fields
    println!("\nEnter your name (required):");
    let name = loop {
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed");
        let trimmed = temp.trim();
        
        if trimmed.is_empty() {
            println!("Name cannot be empty. Try again:");
        } else {
            break trimmed.to_string();
        }
    };
    
    println!("\nEnter your nickname (optional):");
    let mut nickname = String::new();
    io::stdin().read_line(&mut nickname).expect("Failed");
    
    println!("Name: {}", name);
    if nickname.trim().is_empty() {
        println!("Nickname: (none)");
    } else {
        println!("Nickname: {}", nickname.trim());
    }
    
    // 4. Empty vs whitespace
    let test_cases = vec!["", "   ", "\n", "\t", "hello"];
    for case in test_cases {
        println!("\n'{}' (length {})", case.escape_default(), case.len());
        println!("  .is_empty(): {}", case.is_empty());
        println!("  .trim().is_empty(): {}", case.trim().is_empty());
    }
}
```

## ⚡ Common Mistakes

```rust
// ❌ Checking length before trim
let input = "   \n";
if input.is_empty() {  // false - has whitespace!
    // Never executes
}

// ❌ Not handling empty after parse
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");
let number: i32 = input.trim().parse().expect("Failed");  // Panics on empty!

// ❌ Confusing empty string with None
let input = "";
// input is "", not None!
// It's an empty string, not absence of string

// ❌ Using len() == 0 instead of is_empty()
if input.len() == 0 {  // Works but less clear
    // ...
}

// ✅ Correct patterns
// Always trim first:
if input.trim().is_empty() {
    // Catches empty and whitespace-only
}

// Handle empty before parsing:
if input.trim().is_empty() {
    println!("Please enter a number");
} else {
    match input.trim().parse::<i32>() {
        Ok(n) => println!("Number: {}", n),
        Err(_) => println!("Invalid number"),
    }
}

// Use is_empty() for clarity:
if input.trim().is_empty() {  // More readable
    // ...
}
```

## 🎓 Advanced Validation Patterns

**Validation function:**
```rust
fn validate_input(input: &str, min_len: usize) -> Result<&str, String> {
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        Err("Input cannot be empty".to_string())
    } else if trimmed.len() < min_len {
        Err(format!("Input too short (minimum {} characters)", min_len))
    } else {
        Ok(trimmed)
    }
}

fn main() {
    println!("Enter password (min 8 chars):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    match validate_input(&input, 8) {
        Ok(password) => println!("Valid password entered"),
        Err(msg) => eprintln!("Error: {}", msg),
    }
}
```

**Builder pattern for validation:**
```rust
struct InputValidator {
    min_length: Option<usize>,
    max_length: Option<usize>,
    required: bool,
}

impl InputValidator {
    fn new() -> Self {
        Self {
            min_length: None,
            max_length: None,
            required: false,
        }
    }
    
    fn required(mut self) -> Self {
        self.required = true;
        self
    }
    
    fn min_length(mut self, len: usize) -> Self {
        self.min_length = Some(len);
        self
    }
    
    fn validate(&self, input: &str) -> Result<(), String> {
        let trimmed = input.trim();
        
        if self.required && trimmed.is_empty() {
            return Err("This field is required".to_string());
        }
        
        if let Some(min) = self.min_length {
            if trimmed.len() < min {
                return Err(format!("Must be at least {} characters", min));
            }
        }
        
        Ok(())
    }
}
```

## 🔄 Real-World Use Cases

**Registration form:**
```rust
fn register_user() {
    let username = get_required_input("Username:");
    let email = get_required_input("Email:");
    
    println!("Bio (optional):");
    let mut bio = String::new();
    io::stdin().read_line(&mut bio).expect("Failed");
    
    println!("\nAccount created!");
    println!("Username: {}", username);
    println!("Email: {}", email);
    
    if !bio.trim().is_empty() {
        println!("Bio: {}", bio.trim());
    }
}

fn get_required_input(prompt: &str) -> String {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        
        if !input.trim().is_empty() {
            return input.trim().to_string();
        }
        
        println!("This field is required!");
    }
}
```

**Search functionality:**
```rust
fn search() {
    println!("Enter search query:");
    let mut query = String::new();
    io::stdin().read_line(&mut query).expect("Failed");
    
    if query.trim().is_empty() {
        println!("Search cancelled (no query entered)");
        return;
    }
    
    println!("Searching for: {}", query.trim());
    // Perform search...
}
```

**Configuration:**
```rust
struct Config {
    name: String,
    description: Option<String>,
}

impl Config {
    fn from_user_input() -> Self {
        let name = get_required_input("Project name:");
        
        println!("Description (optional):");
        let mut desc = String::new();
        io::stdin().read_line(&mut desc).expect("Failed");
        
        let description = if desc.trim().is_empty() {
            None
        } else {
            Some(desc.trim().to_string())
        };
        
        Config { name, description }
    }
}
```

## 🎯 Success Criteria

- [ ] Can check if string is empty with `.is_empty()`
- [ ] Understand difference between empty and whitespace-only
- [ ] Always trim before checking empty
- [ ] Can loop until valid non-empty input received
- [ ] Distinguish required vs optional fields

---

[← Previous Exercise](../ex075_compare_user_input/README.md) | [Next Exercise →](../ex079_concatenate_strings/README.md)
