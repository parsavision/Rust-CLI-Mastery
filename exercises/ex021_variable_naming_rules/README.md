# Exercise 21: Variable Naming Rules ✅

[← Previous Exercise](../ex020_multiple_variables/README.md) | [Next Exercise →](../ex022_integer_types/README.md)

## 🎯 Goal

Understand what makes a valid variable name in Rust.

## 📝 Task

Create variables with different naming patterns to test what's allowed.

## 🎓 Concept

Rust variable naming follows these rules:

### ✅ Valid Names
- **Letters and numbers:** `my_var`, `var123`, `data1`
- **Underscores allowed:** `my_variable`, `user_name`, `count_by_2`
- **Must start with letter:** `a1`, not `1a`
- **Case sensitive:** `myVar` ≠ `myvar`
- **Unicode supported:** `имя`, `名前`, `café_name`

### ❌ Invalid Names
- **Can't start with number:** `1st_place`, `2d_array`
- **No spaces or hyphens:** `my-var`, `user name`
- **Can't use keywords:** `let`, `fn`, `struct`, `impl`
- **No special symbols:** `my@var`, `name#`, `price$`

## ✅ Solution Approach

```rust
fn main() {
    // Valid variable names
    let my_variable = 42;
    let user_name = "Alice";
    let _private = 10;         // Leading underscore
    let var123 = 100;
    let caffé_name = "Café";  // Unicode
    
    println!("Values: {}, {}, {}, {}, {}", 
             my_variable, user_name, var123, caffé_name);
}
```

## 💡 Why Naming Matters

**Good names:**
```rust
let user_age = 25;           // Clear what it represents
let total_price = 99.99;       // Descriptive
let is_authenticated = true;   // Boolean clearly named
```

**Poor names:**
```rust
let x = 25;                  // What does x mean?
let d = 99.99;                // Unclear abbreviation
let flag = true;              // Flag for what?
```

**Rust convention:** snake_case
```rust
// ✅ Follows convention
let first_name = "John";
let last_name = "Doe";
let years_of_experience = 5;

// ❌ Doesn't follow convention  
let firstName = "John";
let lastName = "Doe";
let yearsOfExperience = 5;
```

## 🔄 Try These

```rust
// Test valid patterns
let temp_value = 100;
let counter = 0;
let is_ready = false;
let _internal = 42;

// Test your naming creativity
let my_awesome_variable = "Rust is fun!";
let number_12345 = 12345;
let customer_data_record = "some data";

// Try invalid patterns (to see errors)
// let 1st_place = 1;     // Uncomment to see error
// let my-var = 10;          // Uncomment to see error
// let fn = "function";     // Uncomment to see error
```

## ⚡ Common Mistakes

```rust
// ❌ Starting with number
let 123abc = 10;  // Compile error!

// ❌ Using keywords
let let = 5;       // Compile error!

// ❌ Spaces in names
let my var = 10;    // Compile error!

// ✅ Valid alternatives
let abc123 = 10;
let my_var = 5;
let let_value = 5;
```

## 🎯 Success Criteria

- [ ] Create variables with valid naming patterns
- [ ] Use snake_case convention
- [ ] Program compiles without naming errors
- [ ] Can explain naming rules
- [ ] Can distinguish valid vs invalid names

---

[← Previous Exercise](../ex020_multiple_variables/README.md) | [Next Exercise →](../ex022_integer_types/README.md)