# Exercise 51: Import Statement ⭐

[← Previous Exercise](../ex050_review_exercise/README.md) | [Next Exercise →](../ex052_create_empty_string/README.md)

## 🎯 Goal

Learn to import modules from Rust's standard library.

## 📝 Task

Import the `io` module to prepare for reading user input.

## 🎓 Concept

Rust's standard library (`std`) is divided into modules:

### Why Import?
- **Namespace access:** Bring modules into scope
- **Type availability:** Use types like `String`, `io::Result`
- **Function access:** Use functions like `println!`, `io::stdin()`

### Module Structure
```
std::      // Root of standard library
├── io     // Input/output operations
├── fs      // File system operations
├── collections // Data structures
├── vec     // Vector operations
└── ...     // Many more modules
```

## ✅ Solution Approach

```rust
// Import the io module from standard library
use std::io;

fn main() {
    println!("Exercise 51: Import Statement");
    println!("Successfully imported std::io module!");
}
```

## 💡 Why Import Matters

**Without import:**
```rust
fn main() {
    std::io::stdin()  // Must write full path every time
        .read_line(&mut input);
}
```

**With import:**
```rust
use std::io;  // Bring io module into scope

fn main() {
    io::stdin()      // Shorter, cleaner
        .read_line(&mut input);
}
```

## 🔄 Try These

```rust
// Multiple imports
use std::io;
use std::fs;
use std::collections::HashMap;

// Selective import (bring specific items)
use std::io::{stdin, stdout, Write};  // Only import what you need

// Import with alias
use std::collections::HashMap as Map;  // Use Map instead of HashMap

// Import all (use sparingly)
use std::prelude::*;  // Common types and traits

// Import specific types
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
```

## ⚡ Common Import Patterns

**File operations:**
```rust
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, BufReader};
use std::path::Path;
```

**Data structures:**
```rust
use std::collections::{HashMap, HashSet, VecDeque};
use std::vec::Vec;
```

**Input/Output:**
```rust
use std::io::{stdin, stdout, BufRead, BufWriter, Write};
use std::env;  // Command line arguments
```

## ⚠️ Import Guidelines

### ✅ Good Practices
```rust
// Be specific - only import what you use
use std::io::{stdin, stdout, Write};

// Group related imports together
use std::fs;
use std::path;

// Use aliases for clarity
use std::collections::HashMap as ConfigMap;
```

### ❌ Avoid These
```rust
// Don't import everything unless needed
use std::prelude::*;  // Only if you really need it

// Don't import things you don't use
use std::collections::LinkedList;  // If you never use LinkedList
```

## 🎓 Module System Benefits

**Organization:**
- Related functionality grouped together
- Clear structure and hierarchy
- Avoids naming conflicts

**Efficiency:**
- Only compile what you actually import
- Faster compilation times
- Cleaner code

**Discoverability:**
- Standard library is well-documented
- Find existing solutions instead of reinventing
- Learn Rust idioms through std lib

## 🎯 Success Criteria

- [ ] Program imports `std::io` module
- [ ] Program compiles and runs
- [ ] Can explain what an import does
- [ ] Understand when imports are needed
- [ ] Ready to use imported module in next exercises

---

[← Previous Exercise](../ex050_review_exercise/README.md) | [Next Exercise →](../ex052_create_empty_string/README.md)