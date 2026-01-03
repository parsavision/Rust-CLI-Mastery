# Exercise 87: Get Float Input ⭐

[← Previous Exercise](../ex086_confirm_action/README.md) | [Next Exercise →](../ex088_multiple_float_inputs/README.md)

## 🎯 Goal

Read floating-point (decimal) numbers from user input.

## 📝 Task

Get a decimal number from user and display it with 2 decimal places.

## 🎓 Concept

Float input differs from integer input:

**Data types:**
- `f32` - 32-bit float (single precision)
- `f64` - 64-bit float (double precision, *default*)

**Parsing:**
```rust
let input = "3.14159";
let number: f64 = input.parse().expect("Not a valid number");
```

## ✅ Solution Approach

```rust
fn main() {
    println!("Enter a decimal number:");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    
    let number: f64 = input.trim().parse()
        .expect("Please enter a valid decimal number");
    
    println!("You entered: {:.2}", number);  // 2 decimal places
}
```

**Example:**
```bash
$ ./get_float_input
Enter a decimal number:
3.14159
You entered: 3.14
```

## 💡 Why Floats Matter

**Integers are limiting:**
```rust
let price = 3;     // Can't represent cents
let temperature = 98; // Can't represent fractions
```

**Floats allow precision:**
```rust
let price = 3.99;     // Can represent cents
let temperature = 98.6; // Can represent fractions
let pi = 3.14159;    // Can represent decimals
```

## 🔄 Try These

```rust
// Different float types
let single_precision: f32 = 3.14;
let double_precision: f64 = 3.14159265359;

// Math operations with floats
let radius = 5.0;
let area = 3.14159 * radius * radius;
println!("Circle area: {:.2}", area);

// User-friendly input prompts
println!("Enter your height in meters:");
let height: f64 = input.trim().parse().expect("Invalid height");

println!("Enter your weight in kg:");
let weight: f64 = input.trim().parse().expect("Invalid weight");

let bmi = weight / (height * height);
println!("BMI: {:.1}", bmi);
```

## ⚠️ Precision Considerations

**f32 vs f64:**
```rust
let single: f32 = 0.1 + 0.2;  // May equal 0.30000000004
let double: f64 = 0.1 + 0.2;  // More precise: 0.30000000000000004
```

**Floating point quirks:**
```rust
// These may not be exactly equal!
let result = 0.1 + 0.2;
println!("{}", result == 0.3);  // Might be false!

// Use tolerance for comparison
let tolerance = 0.0001;
let equal = (result - 0.3).abs() < tolerance;
```

## ⚡ Common Mistakes

```rust
// ❌ Using integer type for float data
let price: i32 = "3.99".parse().expect("Parse");  // Won't compile!

// ❌ Not handling decimal point in input
let input = "3,99";  // Uses comma instead of period
let number: f64 = input.parse().expect("Parse");  // Will fail!

// ❌ Assuming exact equality with floats
if result == 0.3 { /* may never execute */ }

// ✅ Correct float handling
let price: f64 = "3.99".parse().expect("Parse");
let approx_equal = (result - 0.3).abs() < 0.0001;
```

## 🎯 Success Criteria

- [ ] Program reads user input as float
- [ ] Uses `f64` type for double precision
- [ ] Handles invalid input with clear error
- [ ] Displays output with decimal formatting
- [ ] Can explain difference between f32 and f64

---

[← Previous Exercise](../ex086_confirm_action/README.md) | [Next Exercise →](../ex088_multiple_float_inputs/README.md)