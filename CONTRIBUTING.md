# Contributing to Rust CLI Mastery

Thank you for your interest in improving this Rust learning curriculum! This guide will help you create or enhance exercises following our established patterns.

---

## 📋 Table of Contents

- [Exercise Structure](#exercise-structure)
- [Enhancement Guidelines](#enhancement-guidelines)
- [Creating New Exercises](#creating-new-exercises)
- [Writing Good Examples](#writing-good-examples)
- [Clippy Configuration](#clippy-configuration)
- [Testing Standards](#testing-standards)

---

## 🏗️ Exercise Structure

Each exercise (from #84 onwards) follows this structure:

```
exercises/exXXX_name/
├── Cargo.toml          # Standard Rust project config
├── README.md           # Enhanced learning documentation
├── src/
│   └── main.rs         # Starter code with TODO markers
└── .clippy.toml        # Optional: Exercise-specific clippy config
```

---

## 📚 Enhancement Guidelines

### The Four Pillars

Every enhanced exercise MUST include:

#### 1. **The "Why" and Trade-offs**
- Show TWO approaches: Beginner (simple) and Idiomatic (professional)
- Explain trade-offs honestly
- Don't just show code—explain reasoning

**Example:**
```markdown
### Beginner Approach
```rust
let item = vec[0];
```
**Trade-off:** ⚠️ Panics if vector is empty

### Idiomatic Approach
```rust
let item = vec.first().copied().unwrap_or_default();
```
**Why better:** No panic risk, explicit handling
```

#### 2. **Safety First**
- Identify dangerous patterns specific to this exercise
- Explain WHY they're dangerous (not just that they are)
- Show safe alternatives with reasoning

**Example:**
```markdown
### ⚠️ Risky Pattern
```rust
let value = &input[0..input.len()-2];  // Can panic!
```

**Why dangerous:**
- Panics if input < 2 characters
- Panics on UTF-8 boundaries
- Assumes format is always valid

### ✅ Safe Alternative
```rust
let value = input.trim().strip_suffix("kg")?;
```
**Why better:** Graceful failure, validates format
```

#### 3. **Clippy Insights**
- Show actual clippy output for common mistakes
- Explain what the lint catches
- Provide concrete fix

**Format:**
```markdown
Run `cargo clippy`:

```bash
warning: indexing may panic
  --> src/main.rs:12:13
   |
   = help: Consider using `.get(..)` to avoid panicking
```

**What this teaches:**
- `indexing_slicing`: Catches panic-prone array access
- Use `.get()` for safe indexing
```

#### 4. **Progressive Challenges**
- Always provide 3-4 challenge levels
- Each should take 5-30 minutes
- Make them concrete and testable

**Structure:**
```markdown
### Challenge 1: Eliminate Panics ⏱️ 5-10 min
**Tasks:**
- [ ] Replace `.unwrap()` with `.unwrap_or()`
- [ ] Test with empty input

### Challenge 2: Better Errors ⏱️ 10-15 min
**Tasks:**
- [ ] Return `Result<T, String>`
- [ ] Provide specific error messages

### Challenge 3: Idiomatic Rust ⏱️ 15-25 min
**Tasks:**
- [ ] Use `?` operator
- [ ] Pass all clippy lints

### Challenge 4: Tests ⏱️ 20-30 min (Optional)
**Tasks:**
- [ ] Add unit tests
- [ ] Cover edge cases
```

---

## 🆕 Creating New Exercises

### Step 1: Choose Exercise Number and Name

Follow the naming convention:
```
exXXX_descriptive_name
```

Examples:
- `ex084_parse_temperature`
- `ex156_match_exhaustiveness`
- `ex301_custom_error_types`

### Step 2: Identify the Core Concept

Ask yourself:
- What ONE concept does this exercise teach?
- What's the most common mistake beginners make here?
- What's the idiomatic Rust approach?

### Step 3: Create Directory Structure

```bash
mkdir -p exercises/ex084_parse_temperature/src
cd exercises/ex084_parse_temperature
```

### Step 4: Create Cargo.toml

Use this template:
```toml
[package]
name = "ex084_parse_temperature"
version = "0.1.0"
edition = "2024"

[dependencies]
# Add only if absolutely necessary
```

### Step 5: Write README.md

Copy from `EXERCISE_TEMPLATE.md` and fill in:
- Goal (what will they learn?)
- Beginner vs Idiomatic approaches
- Safety pitfalls specific to this exercise
- Relevant clippy lints
- 3-4 progressive challenges
- Key takeaways

### Step 6: Create Starter Code

In `src/main.rs`, provide:
```rust
use std::io::{self, Write};

fn main() {
    // TODO: Implement exercise
    // Hints:
    // 1. [First hint]
    // 2. [Second hint]
    
    println!("Exercise 84: [Name]");
}
```

**Guidelines for starter code:**
- Provide just enough structure
- Leave the core logic as TODO
- Include imports they'll need
- Add helpful hints

---

## ✍️ Writing Good Examples

### Do's ✅

1. **Keep examples minimal**
   ```rust
   // Good: Focused example
   let value = input.parse::<i32>()?;
   ```

2. **Show realistic scenarios**
   ```rust
   // Good: Realistic user input
   let temp = "23.5°C";
   ```

3. **Explain the "why"**
   ```markdown
   Use `.get()` instead of `[]` because it returns `Option` 
   instead of panicking on invalid indices.
   ```

4. **Compare approaches side-by-side**
   ```rust
   // Beginner
   let x = vec[0];
   
   // Idiomatic
   let x = vec.first()?;
   ```

### Don'ts ❌

1. **Don't overwhelm with complexity**
   ```rust
   // Bad: Too many concepts at once
   let result = input
       .trim()
       .split(',')
       .filter_map(|s| s.parse::<f64>().ok())
       .fold(0.0, |acc, x| acc + x);
   ```

2. **Don't use unrealistic examples**
   ```rust
   // Bad: Not realistic
   let data = vec![1, 2, 3];  // Where does this come from?
   ```

3. **Don't skip error handling**
   ```rust
   // Bad: Ignores errors
   let value = input.parse().unwrap();
   ```

4. **Don't explain Rust basics**
   ```markdown
   // Bad: Too basic for exercise 84+
   "A variable is declared with `let`..."
   ```

---

## 🔍 Clippy Configuration

### Global Configuration

The project-wide `.clippy.toml` should warn on dangerous patterns:

```toml
# .clippy.toml (project root)
disallowed-methods = []

# Warn on these for exercises 84+
warn = [
    "unwrap_used",
    "expect_used", 
    "indexing_slicing",
    "panic",
]
```

### Exercise-Specific Configuration

For early exercises, you may need to allow certain patterns:

```toml
# exercises/ex084_name/.clippy.toml
# Allow unwrap in this specific exercise
allow-unwrap-in = ["main"]
```

### Recommended Lints by Category

**Input Handling (Ex 84-100):**
- `indexing_slicing`
- `unwrap_used`
- `string_slice`

**Control Flow (Ex 151-200):**
- `single_match`
- `match_bool`
- `needless_return`

**Collections (Ex 201-250):**
- `needless_range_loop`
- `explicit_iter_loop`
- `manual_filter_map`

**Error Handling (Ex 301-350):**
- `expect_used`
- `unwrap_used`
- `map_unwrap_or`

---

## 🧪 Testing Standards

### Minimum Testing

Every exercise should be testable:

```bash
# Must compile
cargo build

# Must run
cargo run

# Must pass clippy (with allowed exceptions)
cargo clippy

# Must be formatted
cargo fmt --check
```

### Recommended Testing

For complex exercises, add tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path() {
        // Test valid input
    }

    #[test]
    fn test_error_case() {
        // Test invalid input
    }

    #[test]
    fn test_edge_case() {
        // Test boundaries
    }
}
```

### Test Coverage Goals

- **Basic exercises:** Compilation + manual testing
- **Intermediate exercises:** Unit tests for core functions
- **Advanced exercises:** Full test coverage (>80%)

---

## 📝 README Template Checklist

Use this checklist when creating exercise READMEs:

- [ ] **Goal**: Clear, 1-2 sentence objective
- [ ] **Why & Trade-offs**: Beginner vs Idiomatic comparison
- [ ] **Safety First**: Dangerous pattern + safe alternative
- [ ] **Clippy Insights**: Actual clippy output + explanation
- [ ] **Challenge 1**: Eliminate panics (5-10 min)
- [ ] **Challenge 2**: Better errors (10-15 min)
- [ ] **Challenge 3**: Idiomatic code (15-25 min)
- [ ] **Challenge 4**: Tests & polish (20-30 min, optional)
- [ ] **Task**: Original exercise description
- [ ] **Testing**: Commands to verify solution
- [ ] **Key Takeaways**: 3-4 bullet points
- [ ] **Further Reading**: Links to docs
- [ ] **Navigation**: Links to previous/next exercises

---

## 🎯 Quality Standards

### Before Submitting

Ensure your exercise meets these standards:

#### Code Quality
- [ ] All code examples compile
- [ ] No unused imports or variables
- [ ] Follows Rust naming conventions
- [ ] Clippy warnings are intentional (if any)

#### Documentation Quality
- [ ] Grammar and spelling checked
- [ ] Code blocks have syntax highlighting (```rust)
- [ ] Examples are realistic and relevant
- [ ] Links are valid and working

#### Educational Value
- [ ] Teaches ONE core concept clearly
- [ ] Progression is logical
- [ ] Challenges are concrete and achievable
- [ ] Safety concerns are explicitly addressed

---

## 🤝 Contribution Process

### 1. Create or Enhance Exercise
- Follow this guide
- Use EXERCISE_TEMPLATE.md as starting point

### 2. Test Thoroughly
```bash
cd exercises/exXXX_name
cargo build
cargo run
cargo test
cargo clippy
cargo fmt
```

### 3. Document Changes
- Add entry to exercise list in main README
- Update navigation links (previous/next)

### 4. Request Review
- Create pull request
- Describe what concept is taught
- List any clippy warnings and why they're allowed

---

## 💡 Tips for Great Exercises

1. **Start with common mistakes** - What do beginners get wrong?
2. **Show evolution** - How does code improve through iterations?
3. **Be honest about trade-offs** - No approach is perfect
4. **Make it practical** - Use realistic examples
5. **Test your own exercise** - Can you complete it in the expected time?

---

## 📚 Resources

- [EXERCISE_TEMPLATE.md](./EXERCISE_TEMPLATE.md) - Full template
- [Rust Book](https://doc.rust-lang.org/book/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

---

## ❓ Questions?

If you have questions about:
- **Exercise content**: Open a discussion
- **Template structure**: Reference existing enhanced exercises (84-93)
- **Technical issues**: Open an issue

Thank you for contributing to Rust CLI Mastery! 🦀
