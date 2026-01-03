# Exercise 63: Use User Number ⭐

[← Previous Exercise](../ex060_parse_string_to_number/README.md) | [Next Exercise →](../ex071_boolean_from_string/README.md)

## 🎯 Goal

Perform calculations with numbers obtained from user input.

## 📝 Task

Get a number from the user and perform a mathematical operation (like doubling it or squaring it).

## 🎓 Concept

**Using parsed numbers** combines input, parsing, and computation:

```rust
use std::io;

let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");

let number: i32 = input.trim().parse().expect("Not a number");
let doubled = number * 2;  // Use the number!
println!("Result: {}", doubled);
```

**The complete flow:**
1. Read input as String
2. Parse String to number
3. Use number in calculations
4. Display result

**Why this matters:**
- Transforms text into usable data
- Enables interactive calculations
- Foundation for calculators, converters, games

## ✅ Solution Approach

### 🟢 Beginner Way - Simple calculation:
```rust
use std::io;

fn main() {
    println!("Enter a number:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    
    let number: i32 = input.trim().parse().expect("Not a number");
    
    // Use the number
    let doubled = number * 2;
    println!("{} doubled is {}", number, doubled);
}
```

*Trade-off:* ✅ Clear and straightforward  
⚠️ *Caveat:* Crashes on invalid input

### 🔵 Idiomatic Way - Multiple operations with validation:
```rust
use std::io;

fn main() {
    println!("Enter a number:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    
    match input.trim().parse::<i32>() {
        Ok(num) => {
            // Perform multiple calculations
            println!("Number: {}", num);
            println!("Doubled: {}", num * 2);
            println!("Squared: {}", num.pow(2));
            println!("Half: {}", num / 2);  // Integer division
            println!("Negative: {}", -num);
        }
        Err(_) => {
            eprintln!("Error: '{}' is not a valid integer", input.trim());
        }
    }
}
```

*Why better:* Graceful error handling, more operations  
*When to use:* Production code, user-facing applications

## 💡 Why This Matters

**Interactive calculators:**
```rust
// Temperature converter
println!("Enter Celsius:");
let celsius: f64 = get_number();
let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
println!("{}°C = {}°F", celsius, fahrenheit);
```

**Games and scoring:**
```rust
// High score calculator
println!("Enter your score:");
let score: i32 = get_number();
let bonus = score / 10;  // 10% bonus
println!("Score: {}, Bonus: {}, Total: {}", score, bonus, score + bonus);
```

**Financial calculations:**
```rust
// Simple interest calculator
println!("Enter principal amount:");
let principal: f64 = get_number();
let rate = 0.05;  // 5% interest
let interest = principal * rate;
println!("Interest earned: ${:.2}", interest);
```

## 🔄 Try These

```rust
use std::io;

fn get_number(prompt: &str) -> i32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    input.trim().parse().expect("Not a number")
}

fn main() {
    // 1. Basic arithmetic
    let a = get_number("Enter first number:");
    let b = get_number("Enter second number:");
    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    println!("{} / {} = {}", a, b, a / b);  // Integer division!
    println!("{} % {} = {}", a, b, a % b);  // Remainder
    
    // 2. Chained calculations
    let num = get_number("Enter a number:");
    let result = ((num + 10) * 2) - 5;
    println!("(({}  + 10) * 2) - 5 = {}", num, result);
    
    // 3. Type conversion for decimals
    let int_num = get_number("Enter integer:");
    let as_float = int_num as f64;  // Convert to float
    let half = as_float / 2.0;  // Now we can get decimal result
    println!("Half of {} is {}", int_num, half);
    
    // 4. Power calculations
    let base = get_number("Enter base:");
    let exp = get_number("Enter exponent:");
    let power = base.pow(exp as u32);  // pow takes u32
    println!("{} ^ {} = {}", base, exp, power);
}
```

## ⚡ Common Mistakes

```rust
// ❌ Using number as String
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");
let doubled = input * 2;  // ERROR: can't multiply String!

// ❌ Forgetting integer division truncates
let num = 7;
let half = num / 2;  // half is 3, not 3.5!
println!("Half: {}", half);  // Surprising!

// ❌ Overflow with large numbers
let num: i32 = 2_000_000_000;
let doubled = num * 2;  // OVERFLOW! Wraps around to negative

// ❌ Wrong type for power
let num = 5;
let squared = num.pow(2);  // ERROR if 2 isn't u32

// ✅ Correct patterns
// Parse first:
let number: i32 = input.trim().parse().expect("Invalid");
let doubled = number * 2;

// Use floats for fractions:
let num = 7.0;
let half = num / 2.0;  // 3.5

// Check for overflow:
let num = 2_000_000_000i64;  // Use i64 for large numbers
let doubled = num * 2;

// Cast for pow:
let squared = num.pow(2_u32);
```

## 🎓 Advanced Patterns

**Error handling with Option:**
```rust
fn get_number_safe(prompt: &str) -> Option<i32> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse().ok()
}

fn main() {
    if let Some(num) = get_number_safe("Enter a number:") {
        println!("Doubled: {}", num * 2);
    } else {
        println!("Invalid input!");
    }
}
```

**Reusable input function:**
```rust
fn prompt_and_parse<T: std::str::FromStr>(prompt: &str) -> Result<T, T::Err> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    input.trim().parse()
}

fn main() {
    match prompt_and_parse::<i32>("Enter number:") {
        Ok(n) => println!("You entered: {}", n),
        Err(_) => println!("Invalid number!"),
    }
}
```

## 🎯 Success Criteria

- [ ] Can read and parse user input to number
- [ ] Can perform arithmetic with parsed number
- [ ] Understand integer vs float division
- [ ] Can handle multiple operations on same number
- [ ] Understand overflow risks with large numbers

---

[← Previous Exercise](../ex060_parse_string_to_number/README.md) | [Next Exercise →](../ex071_boolean_from_string/README.md)
