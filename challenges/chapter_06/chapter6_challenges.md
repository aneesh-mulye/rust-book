# Chapter 6 Programming Challenges — The Rust Book

These challenges exercise enum definition, variants with and without data (tuple, struct-like, unit), pattern matching with `match`, exhaustive matching, `Option<T>`, `if let`, `let...else`, and methods on enums. Each tier raises the number of moving parts you must hold in working memory simultaneously.

All challenges use only material from Chapters 1–6. No `Vec`, `HashMap`, `Result`, `trait`, or anything beyond Chapter 6. Every challenge is a program to write — no paper exercises.

---

## Tier 1: Enums Without Data, Basic `match`

### Challenge 1.1 — Direction Navigator

Define an enum `Direction` with variants `North`, `East`, `South`, `West`.

Write these functions:
- `opposite(dir: &Direction) -> Direction`
- `turn_right(dir: &Direction) -> Direction`
- `turn_left(dir: &Direction) -> Direction`

In `main`, start facing `North`. Apply this sequence of maneuvers: right, right, left, right, opposite. Print the direction after each step.

**Expected output:**
```
Start:    North
Right:    East
Right:    South
Left:     East
Right:    South
Opposite: North
```

*You'll need `#[derive(Debug)]` on the enum to print it.*

---

### Challenge 1.2 — Planet Weight Calculator

Define an enum `Planet` with eight variants: `Mercury`, `Venus`, `Earth`, `Mars`, `Jupiter`, `Saturn`, `Uranus`, `Neptune`.

Write a function `surface_gravity_ratio(planet: &Planet) -> f64` that returns the planet's surface gravity relative to Earth (e.g., Mercury ≈ 0.38, Jupiter ≈ 2.53 — look up approximate values or invent plausible ones).

Write a function `weight_on(earth_weight: f64, planet: &Planet) -> f64`.

In `main`, define an earth weight of `75.0` kg. Create an array containing all eight planets and iterate over it, printing the weight on each.

**Expected output (approximate):**
```
Mercury:  28.5 kg
Venus:    68.0 kg
Earth:    75.0 kg
Mars:     28.1 kg
Jupiter: 189.8 kg
Saturn:   79.9 kg
Uranus:   63.8 kg
Neptune:  84.8 kg
```

*Note: you cannot iterate over an enum's variants automatically in Rust. You must construct the array manually. This is a deliberate friction point — feel it, and understand why.*

---

## Tier 2: Variants with Data, Destructuring

### Challenge 2.1 — Shape Area Calculator

Define:
```rust
enum Shape {
    Circle(f64),             // radius
    Rectangle(f64, f64),     // width, height
    RightTriangle(f64, f64), // base, height
}
```

Write a function `area(shape: &Shape) -> f64` that matches on the shape and computes the area (circle: `π * r²`, rectangle: `w * h`, triangle: `0.5 * b * h`). Use `std::f64::consts::PI`.

Write a function `describe(shape: &Shape)` that prints a one-line description including the dimensions and the computed area. Call `area()` inside `describe()`.

Create an array of five shapes with varied dimensions. Iterate and describe each. Then find and print which shape has the largest area.

---

### Challenge 2.2 — Card Hand Scorer

Define:
```rust
enum Card {
    Number(u8),  // 2–10
    Jack,
    Queen,
    King,
    Ace,
}
```

Write a method `value(&self) -> u8` that returns the card's point value: `Number` returns its inner value, face cards return 10, Ace returns 11.

Write a function `hand_value(hand: &[Card]) -> u8` that sums the values of all cards in a slice.

Write a function `is_bust(hand: &[Card]) -> bool` — true if hand value exceeds 21.

In `main`, create two hands as arrays:
- Hand A: `[Number(10), Ace]` — should be 21
- Hand B: `[King, Number(7), Number(8)]` — should be 25, bust

Print each hand's value and whether it's a bust.

**Expected output:**
```
Hand A: value = 21, bust = false
Hand B: value = 25, bust = true
```

---

### Challenge 2.3 — UI Event Dispatcher

Define:
```rust
enum Event {
    Click { x: i32, y: i32 },
    KeyPress(char),
    TextInput(String),
    Scroll(i32),
    Close,
}
```

This enum uses all four variant kinds: struct-like (`Click`), single-value tuple (`KeyPress`, `Scroll`), owned-data tuple (`TextInput`), and unit (`Close`).

Write a function `handle(event: &Event)` that prints a different response for each variant:
- `Click`: if `x` is in 0..200 and `y` is in 0..50, print `"Button area clicked"`. Otherwise print the coordinates.
- `KeyPress`: if the character is a letter, print it; if it's a digit, print `"Number key"`. Use `char::is_alphabetic()` and `char::is_numeric()`.
- `TextInput`: print the text with its character count.
- `Scroll`: print `"up"` or `"down"` depending on sign.
- `Close`: print `"Window closed"`.

Process an array of 6–8 events and handle each.

*Pay attention to what you get in each match arm when you match on `&Event`. The `TextInput` arm gives you a `&String` — you're borrowing, not moving.*

---

## Tier 3: `Option<T>`

### Challenge 3.1 — Safe Division and Square Root

Write two functions:
- `safe_divide(a: f64, b: f64) -> Option<f64>` — returns `None` if `b` is zero.
- `safe_sqrt(x: f64) -> Option<f64>` — returns `None` if `x` is negative. Use `f64::sqrt()`.

In `main`, use `match` to handle the results. Test with several inputs including the failure cases. For each, print the result or a meaningful error message.

**Expected output:**
```
10 / 3 = 3.3333333333333335
10 / 0 = undefined (division by zero)
sqrt(25) = 5
sqrt(-4) = undefined (negative input)
```

---

### Challenge 3.2 — First Match Finder

Write a function `find_first_above(numbers: &[i32], threshold: i32) -> Option<usize>` that returns the *index* of the first element strictly greater than `threshold`, or `None` if no such element exists.

Write a second function `find_first_even(numbers: &[i32]) -> Option<i32>` that returns the first even *value* (not index), or `None`.

In `main`, test both functions with the array `[3, 7, 2, 9, 4, 11, 6]` and several thresholds. Use `match` to handle all results.

**Expected output:**
```
First above 5: Some found at index 1 (value 7)
First above 20: None found
First even: 2
```

---

### Challenge 3.3 — Chained Optionality

Write three functions:
- `get(arr: &[i32], index: usize) -> Option<i32>` — returns `None` if index is out of bounds, otherwise `Some(value)`.
- `reciprocal(n: i32) -> Option<f64>` — returns `None` if `n` is zero, otherwise `Some(1.0 / n as f64)`.
- `classify(x: f64) -> &'static str` — returns `"small"` if `x > 0.5`, `"large"` if `x <= 0.5`, for a reciprocal.

Given the array `[0, 5, -3, 0, 8]`, write a function `lookup_and_classify(arr: &[i32], index: usize)` that:
1. Gets the element (might fail: out of bounds).
2. Computes its reciprocal (might fail: zero).
3. Classifies the result.

Use **nested `match`** to handle each layer of `Option`. Print a different message for each failure mode versus success.

Test with indices `1` (→ `Some(5)` → `Some(0.2)` → `"small"`), `0` (→ `Some(0)` → `None`), and `10` (→ `None`).

**Expected output:**
```
Index 1: reciprocal of 5 is 0.2 (small)
Index 0: element is 0 — cannot compute reciprocal
Index 10: index out of bounds
```

*This nested matching is deliberately verbose. You'll learn much cleaner ways to chain Options later (`map`, `and_then`, `?`). For now, experience the pain so you'll appreciate the solution.*

---

### Challenge 3.4 — Struct with Optional Fields

Define:
```rust
struct SensorReading {
    sensor_id: u32,
    temperature: Option<f64>,
    humidity: Option<f64>,
}
```

Implement methods:
- `SensorReading::new(id: u32) -> SensorReading` — both readings start as `None`.
- `with_temperature(self, t: f64) -> SensorReading` — builder-style, sets temperature.
- `with_humidity(self, h: f64) -> SensorReading` — sets humidity.
- `is_complete(&self) -> bool` — true if both readings are `Some`.
- `heat_index(&self) -> Option<f64>` — returns `None` if either reading is missing. If both are present, compute a simplified heat index: `temperature + 0.05 * humidity`. This method must handle four cases (both Some, either None, both None) but only one produces a value.
- `display(&self)` — prints the sensor ID and each reading, showing `"--"` for missing values.

In `main`, create three sensors using the builder pattern:
1. One with both readings.
2. One with only temperature.
3. One with neither.

Call `display()` and `heat_index()` on each.

**Expected output:**
```
Sensor 1: temp=23.5, humidity=65.0, heat index=26.75
Sensor 2: temp=19.0, humidity=--, heat index=--
Sensor 3: temp=--, humidity=--, heat index=--
```

---

## Tier 4: Methods on Enums, `if let`, `let...else`, Composition

### Challenge 4.1 — Temperature Converter with Methods

Define:
```rust
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}
```

Implement methods:
- `to_celsius(&self) -> f64`
- `to_fahrenheit(&self) -> f64`
- `is_freezing(&self) -> bool` — true if the temperature is at or below 0°C (convert first).
- `warmer_than(&self, other: &Temperature) -> bool` — converts both to Celsius and compares. This requires calling `to_celsius()` on both `self` and `other`, exercising methods calling methods.

Test with a variety of temperatures in mixed units:
```rust
let t1 = Temperature::Celsius(100.0);
let t2 = Temperature::Fahrenheit(98.6);
let t3 = Temperature::Kelvin(233.15);
```

Print each in all three units. Then compare pairs.

---

### Challenge 4.2 — Selective Processing with `if let`

You have sensor data where some readings may have failed:

```rust
let readings: [Option<f64>; 8] = [
    Some(23.5), None, Some(22.1), Some(24.0),
    None, Some(21.8), None, Some(23.0),
];
```

Write a program that iterates over the readings and:
1. Uses `if let` to process only the `Some` values — sum them and count them.
2. Uses `if let` with `else` to also count the `None` values.
3. Computes and prints the average of valid readings, plus how many failed.

Then write a function `first_valid(readings: &[Option<f64>]) -> Option<f64>` that returns the first `Some` value. Use a `for` loop with `if let` and early return.

**Expected output:**
```
Valid readings: 5, sum: 114.4, average: 22.88
Failed readings: 3
First valid: 23.5
```

---

### Challenge 4.3 — Happy Path with `let...else`

Write a function that computes BMI from optional inputs:

```rust
fn compute_bmi(weight_kg: Option<f64>, height_m: Option<f64>) -> Option<f64>
```

Use `let...else` to extract each value, returning `None` early if either is missing. On the happy path, compute `weight / (height * height)` and return `Some(result)`.

Write a second function `classify_bmi(bmi: f64) -> &'static str` that returns `"underweight"`, `"normal"`, `"overweight"`, or `"obese"` based on standard thresholds.

In `main`, test with:
1. Both values present → should compute and classify.
2. Weight missing → should return `None`.
3. Height missing → should return `None`.

Use `match` on the result in `main` to print either the BMI with classification or `"insufficient data"`.

**Expected output:**
```
Weight: 70 kg, Height: 1.75 m → BMI: 22.9 (normal)
Weight: --, Height: 1.80 m → insufficient data
Weight: 90 kg, Height: -- → insufficient data
```

---

### Challenge 4.4 — Vehicle Fleet Analyzer

Define:
```rust
enum FuelType {
    Gasoline,
    Diesel,
    Electric,
    Hybrid(f64),  // percentage that is electric (0.0–1.0)
}

struct Vehicle {
    name: String,
    fuel: FuelType,
    efficiency: f64,  // km per liter (or per kWh for electric)
    odometer: f64,
}
```

Implement methods on `Vehicle`:
- `Vehicle::new(name: &str, fuel: FuelType, efficiency: f64) -> Vehicle` — odometer starts at 0.
- `is_green(&self) -> bool` — true for Electric, or Hybrid with ≥ 50% electric.
- `fuel_label(&self) -> &str` — returns a human-readable label. Hybrid should include the percentage (use `match` on the fuel field and format within the arm).
- `drive(&mut self, km: f64)` — adds to odometer.

Wait — `fuel_label` can't easily return a formatted `&str` for Hybrid because a formatted string would be a new allocation. Let me make it `fuel_label(&self)` that prints instead. Or return a `String`.

Actually, for the non-Hybrid cases it can return `&str` literals. For Hybrid it would need to return a `String`. The return type would need to be `String` to accommodate both. Let me just have it be a `display` method that prints.

Let me rethink this method: make it `describe(&self)` that prints everything.

Implement on `Vehicle`:
- `Vehicle::new(name: &str, fuel: FuelType, efficiency: f64) -> Vehicle`
- `is_green(&self) -> bool`
- `cost_per_km(&self) -> f64` — use match on fuel type. Gasoline: `1.5 / efficiency`, Diesel: `1.3 / efficiency`, Electric: `0.4 / efficiency`, Hybrid: weighted average based on the electric percentage.
- `drive(&mut self, km: f64)`
- `describe(&self)`

Create a fleet of 4 vehicles. Drive each a different distance. Print descriptions and find the cheapest to operate (lowest cost_per_km) and the greenest.

This exercises: struct with enum field, methods that match on the enum, mutable methods, composing results across multiple vehicles.

---

## Tier 5: Integration Challenges

### Challenge 5.1 — Door State Machine

Define:
```rust
enum DoorState {
    Locked,
    Closed,
    Open,
}

enum Action {
    InsertKey,
    Turn,
    Push,
    Pull,
    RemoveKey,
}
```

Write a function `transition(state: DoorState, action: &Action) -> DoorState` that encodes these rules:
- `Locked` + `Turn` → `Closed` (unlock)
- `Closed` + `Push` → `Open`
- `Closed` + `Turn` → `Locked`
- `Open` + `Pull` → `Closed`
- Everything else → state unchanged (return current state)

Match on the **tuple** `(&state, action)` to handle the combinations concisely. Use a catch-all arm `_ => state` for all the no-op transitions.

In `main`, start with `Locked`. Process this sequence of actions: `[InsertKey, Turn, Push, Pull, Turn, RemoveKey]`. Print the state after each action.

Then add a `describe(state: &DoorState)` method and a `requires_key(action: &Action) -> bool` function. Before each action, check if it requires a key and print a note.

**Expected output:**
```
[InsertKey] Locked → Locked (no state change)
[Turn]      Locked → Closed
[Push]      Closed → Open
[Pull]      Open → Closed
[Turn]      Closed → Locked
[RemoveKey] Locked → Locked (no state change)
```

*This challenge exercises matching on a tuple of two enums — a powerful pattern the book doesn't show directly but that falls naturally out of `match` syntax.*

---

### Challenge 5.2 — Inventory with Pricing Rules

Define:
```rust
enum Category {
    Electronics,
    Clothing,
    Food { perishable: bool },
}

enum Condition {
    New,
    Refurbished(u8),  // quality 0–100
    Damaged(String),   // reason
}

struct Item {
    name: String,
    base_price: f64,
    category: Category,
    condition: Condition,
    in_stock: bool,
}
```

Implement methods on `Item`:
- `Item::new(name: &str, price: f64, category: Category, condition: Condition) -> Item` — `in_stock` defaults to `true`.
- `effective_price(&self) -> f64`:
  - `New` → full price
  - `Refurbished(q)` → price scaled by `q / 100`
  - `Damaged(_)` → 10% of price
- `is_sellable(&self) -> bool` — not sellable if `Damaged` or not in stock.
- `tax_rate(&self) -> f64` — Electronics: 0.15, Clothing: 0.08, Food (perishable): 0.0, Food (non-perishable): 0.05.
- `final_price(&self) -> Option<f64>` — returns `None` if not sellable. Otherwise returns `effective_price * (1.0 + tax_rate)`. This method composes three other methods and wraps the result in `Option`.
- `describe(&self)` — prints all details. Use `match` on both `category` and `condition`.

In `main`, create an array of 5 items with varied categories and conditions. For each, call `describe()` and print the final price (handling the `None` case). Then find and print the cheapest sellable item.

*This is the heaviest exercise in the set. You're matching on two different enums within the same struct's methods, threading `Option` through the pricing logic, and combining struct methods that call other struct methods.*

---

### Challenge 5.3 — Calculator with Memory

Define:
```rust
enum Op {
    Add(f64),
    Sub(f64),
    Mul(f64),
    Div(f64),
    Store,
    Recall,
    Clear,
}

struct Calculator {
    display: f64,
    memory: Option<f64>,
}
```

Implement methods on `Calculator`:
- `Calculator::new() -> Calculator` — display starts at `0.0`, memory is `None`.
- `execute(&mut self, op: &Op)`:
  - `Add`, `Sub`, `Mul`: straightforward arithmetic on `display`.
  - `Div`: if divisor is zero, print an error and leave `display` unchanged.
  - `Store`: saves `display` into `memory` as `Some(display)`.
  - `Recall`: use `if let` to check memory. If `Some`, replace `display`. If `None`, print `"Memory empty"`.
  - `Clear`: reset `display` to `0.0` and `memory` to `None`.
- `show(&self)` — prints the display value and memory status. Use `match` on `self.memory` to print the stored value or `"empty"`.

In `main`, process this sequence:
```rust
let program = [
    Op::Add(10.0),
    Op::Mul(3.0),      // display: 30
    Op::Store,          // memory: 30
    Op::Sub(5.0),       // display: 25
    Op::Div(0.0),       // error, display stays 25
    Op::Div(5.0),       // display: 5
    Op::Recall,         // display: 30 (from memory)
    Op::Clear,
    Op::Recall,         // memory empty
];
```

Print the calculator state after each operation.

**Expected output:**
```
Add(10.0)  → display: 10.0, memory: empty
Mul(3.0)   → display: 30.0, memory: empty
Store      → display: 30.0, memory: 30.0
Sub(5.0)   → display: 25.0, memory: 30.0
Div(0.0)   → error: division by zero
             display: 25.0, memory: 30.0
Div(5.0)   → display: 5.0, memory: 30.0
Recall     → display: 30.0, memory: 30.0
Clear      → display: 0.0, memory: empty
Recall     → memory empty
             display: 0.0, memory: empty
```

*This capstone combines: an enum with mixed variant types (data and unit), a struct with an `Option` field, `&mut self` methods that match on the enum, `if let` for memory recall, and processing a sequence of operations. Every piece of Chapter 6 shows up somewhere.*

---

## Notes

- These challenges build on Chapters 1–5. Ownership, references, and struct methods all appear. If something from Chapter 4 or 5 feels shaky, go back — these challenges will expose it.
- The `#[derive(Debug)]` attribute is your friend. Put it on every enum you define.
- When matching on `&Enum`, the inner data comes through as references. If a variant holds `String`, you'll get `&String` in the match arm. This is ownership at work and it's by design.
- Challenge 5.1's tuple-of-enums matching pattern (`match (state, action)`) is powerful and idiomatic. It works because `match` can destructure tuples just like anything else.
- The nested `match` in 3.3 is deliberately ugly. Remember this feeling when you reach Chapter 9 and `?`, or when you learn about `Option::and_then`. The pain is the lesson.
- `if let` vs `match`: use `if let` when you only care about one variant. Use `match` when you need exhaustive handling. Challenge 4.2 and 5.3 each use both — notice where each is appropriate.
