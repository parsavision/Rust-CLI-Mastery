# Exercise 20: Multiple Variables ✅

[← Previous Exercise](../ex019_print_variables/README.md) | [Next Exercise →](../ex021_variable_naming_rules/README.md)

## 🎯 Goal

Create and use multiple variables in a single program.

## 📝 Task

Create several different variables (strings, numbers) and use them throughout your program.

## 🎓 Concept

Working with multiple variables demonstrates:

```rust
// Declaration
let name = "Alice";
let age = 30;
let city = "New York";
let country = "USA";

// Usage throughout program
println!("Name: {}", name);
println!("Age: {}", age);
println!("Location: {}, {}", city, country);
```

**Benefits:**
- Organized data storage
- Reusable values
- Clear program structure
- Easy maintenance

## ✅ Solution Approach

```rust
fn main() {
    // Person information
    let first_name = "John";
    let last_name = "Doe";
    let age = 35;
    
    // Location information
    let city = "Seattle";
    let state = "WA";
    let zip_code = "98101";
    
    // Display information
    println!("Name: {} {}", first_name, last_name);
    println!("Age: {}", age);
    println!("Address: {}, {} {}", city, state, zip_code);
}
```

**Output:**
```
Name: John Doe
Age: 35
Address: Seattle, WA 98101
```

## 💡 Why Multiple Variables Matter

**Single-use variables:**
```rust
println!("Hello, Alice!");
println!("You are 30 years old");
println!("You live in New York");
// Hard-coded values, inflexible
```

**Multiple variables:**
```rust
let name = "Alice";
let age = 30;
let city = "New York";
println!("Hello, {}!", name);
println!("You are {} years old", age);
println!("You live in {}", city);
// Easy to change, reuse, consistent
```

## 🔄 Try These

```rust
// Student record
let student_id = "S12345";
let name = "Sarah Chen";
let major = "Computer Science";
let year = 3;
let gpa = 3.8;
println!("{} - {} - Year {} - GPA: {}", student_id, name, major, year, gpa);

// Product inventory
let product_name = "Wireless Mouse";
let price = 24.99;
let quantity = 15;
let category = "Electronics";
println!("{} ({}) - ${:.2} × {} = ${:.2}", product_name, category, price, quantity, price * quantity);

// Date formatting
let day = 3;
let month = "January";
let year = 2026;
let day_of_week = "Saturday";
println!("{}, {} {}, {} - {}", day_of_week, month, day, year);

// Game character stats
let character_name = "Arina";
let level = 15;
let health = 100;
let magic_power = 50;
let gold = 250;
println!("{} - Level {} - HP: {}/{} - MP: {}/{} - Gold: {}", 
           character_name, level, health, health, magic_power, magic_power, gold);
```

## ⚡ Variable Organization

**Group related variables:**
```rust
// Good: Organized by category
let first_name = "John";
let last_name = "Doe";
let age = 30;

let street = "123 Main St";
let city = "Boston";
let state = "MA";

// Poor: Random order
let city = "Boston";
let first_name = "John";
let state = "MA";
let last_name = "Doe";
let age = 30;
```

## 🎯 Success Criteria

- [ ] Create at least 3 variables
- [ ] Use each variable at least once
- [ ] Variables have different meaningful names
- [ ] Program compiles and displays all information
- [ ] Can organize variables logically

---

[← Previous Exercise](../ex019_print_variables/README.md) | [Next Exercise →](../ex021_variable_naming_rules/README.md)