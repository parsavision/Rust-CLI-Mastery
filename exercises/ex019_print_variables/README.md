# Exercise 19: Print Variables ✅

[← Previous Exercise](../ex018_first_variable/README.md) | [Next Exercise →](../ex020_multiple_variables/README.md)

## 🎯 Goal

Print multiple variables using multiple `{}` placeholders.

## 📝 Task

Create 2-3 variables and print them in a single statement using multiple placeholders.

## 🎓 Concept

Multiple placeholders in format strings:

```rust
let name = "Alice";
let age = 25;
println!("{} is {} years old", name, age);
//         ^            ^        ^
//         |            |        |
//       1st variable   2nd variable
```

**Placeholder rules:**
- `{}` represents one variable
- Order in println! = order of variables
- Count of `{}` must match count of variables

## ✅ Solution Approach

```rust
fn main() {
    let first_name = "Bob";
    let last_name = "Smith";
    let age = 42;
    
    // Multiple placeholders
    println!("{} {} is {} years old", first_name, last_name, age);
    
    // Different formats
    println!("Age: {}, Name: {} {}", age, first_name, last_name);
}
```

**Output:**
```
Bob Smith is 42 years old
Age: 42, Name: Bob Smith
```

## 💡 Why Multiple Placeholders Matter

**Without placeholders:**
```rust
println!("Name: " + first_name + ", Age: " + age.to_string());
// Ugly and inefficient!
```

**With placeholders:**
```rust
println!("Name: {}, Age: {}", first_name, age);
// Clean, readable, automatic type conversion
```

## 🔄 Try These

```rust
// Different variable combinations
let city = "Toronto";
let country = "Canada";
let population = 2800000;
println!("{}, {} has {} people", city, country, population);

// Numbers and calculations
let length = 10;
let width = 5;
let area = length * width;
println!("Rectangle {}x{} = {}", length, width, area);

// Mixed types
let product = "Laptop";
let price = 999.99;
let in_stock = true;
println!("Product: {} - ${:.2} - Available: {}", product, price, in_stock);

// Reusing variables
let item = "Apple";
let quantity = 5;
let price = 0.99;
println!("{} × {} = ${:.2}", item, quantity, quantity * price);
```

## ⚡ Common Mistakes

```rust
// ❌ Wrong order
println!("{} is {} years old", age, name);  // Age appears first!

// ❌ Mismatched count
println!("Name: {} Age: {}", name);     // Missing age variable!

// ❌ Extra variable
println!("{} {}", name, age, city);     // Too many variables!

// ✅ Correct matching
println!("{} is {} years old", name, age);  // Order matches!
```

## 🎯 Success Criteria

- [ ] Create at least 2 variables
- [ ] Use multiple `{}` placeholders in one println!
- [ ] Variable order matches placeholder order
- [ ] Program compiles and runs
- [ ] Can use placeholders with different variable types

---

[← Previous Exercise](../ex018_first_variable/README.md) | [Next Exercise →](../ex020_multiple_variables/README.md)