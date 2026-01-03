# Exercise 80: Password Input Simulation ⭐

[← Previous Exercise](../ex079_concatenate_strings/README.md) | [Next Exercise →](../ex081_repeat_until_valid/README.md)

## 🎯 Goal

Simulate password input with validation and attempt limiting (note: passwords will be visible in this basic version).

## 📝 Task

Create a password verification system with limited attempts and proper validation.

## 🎓 Concept

**Password verification** combines multiple concepts:
- Reading sensitive input
- String comparison
- Attempt counting
- Loop control

**Basic pattern:**
```rust
const MAX_ATTEMPTS: u8 = 3;
let mut attempts = 0;

loop {
    // Get password
    // Check password
    // Increment attempts
    // Break if correct or max attempts
}
```

**Note:** Real password input should hide characters (requires special crates like `rpassword`). This exercise focuses on the logic.

## ✅ Solution Approach

### 🟢 Beginner Way - Basic verification:
```rust
use std::io;

fn main() {
    let correct_password = "secret123";
    
    println!("Enter password:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    if input.trim() == correct_password {
        println!("✅ Access granted!");
    } else {
        println!("❌ Access denied!");
    }
}
```

*Trade-off:* ✅ Simple and clear  
⚠️ *Caveat:* No retry attempts, no validation

### 🔵 Idiomatic Way - Limited attempts with validation:
```rust
use std::io::{self, Write};

const MAX_ATTEMPTS: u8 = 3;
const CORRECT_PASSWORD: &str = "secret123";

fn main() {
    println!("🔒 Secure System - Login Required");
    
    for attempt in 1..=MAX_ATTEMPTS {
        print!("Enter password (attempt {}/{}): ", attempt, MAX_ATTEMPTS);
        io::stdout().flush().expect("Failed to flush");
        
        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("Failed to read");
        
        let password = password.trim();
        
        if password.is_empty() {
            println!("⚠️  Password cannot be empty!");
            continue;
        }
        
        if password == CORRECT_PASSWORD {
            println!("✅ Access granted! Welcome back.");
            return;
        }
        
        let remaining = MAX_ATTEMPTS - attempt;
        if remaining > 0 {
            println!("❌ Incorrect password. {} attempt(s) remaining.", remaining);
        }
    }
    
    println!("🔒 Access denied. Maximum attempts exceeded.");
}
```

*Why better:* Limited attempts, validation, user feedback  
*When to use:* Any authentication system, security-conscious applications

## 💡 Why This Matters

**Security basics:**
```rust
// Prevent brute force with attempt limits
const MAX_ATTEMPTS: u8 = 3;

// Don't reveal if username or password is wrong
println!("Invalid credentials");  // Good
// vs
println!("Password is wrong");    // Bad - reveals username exists
```

**User experience:**
```rust
// Show remaining attempts
println!("Incorrect. {} attempts remaining", remaining);

// Clear error messages
if password.is_empty() {
    println!("Password cannot be empty");
}
```

**Account lockout simulation:**
```rust
if attempts >= MAX_ATTEMPTS {
    println!("Account locked. Try again in 15 minutes.");
    // In real app: record timestamp, enforce lockout
}
```

## 🔄 Try These

```rust
use std::io::{self, Write};

const MAX_ATTEMPTS: u8 = 3;

fn main() {
    // 1. Multiple valid passwords
    let valid_passwords = vec!["admin123", "secret", "password"];
    
    for attempt in 1..=MAX_ATTEMPTS {
        print!("Password: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if valid_passwords.contains(&input.trim()) {
            println!("✅ Access granted!");
            return;
        }
        
        println!("❌ Incorrect ({}/{} attempts)", attempt, MAX_ATTEMPTS);
    }
    
    println!("🔒 Locked out!");
    
    // 2. Password strength check (before verification)
    println!("\n--- Password Strength Checker ---");
    print!("Enter password to check: ");
    io::stdout().flush().unwrap();
    
    let mut pwd = String::new();
    io::stdin().read_line(&mut pwd).unwrap();
    let pwd = pwd.trim();
    
    println!("Length: {} characters", pwd.len());
    println!("Has uppercase: {}", pwd.chars().any(|c| c.is_uppercase()));
    println!("Has lowercase: {}", pwd.chars().any(|c| c.is_lowercase()));
    println!("Has digit: {}", pwd.chars().any(|c| c.is_numeric()));
    println!("Has special: {}", pwd.chars().any(|c| !c.is_alphanumeric()));
    
    // 3. Progressive delay (simulate)
    println!("\n--- With Increasing Delay ---");
    for attempt in 1..=MAX_ATTEMPTS {
        print!("Password: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if input.trim() == "correct" {
            println!("✅ Success!");
            break;
        }
        
        println!("❌ Wrong! Waiting {} seconds...", attempt);
        // In real app: std::thread::sleep(Duration::from_secs(attempt as u64));
    }
}
```

## ⚡ Common Mistakes

```rust
// ❌ Not trimming password input
if input == "secret123" {  // Never matches if input has newline!
    // ...
}

// ❌ Revealing too much information
if username == "admin" && password != "secret" {
    println!("Username correct but password wrong");  // Security issue!
}

// ❌ No attempt limit
loop {
    // Get password
    // Check password
    // No break condition - infinite retries!
}

// ❌ Case-sensitive when shouldn't be
if input.trim() == "Admin" {  // Won't match "admin"
    // ...
}

// ❌ Hardcoded password visible in code
let password = "mySecretPassword123";  // Anyone can read source code!

// ✅ Correct patterns
// Always trim:
if input.trim() == CORRECT_PASSWORD {
    // ...
}

// Generic error messages:
println!("Invalid credentials");  // Doesn't reveal what's wrong

// Limit attempts:
for attempt in 1..=MAX_ATTEMPTS {
    // ...
}

// Case-insensitive username (not password!):
if username.trim().to_lowercase() == "admin" && password.trim() == SECRET {
    // ...
}

// Use constants:
const CORRECT_PASSWORD: &str = "secret";  // Still not secure, but clearer
```

## 🎓 Advanced Patterns

**Two-factor authentication simulation:**
```rust
use std::io::{self, Write};

fn verify_password() -> bool {
    print!("Password: ");
    io::stdout().flush().unwrap();
    
    let mut pwd = String::new();
    io::stdin().read_line(&mut pwd).unwrap();
    
    pwd.trim() == "secret"
}

fn verify_2fa() -> bool {
    println!("A code has been sent to your device.");
    print!("Enter 6-digit code: ");
    io::stdout().flush().unwrap();
    
    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();
    
    code.trim() == "123456"  // Simulated
}

fn main() {
    if verify_password() {
        println!("✅ Password correct!");
        
        if verify_2fa() {
            println!("✅ 2FA verified! Access granted.");
        } else {
            println!("❌ 2FA failed!");
        }
    } else {
        println!("❌ Password incorrect!");
    }
}
```

**Progressive lockout:**
```rust
fn calculate_delay(attempt: u8) -> u64 {
    // 1st wrong: 1s, 2nd: 2s, 3rd: 4s, 4th: 8s...
    2_u64.pow(attempt as u32 - 1)
}

fn main() {
    const MAX_ATTEMPTS: u8 = 5;
    
    for attempt in 1..=MAX_ATTEMPTS {
        // Get password...
        
        if attempt > 1 {
            let delay = calculate_delay(attempt);
            println!("⏱️  Waiting {} seconds before next attempt...", delay);
            // std::thread::sleep(Duration::from_secs(delay));
        }
    }
}
```

## 🔄 Real-World Considerations

**⚠️ What this exercise doesn't cover (needs real libraries):**
```rust
// ❌ Plain text password storage
const PASSWORD: &str = "secret";  // Never do this!

// ✅ Real applications use:
// - Password hashing (bcrypt, argon2)
// - Hidden input (rpassword crate)
// - Secure storage (environment variables, secret managers)
// - Rate limiting (track IP addresses)
// - Account lockout (timestamp-based)
```

**Better approach (concept only):**
```rust
// Pseudo-code for real password handling:
// 
// let stored_hash = hash_from_database(username);
// let input_hash = hash_password(input_password);
// 
// if input_hash == stored_hash {
//     // Grant access
// }
```

**Input hiding (requires rpassword crate):**
```rust
// With rpassword crate (not in this exercise):
// use rpassword::read_password;
// 
// print!("Password: ");
// io::stdout().flush().unwrap();
// let password = read_password().unwrap();  // Hidden input!
```

## 🎯 Success Criteria

- [ ] Limit login attempts (MAX_ATTEMPTS constant)
- [ ] Validate input (not empty)
- [ ] Provide clear feedback on success/failure
- [ ] Show remaining attempts
- [ ] Understand this is educational (not production-ready security)

**⚠️ Security Note:** This exercise demonstrates the *logic* of password verification. Real applications must:
1. Hash passwords (never store plain text)
2. Hide password input
3. Use HTTPS for transmission
4. Implement proper rate limiting
5. Follow OWASP security guidelines

---

[← Previous Exercise](../ex079_concatenate_strings/README.md) | [Next Exercise →](../ex081_repeat_until_valid/README.md)
