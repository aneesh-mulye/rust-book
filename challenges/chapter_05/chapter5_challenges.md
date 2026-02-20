# Chapter 5 Programming Challenges — The Rust Book

These challenges exercise struct definition, instantiation, field init shorthand, struct update syntax, tuple structs, methods, and associated functions. Each tier layers new concepts and increases the number of moving parts you need to hold in your head simultaneously.

All challenges use only material from Chapters 1–5. No `enum`, `match`, `Option`, `Vec`, or traits beyond `#[derive(Debug)]`.

---

## Tier 1: Defining and Instantiating Structs

### Challenge 1.1 — Color Mixer

Define a struct `Color` with fields `r`, `g`, `b`, all `u8`.

In `main`:
1. Create a `Color` for pure red `(255, 0, 0)`.
2. Create one for white `(255, 255, 255)`.
3. Print each using `{:?}` (you'll need `#[derive(Debug)]`).
4. Print whether the red color has any blue component.

**Expected output:**
```
Red: Color { r: 255, g: 0, b: 0 }
White: Color { r: 255, g: 255, b: 255 }
Red has blue: false
```

---

### Challenge 1.2 — Field Init Shorthand

Define a struct `Sensor` with fields `id: u32`, `label: String`, `reading: f64`.

Write a function `create_sensor(id: u32, label: String, reading: f64) -> Sensor` that uses **field init shorthand** (since the parameter names match the field names).

Create two sensors and print them with `{:#?}`.

---

### Challenge 1.3 — Struct Update Syntax (and its Trap)

Define a struct `ServerConfig` with fields: `host: String`, `port: u16`, `max_connections: u32`, `verbose: bool`.

In `main`:
1. Create a `default_config` with host `"localhost"`, port `8080`, max connections `100`, verbose `false`.
2. Create a `custom_config` using struct update syntax (`..default_config`) but overriding `port` to `3000` and `verbose` to `true`.
3. Try to print `default_config.host` after creating `custom_config`.

**Predict before running:** Will step 3 compile? Why or why not? Which fields of `default_config` are still usable and which aren't?

---

### Challenge 1.4 — Tuple Structs as Distinct Types

Define two tuple structs:
```rust
struct Celsius(f64);
struct Fahrenheit(f64);
```

Write two functions:
- `to_fahrenheit(c: &Celsius) -> Fahrenheit`
- `to_celsius(f: &Fahrenheit) -> Celsius`

In `main`, create a `Celsius(100.0)`, convert it, and print both values using their `.0` field.

Then try this (without running): would `let temp: Celsius = Fahrenheit(72.0);` compile? Why does this matter?

**Expected output:**
```
100°C = 212°F
212°F = 100°C
```

---

## Tier 2: Methods and `impl` Blocks

### Challenge 2.1 — Basic Methods

Add an `impl` block to your `Color` struct from 1.1 with these methods:

- `is_grayscale(&self) -> bool` — returns `true` if `r == g == b`.
- `brightness(&self) -> u8` — returns the average of `r`, `g`, `b`. Be careful: three `u8` values added together can overflow. Think about what intermediate type you need.

Test with `Color { r: 100, g: 100, b: 100 }` and `Color { r: 200, g: 50, b: 10 }`.

**Expected output:**
```
(100, 100, 100): grayscale=true, brightness=100
(200, 50, 10): grayscale=false, brightness=86
```

---

### Challenge 2.2 — Mutating Methods

Define a struct `Counter` with fields `value: i32`, `min: i32`, `max: i32`.

Add methods:
- `increment(&mut self)` — adds 1 but clamps to `max`.
- `decrement(&mut self)` — subtracts 1 but clamps to `min`.
- `reset(&mut self)` — sets `value` to `min`.
- `is_maxed(&self) -> bool`

In `main`, create a counter with min `0`, max `3`, value `0`. Increment it 5 times (printing each time) to show clamping.

**Expected output:**
```
value: 1
value: 2
value: 3
value: 3
value: 3
Maxed out: true
```

---

### Challenge 2.3 — Associated Functions (Constructors)

Add associated functions to `Counter`:
- `Counter::new(min: i32, max: i32) -> Counter` — creates a counter with `value` set to `min`.
- `Counter::zero_to(max: i32) -> Counter` — shorthand for `min = 0`.

These use `Self` instead of needing to spell out the struct name. Test both constructors.

---

### Challenge 2.4 — Methods Taking Other Instances

Go back to `Color` and add:
- `mix(&self, other: &Color) -> Color` — averages each channel between the two colors.
- `distance(&self, other: &Color) -> f64` — Euclidean distance in RGB space: `sqrt((r1-r2)² + (g1-g2)² + (b1-b2)²)`. You'll need intermediate `f64` casts. Use `f64::sqrt()`.

Test by mixing red `(255, 0, 0)` and blue `(0, 0, 255)`, and computing the distance between them.

**Expected output:**
```
Mixed: Color { r: 127, g: 0, b: 127 }
Distance: 360.6
```

*(The exact distance value depends on rounding — `360.6` is approximate.)*

---

## Tier 3: Multiple Structs, Methods Calling Methods

### Challenge 3.1 — Internal Method Calls

Define a struct `Rect` with fields `x: f64`, `y: f64`, `width: f64`, `height: f64`.

Add methods:
- `area(&self) -> f64`
- `perimeter(&self) -> f64`
- `is_square(&self) -> bool`
- `center(&self) -> (f64, f64)` — returns `(x + width/2, y + height/2)`.
- `contains_point(&self, px: f64, py: f64) -> bool` — true if the point is inside the rectangle.
- `overlaps(&self, other: &Rect) -> bool` — true if the two rectangles overlap. *This is the hard one.* Two rects do NOT overlap if one is entirely to the left, right, above, or below the other. The `overlaps` check is the negation of that.

Test with a couple of rectangles and points.

---

### Challenge 3.2 — Struct Composition

Define two structs:
```rust
struct Coordinate {
    x: f64,
    y: f64,
}

struct City {
    name: String,
    population: u64,
    location: Coordinate,
}
```

Add methods:
- `Coordinate::distance_to(&self, other: &Coordinate) -> f64`
- `City::new(name: &str, population: u64, x: f64, y: f64) -> City`
- `City::is_bigger_than(&self, other: &City) -> bool`
- `City::distance_to(&self, other: &City) -> f64` — delegates to `Coordinate::distance_to`. This is the key design insight: the `City` method reuses the `Coordinate` method rather than reimplementing the math.

Create three cities. Find which pair is closest and which city is largest.

---

### Challenge 3.3 — Methods That Consume `self`

Define a struct `EmailDraft` with fields `to: String`, `subject: String`, `body: String`.

Add methods:
- `EmailDraft::new(to: &str) -> EmailDraft` — creates a draft with empty subject and body.
- `with_subject(self, subject: &str) -> EmailDraft` — takes ownership, returns new draft with subject set.
- `with_body(self, body: &str) -> EmailDraft` — same pattern.
- `send(self)` — takes ownership, prints the email, and consumes the draft so it can't be used again.

Use method chaining in `main`:
```rust
EmailDraft::new("alice@example.com")
    .with_subject("Hello")
    .with_body("How are you?")
    .send();
```

After calling `.send()`, verify (mentally or by testing) that you can no longer use the draft. Why does taking `self` by value enforce this?

---

## Tier 4: Ownership Meets Structs

### Challenge 4.1 — Predict the Error

**Do not run this.** Read it, predict whether it compiles, and identify the exact problem.

```rust
#[derive(Debug)]
struct Tweet {
    author: String,
    content: String,
    likes: u32,
}

fn main() {
    let t1 = Tweet {
        author: String::from("alice"),
        content: String::from("hello world"),
        likes: 0,
    };

    let t2 = Tweet {
        content: String::from("different content"),
        ..t1
    };

    println!("t1 author: {}", t1.author);
    println!("t1 likes: {}", t1.likes);
    println!("t2: {:?}", t2);
}
```

Questions:
1. Which of the two `println!` lines for `t1` will fail?
2. Why does one field survive and the other doesn't?
3. What if `author` were `&str` instead of `String`? (Don't try to make it compile — just reason about what would change and what new problem would arise.)

---

### Challenge 4.2 — Borrowing Struct Fields

Define a struct `Inventory` with fields `name: String` and `items: [i32; 5]` (an array of quantities for 5 products).

Write these functions (not methods — standalone functions):
- `print_name(name: &str)` — prints just the name.
- `total_items(items: &[i32]) -> i32` — sums the array.

In `main`, call both functions by borrowing individual *fields* of a single `Inventory` instance. Then mutate one of the item quantities and print the total again.

The point: you can borrow individual fields of a struct independently. A mutable borrow of `inv.items[2]` doesn't prevent an immutable borrow of `inv.name`.

---

### Challenge 4.3 — Method Signatures and Ownership

Define a struct `Buffer` with a single field `data: String`.

Add these methods — each one uses a different form of `self`:

- `len(&self) -> usize` — returns the length of `data`.
- `append(&mut self, text: &str)` — appends to `data`.
- `into_data(self) -> String` — consumes the `Buffer` and returns the inner `String`.
- `Buffer::from_str(s: &str) -> Buffer` — associated function, creates a new buffer.

In `main`:
1. Create a buffer with `"hello"`.
2. Print its length (`&self` borrow).
3. Append `" world"` (`&mut self` borrow).
4. Print its length again.
5. Extract the inner string (`self` — consumes buffer).
6. Try to use the buffer after step 5 — observe the error, then comment it out.

**Expected output:**
```
Length: 5
Length: 11
Extracted: hello world
```

---

## Tier 5: Integration Challenges

### Challenge 5.1 — Student Grade Tracker

Define:
```rust
struct Student {
    name: String,
    scores: [f64; 5],
}
```

Implement these methods:
- `Student::new(name: &str, scores: [f64; 5]) -> Student`
- `average(&self) -> f64`
- `highest(&self) -> f64`
- `lowest(&self) -> f64`
- `letter_grade(&self) -> char` — based on `average()`: A (90+), B (80+), C (70+), D (60+), F (below 60). This method should **call** `average()` internally.
- `summary(&self)` — prints a formatted summary, calling the other methods. Methods calling methods on `&self`.

Create three students with different score distributions. Print each summary, then find and print which student has the highest average.

**Expected output (example):**
```
--- Alice ---
Scores: [92.0, 88.0, 95.0, 78.0, 90.0]
Average: 88.6
Highest: 95.0
Lowest: 78.0
Grade: B

--- Bob ---
Scores: [55.0, 62.0, 70.0, 45.0, 58.0]
Average: 58.0
Highest: 70.0
Lowest: 45.0
Grade: F

Best student: Alice (88.6)
```

---

### Challenge 5.2 — Particle Simulation

Define:
```rust
struct Vec2 {
    x: f64,
    y: f64,
}

struct Particle {
    position: Vec2,
    velocity: Vec2,
    mass: f64,
}
```

Implement for `Vec2`:
- `Vec2::new(x: f64, y: f64) -> Vec2`
- `add(&self, other: &Vec2) -> Vec2`
- `scale(&self, factor: f64) -> Vec2`
- `magnitude(&self) -> f64`

Implement for `Particle`:
- `Particle::new(x: f64, y: f64, vx: f64, vy: f64, mass: f64) -> Particle`
- `step(&mut self, dt: f64)` — updates position by adding `velocity * dt`. Use `Vec2::scale` and `Vec2::add` inside this method.
- `kinetic_energy(&self) -> f64` — `0.5 * mass * velocity.magnitude()²`
- `distance_to(&self, other: &Particle) -> f64`

In `main`:
1. Create two particles.
2. Run a simulation loop of 5 steps with `dt = 0.1`.
3. Each step: update both particles, print their positions, and print the distance between them.

This challenge tests struct composition, methods delegating to other struct methods, `&self` vs `&mut self` in a realistic context, and managing multiple structs across a loop.

---

### Challenge 5.3 — The Builder Gauntlet

Combine the builder pattern from 3.3 with validation. Define:

```rust
struct Form {
    username: String,
    age: u32,
    accepted_terms: bool,
}

struct FormBuilder {
    username: String,
    age: u32,
    accepted_terms: bool,
    errors: u32,
}
```

Implement for `FormBuilder`:
- `FormBuilder::new() -> FormBuilder` — all fields default/empty.
- `username(self, name: &str) -> FormBuilder` — if `name` is empty (`.is_empty()`), increment `errors` and print a warning. Otherwise set the field.
- `age(self, age: u32) -> FormBuilder` — if age is `0` or `> 150`, increment `errors` and print a warning. Otherwise set the field.
- `accept_terms(self) -> FormBuilder` — sets `accepted_terms` to `true`.
- `build(self) -> Form` — if `errors > 0` or `accepted_terms` is `false`, print what's wrong. Otherwise return the `Form`. *(Since you don't have `Option` or `Result` yet, just print the error and return a Form with empty/default values as a placeholder — note this limitation.)*

Test with both a valid and an invalid builder chain.

The working memory demand here: you're tracking ownership transfer through a chain of method calls, each of which conditionally modifies state. The builder takes `self` by value each time, so the chain is a sequence of moves.

---

## Notes

- Every challenge should compile and run (except the deliberate "predict the error" ones).
- The builder pattern (3.3, 5.3) takes `self` by value and returns `Self`. This is idiomatic Rust — understand *why* it must take ownership rather than `&mut self` for chaining to work.
- Challenge 4.1 is one of the most important here. The struct update syntax (`..other`) silently moves non-Copy fields. This bites real Rust programmers regularly.
- If a challenge feels easy, ask: "could I explain which borrow of `self` each method uses and *why*?" The method signatures are the design decisions.
- Once comfortable, try adding `#[derive(Debug)]` to every struct and use `{:#?}` liberally for inspection.
