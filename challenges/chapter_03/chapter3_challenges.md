# Chapter 3 Programming Challenges — The Rust Book

These challenges are staged so that each tier introduces new concepts and combines them with what came before. Work through them in order. Every challenge should compile and run.

---

## Tier 1: Variables, Mutability, and Basic Types

### Challenge 1.1 — Shadow Arithmetic

Declare an immutable variable `x` with value `5`. Shadow it by multiplying by `3`. Shadow it again inside a new scope by adding `10`. Print the value inside the scope, then print the value outside the scope.

**Expected output:**
```
Inner x: 25
Outer x: 15
```

---

### Challenge 1.2 — Type-Shifting Shadow

Declare a variable `data` with the string value `"1024"`. Shadow `data` by parsing it into a `u32`. Shadow it again by converting it to an `f64` and dividing by `3.0`. Print the final value.

*Hint: Use `.parse::<u32>().unwrap()` and `as f64`.*

**Expected output:**
```
data = 341.3333333333333
```

---

### Challenge 1.3 — Mutable Counter with Overflow Awareness

Declare a mutable variable `counter` of type `u8` with value `250`. In a loop, increment it by `1` five times, printing the value each time. Then answer: what happens if you try to increment it past `255` in debug mode vs release mode?

Write the code so it **stops at 255** using `checked_add` instead of `+`.

**Expected output:**
```
counter = 251
counter = 252
counter = 253
counter = 254
counter = 255
Overflow would occur — stopping.
```

---

## Tier 2: Functions, Expressions, and Return Values

### Challenge 2.1 — Expression Blocks as Values

Write a function `classify_age(age: u32) -> &'static str` that uses an expression block (not `if`/`else`) assigned to a variable to return one of: `"child"` (0–12), `"teen"` (13–19), `"adult"` (20–64), `"senior"` (65+). Call it from `main` with several values and print results.

*Hint: An `if`/`else` chain is itself an expression and can be the right-hand side of a `let` binding — that's the "expression block" pattern.*

**Expected output (example):**
```
Age 5: child
Age 15: teen
Age 30: adult
Age 70: senior
```

---

### Challenge 2.2 — Statements vs Expressions

Write a function `last_digit_squared(n: i32) -> i32` that:
1. Extracts the last digit of `n` (use `% 10` and `.abs()`).
2. Returns the square of that digit.

The function body must be a single expression (no `let`, no semicolon on the last line).

Test with: `1984`, `-37`, `100`.

**Expected output:**
```
last_digit_squared(1984) = 16
last_digit_squared(-37) = 49
last_digit_squared(100) = 0
```

---

### Challenge 2.3 — Multi-Return with Tuples

Write a function `divide(dividend: i32, divisor: i32) -> (i32, i32)` that returns both the quotient and the remainder. In `main`, destructure the tuple and print both values. Handle the case where `divisor` is `0` by returning `(0, 0)` (we'll learn better error handling later).

**Expected output:**
```
17 / 5 = 3 remainder 2
10 / 3 = 3 remainder 1
7 / 0 = 0 remainder 0 (division by zero)
```

---

## Tier 3: Control Flow — `if`, `loop`, `while`, `for`

### Challenge 3.1 — FizzBuzz Classic (with `for` and `if`)

Write FizzBuzz for 1 through 30. Use a `for` range loop and `if`/`else if`/`else`. No functions besides `main` required.

---

### Challenge 3.2 — Collatz Conjecture with `while`

Write a function `collatz_steps(mut n: u64) -> u32` that counts how many steps it takes for `n` to reach `1` under the Collatz rules:
- If even: `n = n / 2`
- If odd: `n = 3 * n + 1`

Use a `while` loop. Print the step count for starting values `6`, `27`, and `1`.

**Expected output:**
```
collatz_steps(6) = 8
collatz_steps(27) = 111
collatz_steps(1) = 0
```

---

### Challenge 3.3 — Loop with Break-Value

Use `loop` and `break` *with a value* to find the smallest positive integer whose cube exceeds `10_000`. Store the result of `loop` in a variable and print it.

**Expected output:**
```
Smallest n where n³ > 10000: 22
```

---

## Tier 4: Arrays, Iteration, and Combining Concepts

### Challenge 4.1 — Array Statistics

Given the array `[23, 82, 6, 51, 44, 17, 93, 38, 65, 10]`, write functions:

- `sum(arr: &[i32]) -> i32`
- `min(arr: &[i32]) -> i32`
- `max(arr: &[i32]) -> i32`

Use `for` loops to iterate over the slice. Print the sum, min, max, and compute the mean (as `f64`) in `main`.

**Expected output:**
```
Sum: 429
Min: 6
Max: 93
Mean: 42.9
```

*Note: You haven't formally learned slices/references yet, but `&[i32]` is how you pass an array to a function. Accept it as a recipe for now — Chapter 4 will explain why.*

---

### Challenge 4.2 — Nested Loop Multiplication Table

Print a 5×5 multiplication table using nested `for` loops. Format each cell to be 4 characters wide using `print!("{:4}", value)`. Print a newline after each row.

**Expected output:**
```
   1   2   3   4   5
   2   4   6   8  10
   3   6   9  12  15
   4   8  12  16  20
   5  10  15  20  25
```

---

### Challenge 4.3 — Temperature Conversion Table

Write two functions:
- `f_to_c(f: f64) -> f64` — Fahrenheit to Celsius
- `c_to_f(c: f64) -> f64` — Celsius to Fahrenheit

Use a `for` loop over an array of landmark temperatures `[0.0, 32.0, 100.0, 212.0]` (in Fahrenheit). For each, print the Fahrenheit value and its Celsius equivalent, rounded to one decimal. Then do the reverse for `[-40.0, 0.0, 37.0, 100.0]` (Celsius).

**Expected output:**
```
Fahrenheit -> Celsius:
  0.0°F = -17.8°C
  32.0°F = 0.0°C
  100.0°F = 37.8°C
  212.0°F = 100.0°C

Celsius -> Fahrenheit:
  -40.0°C = -40.0°F
  0.0°C = 32.0°F
  37.0°C = 98.6°F
  100.0°C = 212.0°F
```

---

## Tier 5: Integration Challenges

These combine everything from the chapter. They are deliberately harder.

### Challenge 5.1 — Prime Sieve

Write a program that finds all prime numbers up to `100` using a sieve approach:

1. Create a mutable array of 101 `bool` values, all initialized to `true`.
2. Set indices `0` and `1` to `false`.
3. For each number `i` from `2..`, if `sieve[i]` is `true`, mark all multiples of `i` (starting from `i * i`) as `false`.
4. Collect and print all indices where the value is `true`.

Use `while` or `for` loops. Use a function `print_primes(sieve: &[bool])` to handle the output.

---

### Challenge 5.2 — Fibonacci with Tuples

Write a function `fibonacci(n: u32) -> u64` that returns the `n`th Fibonacci number (0-indexed: `fib(0) = 0`, `fib(1) = 1`). Use a `for` loop and tuple destructuring to track the state `(a, b)` — no array needed.

Print the first 20 Fibonacci numbers.

**Expected output:**
```
fib(0) = 0
fib(1) = 1
fib(2) = 1
...
fib(19) = 4181
```

---

### Challenge 5.3 — Number Guessing Engine

Write a **deterministic** number guessing game engine (no randomness needed — hardcode the secret):

1. A `const SECRET: i32 = 42;`
2. An array of guesses: `[25, 50, 37, 42, 99]`
3. Loop through the guesses. For each, print whether it's too low, too high, or correct.
4. If correct, use `break` to exit the loop and print how many guesses it took.
5. If the array is exhausted without a correct guess, print a failure message.

This exercises: `const`, arrays, `for`, `if`/`else`, `break`, mutable counter, functions.

**Expected output:**
```
Guess 1: 25 — Too low!
Guess 2: 50 — Too high!
Guess 3: 37 — Too low!
Guess 4: 42 — Correct! Found it in 4 guesses.
```

---

### Challenge 5.4 — The Gauntlet

Write a single program that:

1. Declares a `const MAX: u32 = 50;`
2. Has a function `is_prime(n: u32) -> bool` using a loop.
3. Has a function `digit_sum(mut n: u32) -> u32` using a `while` loop.
4. In `main`, iterates from `2` to `MAX`. For each number that is prime AND whose digit sum is also prime, prints it.
5. Prints the total count found.

**Expected output:**
```
2 (digit sum: 2)
3 (digit sum: 3)
5 (digit sum: 5)
7 (digit sum: 7)
11 (digit sum: 2)
23 (digit sum: 5)
29 (digit sum: 11)
41 (digit sum: 5)
43 (digit sum: 7)
47 (digit sum: 11)
Total: 10
```

---

## Notes

- Every challenge should be its own `fn main()` program (or you can combine multiple into one file with helper functions).
- Resist using anything from outside Chapter 3. No `String`, `Vec`, `match`, `enum`, `struct`, `Result`, or `Option` (except the `.unwrap()` in Challenge 1.2 — treat it as a black box for now).
- If you get stuck, re-read the relevant section of Chapter 3 before looking up the answer.
- Once you've completed a tier, try modifying the challenges: change bounds, add edge cases, see what breaks.
