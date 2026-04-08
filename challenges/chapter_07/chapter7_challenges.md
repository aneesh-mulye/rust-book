# Chapter 7 Programming Challenges — The Rust Book

These challenges exercise packages, crates, modules, paths, visibility (`pub`/private), `use`, `pub use`, `super`, the `as` keyword, nested imports, and file-based module separation. Each tier raises the organizational complexity and demands more judgment about *what goes where and why*.

Every challenge is a Cargo project you build from scratch with `cargo new`. All challenges exercise material from Chapters 1–7 — you will write real logic using enums, structs, methods, `Option`, `match`, `if let`, ownership, and references, organized into modules.

---

## Tier 1: Inline Modules, Privacy, and Paths

These use a single file with `mod { }` blocks. The goal is to internalize the privacy rules and path syntax before introducing the complexity of multiple files.

### Challenge 1.1 — Traffic Light Controller

Create a binary crate `traffic_light`.

In `main.rs`, define the following **inline modules** and types:

**Module `signal`** (public):
- A public enum `Light` with variants `Red`, `Yellow`, `Green`.
- A public function `duration(light: &Light) -> u32` that returns the duration in seconds (Red: 30, Yellow: 5, Green: 25).
- A *private* function `is_stop(light: &Light) -> bool` — true for Red and Yellow.
- A public function `can_proceed(light: &Light) -> bool` that calls `is_stop` internally and returns its negation. This exposes the decision without leaking the private helper.

**Module `controller`** (public):
- A public function `next(light: &signal::Light) -> signal::Light` that returns the next light in the cycle: Green → Yellow → Red → Green.
- A public function `run_cycle(start: &signal::Light)` that starts from the given light and prints each state transition through one complete cycle (3 transitions), printing the light and its duration each step.

In `main`, create a `Green` light and run a full cycle.

**What this exercises:** Cross-module paths (the `controller` module references types from the `signal` module), private helper functions, `pub` on functions and modules, `match` on enums, methods vs free functions.

**Expected output:**
```
Green (25s) → Yellow
Yellow (5s) → Red
Red (30s) → Green
```

---

### Challenge 1.2 — Bank Account System

Create a binary crate `bank`.

Define these inline modules in `main.rs`:

**Module `account`** (public):
- A public struct `Account` with fields: `owner: String` (public), `id: u32` (public), `balance: f64` (**private**).
- Because `balance` is private, external code cannot construct an `Account` directly. Implement:
  - `Account::new(owner: &str, id: u32, initial_deposit: f64) -> Option<Account>` — returns `None` if `initial_deposit` is negative.
  - `balance(&self) -> f64`
  - `deposit(&mut self, amount: f64) -> bool` — returns `false` and does nothing if amount is negative.
  - `withdraw(&mut self, amount: f64) -> bool` — returns `false` if amount is negative or exceeds balance.

**Module `transaction`** (public):
- A public enum `Transaction` with variants:
  - `Deposit(f64)`
  - `Withdrawal(f64)`
  - `Transfer { amount: f64, to_id: u32 }`
- A public function `describe(tx: &Transaction) -> String` that returns a human-readable description.
- A public function `apply(account: &mut account::Account, tx: &Transaction) -> bool` that applies a transaction to an account. For `Transfer`, only process the withdrawal side (we don't have a way to look up the target account — note this limitation and move on). Use `match` to destructure each variant.

In `main`:
1. Create two accounts.
2. Apply a sequence of 4–5 transactions to the first account, printing the description and success/failure of each.
3. Print the final balance.

**What this exercises:** `pub` struct with private fields (the core Ch. 7 pattern for encapsulation), `Option` return from constructor, `&mut` across module boundaries, `match` on enum with struct variant, the constructor pattern forced by private fields.

---

## Tier 2: Multi-File Modules

These projects split code across multiple files. You'll practice the `mod name;` declaration, `use` with `crate::` paths, `as` aliases, and `super`.

### Challenge 2.1 — Geometry Library

Create a library crate: `cargo new geometry --lib`.

Implement the following file structure:

```
geometry/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── point.rs
    ├── shapes.rs
    └── shapes/
        └── analysis.rs
```

**`src/point.rs`**: A public struct `Point` with public fields `x: f64`, `y: f64`. Implement methods:
- `Point::new(x: f64, y: f64) -> Point`
- `distance_to(&self, other: &Point) -> f64`

**`src/shapes.rs`**: Declare the submodule `analysis`. Define a public enum:
```rust
pub enum Shape {
    Circle { center: Point, radius: f64 },
    Rectangle { origin: Point, width: f64, height: f64 },
    Triangle(Point, Point, Point),
}
```
This module needs to `use` the `Point` type from its sibling module. Think about whether to use `crate::point::Point` or `super::point::Point`. Both work from here — pick one and know why.

Implement a method `area(&self) -> f64` on `Shape`.

**`src/shapes/analysis.rs`**: A submodule of `shapes`. Define:
- `pub fn is_large(shape: &super::Shape) -> bool` — true if area > 100.0. Use `super` to reach the parent module's `Shape` type.
- `pub fn largest_area(shapes: &[super::Shape]) -> Option<f64>` — returns the largest area, or `None` if the slice is empty.

**`src/lib.rs`**: Declare the `point` and `shapes` modules. Make them public.

Now add a binary that uses this library. Create `src/main.rs`:
- Use `geometry::point::Point` and `geometry::shapes::Shape` to create several shapes.
- Call `geometry::shapes::analysis::is_large` and `largest_area`.
- Print results.

**What this exercises:** File-based modules, submodules in subdirectories, `super` for parent access, `use crate::` vs `use super::`, library crate with a binary in the same package, the `mod name;` declaration pattern.

---

### Challenge 2.2 — Extract and Reorganize

Start with this single-file program (type it into `src/main.rs` of a new crate `converter`):

```rust
const INCH_PER_CM: f64 = 0.393701;
const CM_PER_INCH: f64 = 2.54;
const KG_PER_LB: f64 = 0.453592;
const LB_PER_KG: f64 = 2.20462;

enum LengthUnit { Inches, Centimeters }
enum WeightUnit { Pounds, Kilograms }

struct Length { value: f64, unit: LengthUnit }
struct Weight { value: f64, unit: WeightUnit }

impl Length {
    fn convert(&self) -> Length {
        match self.unit {
            LengthUnit::Inches => Length {
                value: self.value * CM_PER_INCH,
                unit: LengthUnit::Centimeters,
            },
            LengthUnit::Centimeters => Length {
                value: self.value * INCH_PER_CM,
                unit: LengthUnit::Inches,
            },
        }
    }
    fn describe(&self) -> String {
        let label = match self.unit {
            LengthUnit::Inches => "in",
            LengthUnit::Centimeters => "cm",
        };
        format!("{:.2} {}", self.value, label)
    }
}

impl Weight {
    fn convert(&self) -> Weight {
        match self.unit {
            WeightUnit::Pounds => Weight {
                value: self.value * KG_PER_LB,
                unit: WeightUnit::Kilograms,
            },
            WeightUnit::Kilograms => Weight {
                value: self.value * LB_PER_KG,
                unit: WeightUnit::Pounds,
            },
        }
    }
    fn describe(&self) -> String {
        let label = match self.unit {
            WeightUnit::Pounds => "lb",
            WeightUnit::Kilograms => "kg",
        };
        format!("{:.2} {}", self.value, label)
    }
}

fn main() {
    let l = Length { value: 12.0, unit: LengthUnit::Inches };
    println!("{} = {}", l.describe(), l.convert().describe());
    let w = Weight { value: 150.0, unit: WeightUnit::Pounds };
    println!("{} = {}", w.describe(), w.convert().describe());
}
```

Your task: **refactor** this into a multi-file module structure. Requirements:

1. Create a module `length` in its own file containing `LengthUnit`, `Length`, the constants, and all methods.
2. Create a module `weight` in its own file, same idea.
3. All types and methods used in `main` must be `pub`. Constants should be **private** to their respective modules.
4. In `main.rs`, bring the types into scope with `use` statements. Use **nested imports** where appropriate (e.g., `use crate::length::{Length, LengthUnit};`).
5. The program must produce identical output after refactoring.

Then extend it: add a `volume` module with `VolumeUnit` (Liters, Gallons), `Volume` struct, and conversion logic. Import and test it in `main`.

**What this exercises:** The practical skill of extracting modules from a flat file — something you'll do constantly in real codebases. Private constants, nested `use`, extending a modular structure.

---

## Tier 3: Library + Binary, `pub use`, API Design

These projects have both `src/lib.rs` and `src/main.rs`. The key new skill is designing a public API surface with `pub use`.

### Challenge 3.1 — Color Toolkit

Create a package `colorkit` with both a library and binary crate.

**Internal library structure** (in `src/lib.rs` and submodules):

Module `rgb` (file: `src/rgb.rs`):
- `pub struct Rgb { pub r: u8, pub g: u8, pub b: u8 }`
- Method `to_hex(&self) -> String` — returns e.g. `"#FF00AA"`. You'll need `format!("{:02X}", value)`.
- Method `brightness(&self) -> u8` — average of r, g, b (watch for overflow — use `u16` intermediates).
- Method `mix(&self, other: &Rgb) -> Rgb` — averages each channel.
- Associated function `Rgb::from_hex(hex: &str) -> Option<Rgb>` — parses a string like `"FF00AA"` (no `#` prefix) into an Rgb. Return `None` on bad input. You'll need `u8::from_str_radix(slice, 16)` — this returns a `Result`, use `.ok()` to convert it to `Option`, then handle with `match` or `if let`. This is a small piece of "look it up" — it's in std and straightforward.

Module `palette` (file: `src/palette.rs`):
- `pub struct Palette` with fields `name: String` (public) and `colors: [Option<Rgb>; 8]` (**private**).
- `Palette::new(name: &str) -> Palette` — all slots `None`.
- `add(&mut self, color: Rgb) -> bool` — adds to the first empty slot. Returns false if full.
- `get(&self, index: usize) -> Option<&Rgb>` — returns the color at index, or `None` if the slot is empty or index is out of bounds. This returns a reference into the struct's own data.
- `count(&self) -> usize` — number of filled slots.
- `describe(&self)` — prints palette name and all filled colors as hex.

Module `named` (file: `src/named.rs`):
- `pub fn red() -> Rgb`
- `pub fn green() -> Rgb`
- `pub fn blue() -> Rgb`
- `pub fn white() -> Rgb`
- `pub fn black() -> Rgb`
- These are convenience constructors.

**`src/lib.rs`**: Declare all three modules. Then use **`pub use`** to re-export a clean API:

```rust
pub use rgb::Rgb;
pub use palette::Palette;
pub use named::{red, green, blue, white, black};
```

This means external users write `colorkit::Rgb`, not `colorkit::rgb::Rgb`. The internal module structure is hidden from the public API.

**`src/main.rs`** (binary crate):
- Use `colorkit::Rgb`, `colorkit::Palette`, `colorkit::red`, etc. — confirming the re-exports work.
- Create a palette, add some named colors and a custom color, describe it.
- Parse a hex string, mix two colors, print results.

**What this exercises:** The `pub use` re-exporting pattern (the single most important API design tool in the chapter), library + binary in one package, `Option` in struct fields (array of Options), `u8::from_str_radix` (a small stdlib lookup), methods that return `Option<&T>` (borrowing from struct internals).

---

### Challenge 3.2 — Dice Toolkit with Multiple Binaries

Create a package `dice` with a library crate and **two** binary crates.

**Library** (`src/lib.rs` + modules):

Module `types` (`src/types.rs`):
- `pub enum Die { D4, D6, D8, D10, D12, D20 }`
- Method `sides(&self) -> u32`
- Method `label(&self) -> &str` — returns `"d4"`, `"d6"`, etc.
- `pub struct Roll { pub die: Die, pub value: u32 }` — represents a single roll result.
- `Roll::new(die: Die, value: u32) -> Option<Roll>` — returns `None` if value is 0 or exceeds the die's sides.

Module `scoring` (`src/scoring.rs`):
- `pub fn total(rolls: &[Roll]) -> u32`
- `pub fn highest(rolls: &[Roll]) -> Option<&Roll>` — returns the roll with the highest value.
- `pub fn count_above(rolls: &[Roll], threshold: u32) -> u32`
- `pub fn all_same_die(rolls: &[Roll]) -> bool` — true if every roll used the same die type. You'll need to compare die variants — add `#[derive(PartialEq)]` to `Die`.

Re-export from `lib.rs`:
```rust
pub use types::{Die, Roll};
pub use scoring;
```

Note the asymmetry: `Die` and `Roll` are re-exported as top-level names, but `scoring` is re-exported as a module — users write `dice::scoring::total()`. This is a deliberate API design choice: the types are so fundamental they belong at the top level, but the scoring functions are a coherent group that benefits from staying namespaced.

**Binary 1** (`src/bin/roller.rs`):
- Creates an array of `Roll` values (hardcoded — no randomness), prints each, and prints scoring summaries.

**Binary 2** (`src/bin/stats.rs`):
- Takes a different hardcoded set of rolls and prints analysis: total, highest roll, how many exceeded a threshold, whether all dice were the same type.

Run them with `cargo run --bin roller` and `cargo run --bin stats`.

**What this exercises:** Multiple binary crates in `src/bin/`, selective re-exporting (some items flattened, some kept namespaced), `#[derive(PartialEq)]` (a small but necessary addition — it lets you use `==` on enum variants), `Option<&Roll>` returning a reference into a slice.

---

## Tier 4: Real Projects with Organizational Judgment

In these challenges, the module structure is **not given to you**. You must design it yourself. The specification tells you *what* to build; you decide *how* to organize it.

### Challenge 4.1 — Text Encoding Toolkit

Create a package `cipherkit` with a library and a binary.

**Implement the following ciphers:**

1. **Caesar cipher**: Shifts each letter by a fixed amount (wrapping). Non-letter characters pass through unchanged. Works on both uppercase and lowercase, preserving case.

2. **ROT13**: Caesar cipher with shift 13. Implement it as a *public convenience function* that delegates to your internal Caesar implementation. The Caesar shift parameter should not be exposed in the public API — the library is opinionated and only exposes ROT13 as its Caesar-family offering.

3. **Atbash cipher**: Reverses the alphabet (A↔Z, B↔Y, ...). Preserves case and passes non-letters through.

**Requirements:**

- The library must expose a clean public API. A user should be able to write:
  ```rust
  use cipherkit::{rot13, atbash};
  let encoded = rot13::encode("Hello");
  let decoded = rot13::decode("Uryyb");
  ```
- Each cipher must have both `encode` and `decode` functions (for ROT13, encode and decode are the same operation; for Atbash, likewise — but expose both names for API clarity).
- Internally, have a shared helper module for alphabet manipulation (shifting a character, reversing a character). This module must be **private**.
- The binary should encode and decode sample text with each cipher, printing results and verifying that `decode(encode(text)) == text`.

**Design constraints:**
1. At least 3 modules (not counting `lib.rs`/`main.rs`).
2. At least one module must be private (not `pub mod`).
3. Use `pub use` in `lib.rs` to create the public API.
4. No public function should expose the Caesar shift parameter.
5. All cipher functions take `&str` and return `String`.

**Hints for the character manipulation:**
- `b'a'..=b'z'` matches lowercase ASCII bytes.
- `(byte - b'a' + shift) % 26 + b'a'` performs a Caesar shift on a lowercase byte.
- `b'z' - (byte - b'a')` performs Atbash on a lowercase byte.
- `char::from(byte)` converts a `u8` back to `char`.
- You can iterate over `text.bytes()` and build a `String` with `.push(char)`.

**What this exercises:** Module design judgment (you decide the structure), private helper modules, `pub use`, `String` building from byte manipulation, `match` on byte ranges, the "opinionated API" pattern where internals are more general than what's exposed, library + binary.

---

### Challenge 4.2 — Bowling Scorer

Create a package `bowling` with a library and a binary.

Implement a ten-pin bowling scorer.

**Domain rules** (simplified but real):
- A game has 10 frames.
- Each frame has up to 2 rolls (except the 10th, which can have 3).
- A **strike** (all 10 pins on first roll) scores 10 + the next 2 rolls.
- A **spare** (all 10 pins across both rolls) scores 10 + the next 1 roll.
- An **open frame** scores the sum of its two rolls.
- Maximum possible score: 300 (12 strikes).

**What to implement:**

Types:
- An enum `FrameResult` with variants: `Strike`, `Spare(u8, u8)` (first roll, second roll — must sum to 10), `Open(u8, u8)` (must sum to less than 10).
- A struct `Frame` that holds a `FrameResult` and an `Option<u8>` for the bonus roll in the 10th frame.
- A struct `Game` that holds `[Option<Frame>; 10]` — frames are `None` until played.

Logic:
- A method to compute the score of a single frame, given access to subsequent rolls for strike/spare bonuses. This is the hard part — you need to look ahead into future frames to get the bonus rolls, which requires careful indexing.
- A method to compute the total game score.
- Validation: pin counts 0–10, spare rolls sum to 10, open frame rolls sum to less than 10. Return `bool` or `Option` to signal invalid inputs.

Display:
- A method to print the scorecard showing each frame's rolls and running score.

**Organizational requirements:**
- Separate the *types* (Frame, FrameResult, Game) from the *scoring logic* from the *display/formatting* into different modules.
- The scoring module should be the only place that understands the bonus rules.
- The display module should only need read access (`&self` / `&Game`).
- Use `pub use` in `lib.rs` to present a clean API.

**Binary:**
Score at least two complete games:
1. A perfect game (all strikes) — should score 300.
2. A realistic mixed game with strikes, spares, and open frames.

Print the scorecard for each.

**What this exercises:** Non-trivial domain modeling with enums and structs, `Option` arrays as a stand-in for growable collections, look-ahead indexing (scoring requires peeking at future frames), module separation by concern, privacy for scoring internals.

---

## Tier 5: Capstone

### Challenge 5.1 — `measure`: A Unit Conversion Library

Build a package called `measure` — a genuinely useful unit conversion library with a CLI.

**Scope — four families of units:**

| Family      | Units                                   |
|-------------|-----------------------------------------|
| Length      | Meters, Kilometers, Miles, Feet, Inches |
| Weight      | Kilograms, Grams, Pounds, Ounces        |
| Temperature | Celsius, Fahrenheit, Kelvin             |
| Volume      | Liters, Milliliters, Gallons, Cups      |

**Library requirements:**

1. Each unit family gets its own module.

2. Each family module defines:
   - A public enum for the unit variants (e.g., `LengthUnit::Meters`).
   - A public struct (e.g., `Length`) with a value and a unit.
   - A `convert(&self, to: &XxxUnit) -> Self` method.
   - A `describe(&self) -> String` method.

3. Conversion factors are **private constants** within each module. They must not appear in the public API.

4. Temperature conversion is special — it's not a simple multiplication factor. Your module design must accommodate this without forcing an ugly special case into a shared abstraction. The other three families can use a "convert to base unit, then convert to target" strategy with private factor functions. Temperature needs private `to_celsius` / `from_celsius` functions instead.

5. Each module should have a private helper that returns the factor to convert a given unit to the family's base unit (meters for length, grams for weight, liters for volume). Conversion between any two units goes: source → base → target.

6. `lib.rs` re-exports a curated public API. Support **both** of these usage patterns:
   ```rust
   // Flat imports for common types
   use measure::{Length, LengthUnit, Weight, WeightUnit};

   // Namespaced imports for those who prefer it
   use measure::length::{Length, LengthUnit};
   ```
   This means `pub use` for key types AND `pub mod` for the family modules.

7. Add a `pub fn unit_names() -> &'static [&'static str]` to each family module that lists its units. Add a public function or module in `lib.rs` that lists all supported families.

**Binary** (`src/main.rs`):

A demonstration program that:
1. Converts between several pairs of units within each family, printing results.
2. Demonstrates round-trip consistency: convert A→B→A and verify closeness (floating-point tolerance).
3. Prints all supported unit families and their unit names.

**Stretch (optional):** Add a second binary in `src/bin/convert.rs` that hardcodes a few specific conversion requests and prints them — exercising the `src/bin/` pattern.

**Design decisions you must make:**
- Do all four families share a parallel module structure, or is temperature shaped differently internally? Either choice is defensible — pick one and be consistent.
- Does each family define its own struct (e.g., `Length`, `Weight`, `Temperature`, `Volume`) or share one? Without generics (Ch. 10), you need separate structs — but name them consistently.
- How deep is the module tree? One level (`src/length.rs`) or two (`src/length/mod.rs` + private helpers)?
- What exactly gets `pub use`'d at the top level vs. left in submodules?

**What this exercises:** Every concept from Ch. 7 (packages, crates, modules, paths, `pub`, `pub use`, `super`, `use` / `as`, multi-file, multiple binaries) plus every concept from Ch. 1–6 (enums, structs with private fields, methods, associated functions, `Option`, `match`, `if let`, ownership, references, `String` building). The organizational decisions are the kind a mid-level engineer makes when setting up a real library crate.

---

## Notes

- **Build every challenge with `cargo`.** Use `cargo new name` for binaries, `cargo new name --lib` for libraries. If you need both, create with `--lib` and add `src/main.rs`.
- **Run `cargo check` constantly.** The compiler errors for privacy violations and unresolved paths are extremely informative — they usually tell you exactly which `pub` is missing or which `use` to add.
- **File convention:** `mod foo;` in `src/lib.rs` looks for `src/foo.rs` or `src/foo/mod.rs`. `mod bar;` inside `src/foo.rs` looks for `src/foo/bar.rs` or `src/foo/bar/mod.rs`. Prefer the flat file style (`src/foo.rs`) unless a module has submodules that warrant a directory.
- **`pub use` is the most important new tool.** It's what separates "I know module syntax" from "I can design a library API." Challenges 3.1, 4.1, and 5.1 all demand it. When unsure, ask: *"What would a user of this library want to type?"*
- **Privacy is a design tool, not a checkbox.** In 4.1 and 4.2, ask yourself for every item: if I make this public, can a user violate an invariant? If conversion constants leak, a user might misuse them. If someone can directly construct `FrameResult::Spare(3, 5)` (which sums to 8, not 10), that's a design flaw. Private defaults protect you.
- **`super` vs `crate::`:** Use `super` when a child module references its immediate parent. Use `crate::` for absolute paths that cross the module tree. When in doubt, prefer `crate::` — it reads more clearly in isolation.
- **Tier 4–5 don't specify the file tree.** That's deliberate. Designing the module structure *is* the exercise. If your first attempt feels wrong, refactor — that's how real codebases evolve.
