# Exercise 18: First Variable ⭐✅

[← Previous Exercise](../ex017_comments_multi_line/README.md) | [Next Exercise →](../ex019_print_variables/README.md)

## 🎯 Goal

Create your first variable and understand how to store data in Rust.

## 📝 Task

Create a variable with a value and print it using the `{}` placeholder.

## 🎓 Concept

Variables are named storage locations for data:

```rust
let name = "Rust";  // Store text in 'name' variable
let age = 25;       // Store number in 'age' variable
```

**Key points:**
- `let` keyword creates variables
- Variables are immutable by default
- Variables can hold different types of data

## ✅ Solution Approach

```rust
fn main() {
    let name = "Alice";     // Create string variable
    let age = 30;         // Create number variable
    
    println!("Name: {}", name);   // Use {} placeholder
    println!("Age: {}", age);
}
```

**Output:**
```
Name: Alice
Age: 30
```

## 💡 Why Variables Matter

**Without variables:**
```rust
println!("Alice is 30 years old");  // Hard to change
println!("Bob is 25 years old");   // Must rewrite everything
```

**With variables:**
```rust
let name = "Alice";     // Easy to change
let age = 30;
println!("{} is {} years old", name, age);
```

## 🔄 Try These

```rust
// Different types
let country = "Canada";     // String
let population = 38000000;  // Integer
let is_cold = true;         // Boolean
let percentage = 75.5;       // Float

// Multiple variables
let first_name = "John";
let last_name = "Doe";
let age = 42;
println!("{} {}, age {}", first_name, last_name, age);

// Variable names can describe content
let rust_year = "2015";
let rust_creator = "Mozilla";
println!("{} created in {}", rust_creator, rust_year);
```

## ⚡ Variable Naming Rules

✅ **Valid names:**
```rust
let my_variable = 5;
let user_name = "Bob";
let is_ready = true;
let _private = 10;  // Leading underscore
let var123 = 42;
```

❌ **Invalid names:**
```rust
let 1st_place = 1;     // Can't start with number
let my-variable = 5;      // Can't use hyphen
let let = 10;             // Can't use keywords
let my variable = 10;     // Can't use spaces
```

## 🎯 Success Criteria

- [ ] Create at least 1 variable using `let`
- [ ] Use `{}` placeholder to print variable value
- [ ] Program compiles without errors
- [ ] Output shows the stored value
- [ ] Can explain what a variable does

---

[← Previous Exercise](../ex017_comments_multi_line/README.md) | [Next Exercise →](../ex019_print_variables/README.md)