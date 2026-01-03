# Exercise 83: Input with Units ⭐

[← Previous Exercise](../ex082_multiple_choice_input/README.md) | [Next Exercise →](../ex084_split_input/README.md)

## 🎯 Goal

Parse user input that contains both numbers and text units (like "5kg", "10m", "3.5hours").

## 📝 Task

Ask the user for their weight with a unit (e.g., "5kg"), extract the numeric value, and display it separately from the unit.

## 🎓 Concept

**Real-world input is messy.** Users don't just type numbers—they type "5kg", "10 meters", "3.5 hours". Your program needs to extract the meaningful parts.

**Unit parsing strategies:**
1. **Strip suffix:** Remove known unit from end
2. **Split by space:** Separate "5 kg" into ["5", "kg"]
3. **Pattern matching:** Use regex or manual parsing
4. **Validation:** Ensure both number and unit are valid

**Why unit parsing matters:**
- **Natural input:** Users type how they think ("5kg" not just "5")
- **Context:** Units provide meaning (5kg vs 5lbs)
- **Error prevention:** Detect mismatched units
- **User experience:** Accept flexible formats

## ✅ Solution Approach

### 🟢 Beginner Way - String slicing (fragile but simple):
```rust
use std::io::{self, Write};

fn main() {
    print!("How much do you weigh?\n(example: 5kg)\nEnter your weight: ");
    io::stdout().flush().expect("Failed to flush stdout");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let trimmed = input.trim();
    
    // Extract number part (everything except last 2 chars)
    let number_part = &trimmed[0..trimmed.len() - 2];
    
    println!("You weigh {} kilograms", number_part);
}
```

*Trade-off:* ⚠️ **Very fragile!** Panics if input is too short, assumes exactly 2-char unit, no validation.

### 🔵 Idiomatic Way - Safe string manipulation with validation:
```rust
use std::io::{self, Write};

fn main() {
    print!("Enter your weight (e.g., 75kg, 165lbs): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let trimmed = input.trim();
    
    // Try to strip known units
    if let Some(num_str) = trimmed.strip_suffix("kg") {
        match num_str.trim().parse::<f64>() {
            Ok(weight) => println!("Weight: {} kilograms", weight),
            Err(_) => println!("Invalid number before 'kg'"),
        }
    } else if let Some(num_str) = trimmed.strip_suffix("lbs") {
        match num_str.trim().parse::<f64>() {
            Ok(weight) => println!("Weight: {} pounds", weight),
            Err(_) => println!("Invalid number before 'lbs'"),
        }
    } else {
        println!("Unknown unit. Please use 'kg' or 'lbs'");
    }
}
```

*Why better:* `.strip_suffix()` is safe (returns `Option`), validates both number and unit, handles spaces, clear error messages.

*When to use:* Production code, when units are known in advance, when you need validation.

## 💡 Why This Matters

Unit parsing is **everywhere** in CLI tools:

1. **System commands** - `sleep 5s`, `timeout 30m`, `dd bs=1M`
2. **Package managers** - `npm install package@1.2.3` (version is a "unit")
3. **File tools** - `find . -size +100M`, `du -h` (human-readable units)
4. **Network tools** - `curl --max-time 10s`, `wget --limit-rate=1M`
5. **Configuration** - `timeout=30s`, `max_size=500MB`

**Without unit parsing:**
```bash
# ❌ User must separate values
$ backup --size 500 --unit MB
```

**With unit parsing:**
```bash
# ✅ Natural input
$ backup --size 500MB
```

## 🔄 Try These

```rust
// Experiment 1: Multiple unit support
fn parse_weight(input: &str) -> Option<(f64, String)> {
    let units = ["kg", "lbs", "g", "oz"];
    
    for unit in &units {
        if let Some(num_str) = input.trim().strip_suffix(unit) {
            if let Ok(value) = num_str.trim().parse::<f64>() {
                return Some((value, unit.to_string()));
            }
        }
    }
    None
}

// Usage:
match parse_weight("75kg") {
    Some((value, unit)) => println!("{} {}", value, unit),
    None => println!("Invalid format"),
}

// Experiment 2: Space-separated format ("5 kg")
fn parse_with_space(input: &str) -> Option<(f64, String)> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    if parts.len() == 2 {
        if let Ok(value) = parts[0].parse::<f64>() {
            return Some((value, parts[1].to_string()));
        }
    }
    None
}

// Experiment 3: Unit conversion
fn parse_and_convert_to_kg(input: &str) -> Option<f64> {
    if let Some(num_str) = input.strip_suffix("kg") {
        num_str.trim().parse().ok()
    } else if let Some(num_str) = input.strip_suffix("lbs") {
        num_str.trim().parse::<f64>().ok().map(|lbs| lbs * 0.453592)
    } else if let Some(num_str) = input.strip_suffix("g") {
        num_str.trim().parse::<f64>().ok().map(|g| g / 1000.0)
    } else {
        None
    }
}

println!("{:?} kg", parse_and_convert_to_kg("165lbs")); // ~74.84 kg

// Experiment 4: Time duration parsing
fn parse_duration(input: &str) -> Option<u64> {
    if let Some(num) = input.strip_suffix("s") {
        num.trim().parse().ok()
    } else if let Some(num) = input.strip_suffix("m") {
        num.trim().parse::<u64>().ok().map(|m| m * 60)
    } else if let Some(num) = input.strip_suffix("h") {
        num.trim().parse::<u64>().ok().map(|h| h * 3600)
    } else {
        None
    }
}

println!("{:?} seconds", parse_duration("5m")); // 300 seconds
```

## ⚡ Common Mistakes

```rust
// ❌ Direct slicing without validation
let input = "5kg";
let number = &input[0..input.len()-2];  // PANIC if input is "k"!

// ❌ Not trimming spaces
let input = "5kg ";
input.strip_suffix("kg");  // Returns None! Trailing space prevents match

// ❌ Assuming unit length
let number = &input[0..input.len()-2];  // Fails for "5m" (1-char unit)

// ❌ No number validation
let number_str = input.strip_suffix("kg").unwrap();
let weight = number_str.parse::<f64>().unwrap();  // Panics on "hellokg"!

// ✅ Correct pattern
if let Some(num_str) = input.trim().strip_suffix("kg") {
    match num_str.trim().parse::<f64>() {
        Ok(weight) => println!("Valid: {} kg", weight),
        Err(_) => println!("Invalid number"),
    }
} else {
    println!("Unit not recognized");
}
```

## 🎓 Advanced Patterns

### Generic unit parser:
```rust
struct Measurement {
    value: f64,
    unit: String,
}

fn parse_measurement(input: &str, valid_units: &[&str]) -> Option<Measurement> {
    for unit in valid_units {
        if let Some(num_str) = input.trim().strip_suffix(unit) {
            if let Ok(value) = num_str.trim().parse::<f64>() {
                return Some(Measurement {
                    value,
                    unit: unit.to_string(),
                });
            }
        }
    }
    None
}

// Usage:
let units = ["kg", "lbs", "g"];
if let Some(m) = parse_measurement("75kg", &units) {
    println!("{} {}", m.value, m.unit);
}
```

### Regex-based parsing (advanced):
```rust
use regex::Regex;

fn parse_with_regex(input: &str) -> Option<(f64, String)> {
    let re = Regex::new(r"^(\d+\.?\d*)\s*([a-zA-Z]+)$").unwrap();
    
    if let Some(caps) = re.captures(input.trim()) {
        let value = caps[1].parse::<f64>().ok()?;
        let unit = caps[2].to_string();
        return Some((value, unit));
    }
    None
}

// Handles: "5kg", "5 kg", "5.5kg", "5.5 kg"
```

### Builder pattern for complex parsing:
```rust
struct UnitParser {
    input: String,
}

impl UnitParser {
    fn new(input: String) -> Self {
        Self { input }
    }
    
    fn strip_unit(&self, unit: &str) -> Option<f64> {
        self.input
            .trim()
            .strip_suffix(unit)
            .and_then(|s| s.trim().parse().ok())
    }
    
    fn parse(&self) -> Option<(f64, String)> {
        for unit in &["kg", "lbs", "g", "oz"] {
            if let Some(value) = self.strip_unit(unit) {
                return Some((value, unit.to_string()));
            }
        }
        None
    }
}
```

## 🔄 Real-World Use Cases

1. **File size parsing** - "100MB", "1.5GB", "500KB"
2. **Time duration** - "30s", "5m", "2h", "1d"
3. **Distance/measurement** - "5km", "10mi", "3.5ft"
4. **Network bandwidth** - "100Mbps", "1Gbps"
5. **Temperature** - "25C", "77F", "298K"

## 🎯 Success Criteria

- [ ] Program accepts input with units (e.g., "5kg")
- [ ] Safely extracts numeric value
- [ ] Identifies the unit separately
- [ ] Handles invalid input gracefully (no panics)
- [ ] Trims whitespace correctly
- [ ] Can explain `.strip_suffix()` method
- [ ] Understands why string slicing is dangerous

---

[← Previous Exercise](../ex082_multiple_choice_input/README.md) | [Next Exercise →](../ex084_split_input/README.md)
