# Exercise 82: Multiple Choice Input ⭐

[← Previous Exercise](../ex081_repeat_until_valid/README.md) | [Next Exercise →](../ex083_input_with_units/README.md)

## 🎯 Goal

Create menu-driven interfaces where users select from numbered options.

## 📝 Task

Display a menu with multiple choices, get the user's selection, and execute the corresponding action.

## 🎓 Concept

**Menu-driven interfaces** are fundamental to CLI applications. They provide clear options and route user input to the correct functionality.

**Menu pattern structure:**
```rust
loop {
    // 1. Display menu
    // 2. Get user choice
    // 3. Match choice to action
    // 4. Execute action
    // 5. Option to quit or continue
}
```

**Why menus matter:**
- **User-friendly:** Clear visual options
- **Self-documenting:** User sees what's available
- **Error-proof:** Limits choices to valid options
- **Scalable:** Easy to add new features

## ✅ Solution Approach

### 🟢 Beginner Way - Simple if-else chain:
```rust
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    loop {
        println!("\n=== Menu ===");
        println!("1) Option A");
        println!("2) Option B");
        println!("3) Option C");
        println!("4) Quit");
        print!("Choose (1-4): ");
        io::stdout().flush().unwrap();
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<u8>() {
            Ok(1) => println!("You chose Option A"),
            Ok(2) => println!("You chose Option B"),
            Ok(3) => println!("You chose Option C"),
            Ok(4) => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter 1-4."),
        }
    }
}
```

*Trade-off:* Works well, uses `match` for routing, clear structure.

### 🔵 Idiomatic Way - String matching for flexibility:
```rust
use std::io::{self, Write};

fn main() {
    loop {
        println!("\n=== Main Menu ===");
        println!("1) Add item");
        println!("2) Remove item");
        println!("3) List items");
        println!("q) Quit");
        print!("Your choice: ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim().to_lowercase().as_str() {
            "1" | "add" => handle_add(),
            "2" | "remove" => handle_remove(),
            "3" | "list" => handle_list(),
            "q" | "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "" => continue,  // Empty input, show menu again
            _ => println!("❌ Invalid choice. Please try again."),
        }
    }
}

fn handle_add() {
    println!("✅ Adding item...");
}

fn handle_remove() {
    println!("🗑️  Removing item...");
}

fn handle_list() {
    println!("📋 Listing items...");
}
```

*Why better:* Accepts multiple input formats ("1" or "add"), functions for each action (cleaner), case-insensitive, handles empty input.

*When to use:* Production CLIs, when you want flexible input, when actions are complex enough to warrant separate functions.

## 💡 Why This Matters

Menu systems are the **backbone** of interactive CLIs:

1. **Git-style CLIs** - `git add`, `git commit`, `git push` (subcommands are menus)
2. **Package managers** - `npm install`, `cargo build` (commands are menu options)
3. **Interactive tools** - Database clients, system monitors, games
4. **Admin panels** - Server management, configuration tools
5. **Setup wizards** - Multi-step configuration with choices at each step

**Without menus:**
```bash
# ❌ User must know exact commands
$ mytool --add-item "thing" --with-flag --another-option
```

**With menus:**
```bash
# ✅ User is guided
$ mytool
=== Menu ===
1) Add item
2) Remove item
What would you like to do? 
```

## 🔄 Try These

```rust
// Experiment 1: Nested menus (submenu)
fn main() {
    loop {
        println!("\n=== Main Menu ===");
        println!("1) File Operations");
        println!("2) Settings");
        println!("3) Exit");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            "1" => file_menu(),  // Submenu!
            "2" => settings_menu(),
            "3" => break,
            _ => println!("Invalid"),
        }
    }
}

fn file_menu() {
    loop {
        println!("\n--- File Operations ---");
        println!("1) Create file");
        println!("2) Delete file");
        println!("3) Back to main menu");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            "1" => println!("Creating file..."),
            "2" => println!("Deleting file..."),
            "3" => break,  // Return to main menu
            _ => println!("Invalid"),
        }
    }
}

// Experiment 2: Menu with input parameters
match choice.trim() {
    "1" => {
        print!("Enter item name: ");
        io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        add_item(name.trim());
    }
    // ...
}

// Experiment 3: Colorized menu (conceptual)
println!("\n\x1b[1;36m=== Menu ===\x1b[0m");
println!("\x1b[32m1)\x1b[0m Add item");
println!("\x1b[32m2)\x1b[0m Remove item");
println!("\x1b[31m3)\x1b[0m Quit");

// Experiment 4: Menu with descriptions
println!("\n=== Task Manager ===");
println!("1) Add task      - Create a new task");
println!("2) List tasks    - Show all tasks");
println!("3) Complete task - Mark task as done");
println!("4) Delete task   - Remove a task");
println!("5) Exit");
```

## ⚡ Common Mistakes

```rust
// ❌ Forgetting to flush stdout before input
print!("Choose: ");
io::stdin().read_line(&mut input).unwrap();
// Bug: Prompt might not appear before waiting for input

// ❌ Not handling invalid input
let choice: u8 = input.trim().parse().unwrap();
// Bug: Crashes if user enters non-number!

// ❌ No way to exit the loop
loop {
    // ... menu ...
    // No break statement - user trapped forever!
}

// ❌ Not clearing input between iterations
let mut input = String::new();
loop {
    io::stdin().read_line(&mut input).unwrap();
    // Missing: input.clear()
    // Bug: Input accumulates!
}

// ✅ Correct pattern
loop {
    print_menu();
    
    let mut choice = String::new();  // Fresh each time
    io::stdin().read_line(&mut choice).unwrap();
    
    match choice.trim() {
        "1" => do_action_1(),
        "q" => break,  // Exit option!
        _ => println!("Invalid"),  // Handle bad input
    }
}
```

## 🎓 Advanced Patterns

### Function-based menu for reusability:
```rust
fn show_menu_and_get_choice(options: &[&str]) -> String {
    println!("\n=== Menu ===");
    for (i, option) in options.iter().enumerate() {
        println!("{}) {}", i + 1, option);
    }
    print!("Your choice: ");
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    choice.trim().to_string()
}

// Usage:
let options = ["Add item", "Remove item", "Quit"];
let choice = show_menu_and_get_choice(&options);
```

### Match with guards for complex conditions:
```rust
match choice.trim() {
    n if n.parse::<i32>().is_ok() => {
        let num = n.parse::<i32>().unwrap();
        if num >= 1 && num <= items.len() {
            select_item(num - 1);
        } else {
            println!("Number out of range");
        }
    }
    "q" => break,
    _ => println!("Invalid"),
}
```

### Enum-based menu for type safety:
```rust
enum MenuChoice {
    Add,
    Remove,
    List,
    Quit,
}

impl MenuChoice {
    fn from_input(input: &str) -> Option<Self> {
        match input.trim().to_lowercase().as_str() {
            "1" | "add" => Some(MenuChoice::Add),
            "2" | "remove" => Some(MenuChoice::Remove),
            "3" | "list" => Some(MenuChoice::List),
            "q" | "quit" => Some(MenuChoice::Quit),
            _ => None,
        }
    }
}

// Usage:
match MenuChoice::from_input(&choice) {
    Some(MenuChoice::Add) => handle_add(),
    Some(MenuChoice::Quit) => break,
    None => println!("Invalid choice"),
    // ...
}
```

## 🔄 Real-World Use Cases

1. **Database tools** - `psql`, `mysql` interactive shells
2. **System utilities** - `top`, `htop` interactive mode
3. **Game menus** - Start, Options, Quit
4. **Admin panels** - Server management with category menus
5. **Setup wizards** - Step-by-step configuration

## 🎯 Success Criteria

- [ ] Program displays clear menu options
- [ ] User can select from numbered or named choices
- [ ] Invalid input is handled gracefully
- [ ] Menu loops until user chooses to quit
- [ ] Each choice routes to correct action
- [ ] Can explain menu pattern structure
- [ ] Understands when to use submenus vs flat menus

---

[← Previous Exercise](../ex081_repeat_until_valid/README.md) | [Next Exercise →](../ex083_input_with_units/README.md)
