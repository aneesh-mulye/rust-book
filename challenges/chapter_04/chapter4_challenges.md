# Chapter 4 Programming Challenges — The Rust Book

These challenges exercise ownership, borrowing, references, and slices. Each tier layers new concepts onto the previous ones. **Every challenge must compile.** If it doesn't, the compiler error *is* the lesson — read it carefully before changing your code.

---

## Tier 1: Ownership and Move Semantics

### Challenge 1.1 — Predict the Error

**Do not run this yet.** Read it and predict exactly which line will fail to compile and why. Then try to compile it and check.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1);
    println!("{}", s2);
}
```

**Task:** Fix it two different ways — once with `.clone()`, once by rearranging the code so you only use each binding when it's still valid.

---

### Challenge 1.2 — Move Into a Function

Write a function `print_and_return(s: String) -> String` that prints the string and returns it back. In `main`:

1. Create a `String` with value `"ownership"`.
2. Pass it to `print_and_return`.
3. Use the returned value to print it again.

Then try removing the return and using the original variable after the call — observe the compiler error.

**Expected output:**
```
Inside function: ownership
Back in main: ownership
```

---

### Challenge 1.3 — Copy vs Move

**Predict before running.** Will this compile? Why or why not?

```rust
fn main() {
    let a = 42;
    let b = a;
    println!("a = {}, b = {}", a, b);

    let s = String::from("hello");
    let t = s;
    println!("s = {}, t = {}", s, t);
}
```

**Task:** Explain which types implement `Copy` and which don't, then fix the code.

---

### Challenge 1.4 — Ownership in a Loop

Create a `String` before a `for` loop that runs 3 times. On each iteration, pass the string to a function that prints it and returns it. Capture the return value so you can pass it again on the next iteration.

*This is deliberately awkward — Tier 2 shows why references are the better solution.*

**Expected output:**
```
Iteration 1: hello
Iteration 2: hello
Iteration 3: hello
```

---

## Tier 2: References and Borrowing

### Challenge 2.1 — Immutable References

Rewrite Challenge 1.4 using an immutable reference (`&String`) instead of transferring ownership. The function should take `&String`, and you should **not** need to return anything.

Notice how much simpler the code becomes.

---

### Challenge 2.2 — Mutable References

Write a function `append_exclamation(s: &mut String)` that appends `"!"` to the given string. In `main`:

1. Create a mutable `String` with value `"hello"`.
2. Call `append_exclamation` three times.
3. Print the result.

**Expected output:**
```
hello!!!
```

---

### Challenge 2.3 — The Borrow Rules

**Predict before running.** Which of these three blocks will fail to compile? For each failure, state the rule being violated.

```rust
fn main() {
    // Block A
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    // Block B
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;
    println!("{} and {}", r1, r2);

    // Block C
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{} and {}", r1, r2);
}
```

**Task:** Fix blocks B and C so they compile, by ensuring mutable and immutable borrows don't overlap. *Hint: A reference's lifetime ends at its last use, not at the end of the scope (Non-Lexical Lifetimes).*

---

### Challenge 2.4 — Borrowing in Practice

Write a function `longest_word_length(text: &String) -> usize` that iterates over the bytes of the string, tracks the current word length and the maximum word length, and returns the maximum. Use a `for` loop over `text.as_bytes()`. Count a "word" as any sequence of non-space bytes.

Test with: `"the quick brown fox"` and `"rust"`.

**Expected output:**
```
Longest word length in "the quick brown fox": 5
Longest word length in "rust": 4
```

*The original string must still be usable after the function call — that's the whole point of borrowing.*

---

## Tier 3: String Slices

### Challenge 3.1 — First Word

Write a function `first_word(s: &str) -> &str` that returns a slice of everything up to the first space (or the whole string if there's no space).

Test with: `"hello world"`, `"rust"`, `"one two three"`.

**Expected output:**
```
First word of "hello world": hello
First word of "rust": rust
First word of "one two three": one
```

---

### Challenge 3.2 — Slice Indexing

Given a `String`, take various slices and print them:

1. The first 3 characters.
2. The last 4 characters.
3. Everything except the first and last characters.

Test with `"Rustacean"`.

**Expected output:**
```
First 3: Rus
Last 4: cean
Middle: ustacea
```

*Remember: string slices use byte indices, and this only works cleanly with ASCII. Chapter 8 covers UTF-8 properly.*

---

### Challenge 3.3 — Why `&str` Over `&String`?

Write a function `starts_with_r(s: &str) -> bool` that returns `true` if the first byte of the string is `b'r'` or `b'R'`.

In `main`, call it with:
1. A string literal (`"rust"`)
2. A `String` (`String::from("Rust")`)
3. A slice of a `String` (`&some_string[1..]`)

All three calls should work without any changes to the function signature. This demonstrates why `&str` is preferred over `&String` in function parameters.

**Expected output:**
```
"rust" starts with R: true
"Rust" starts with R: true
"ust" starts with R: false
```

---

## Tier 4: Array/Vec Slices and Combined Borrowing

### Challenge 4.1 — Array Slices

Write a function `sum(numbers: &[i32]) -> i32` that sums a slice of integers. In `main`, call it with:

1. An entire array: `&arr`
2. The first 3 elements: `&arr[..3]`
3. The last 2 elements: `&arr[3..]`

Use the array `[10, 20, 30, 40, 50]`.

**Expected output:**
```
Total: 150
First 3: 60
Last 2: 90
```

---

### Challenge 4.2 — Mutable Slice Operations

Write a function `double_all(numbers: &mut [i32])` that doubles every element in a mutable slice. In `main`:

1. Create a mutable array `[1, 2, 3, 4, 5]`.
2. Double only the first 3 elements using a slice.
3. Print the entire array.

**Expected output:**
```
[2, 4, 6, 4, 5]
```

*Hint: Iterate with `for item in numbers.iter_mut()`.*

---

### Challenge 4.3 — Borrowing Discipline

Write two functions:
- `find_max(numbers: &[i32]) -> i32` — returns the maximum value.
- `replace_max(numbers: &mut [i32], replacement: i32)` — finds the maximum and replaces its first occurrence with `replacement`.

In `main`:
1. Create a mutable array `[3, 7, 2, 9, 4]`.
2. Print the max (immutable borrow).
3. Replace the max with `0` (mutable borrow).
4. Print the array.
5. Print the new max.

This works because each borrow ends before the next one begins.

**Expected output:**
```
Max: 9
After replacement: [3, 7, 2, 0, 4]
New max: 7
```

---

## Tier 5: Integration Challenges

### Challenge 5.1 — Word Counter

Write a program with these functions:
- `word_count(text: &str) -> u32` — counts words (separated by spaces).
- `char_count(text: &str) -> u32` — counts total non-space characters.
- `average_word_length(text: &str) -> f64` — returns `char_count / word_count` as a float.

Test with: `"the rust programming language is great"`.

**Expected output:**
```
Text: "the rust programming language is great"
Words: 6
Characters: 32
Average word length: 5.3
```

*The key constraint: `text` is borrowed immutably by all three functions, and the original `String` remains usable throughout.*

---

### Challenge 5.2 — Ownership Ping-Pong

Write a program that demonstrates ownership transfer through a chain of functions:

1. `create_greeting(name: &str) -> String` — creates and returns `"Hello, {name}!"`.
2. `make_loud(s: String) -> String` — takes ownership, returns an uppercase version.
3. `add_punctuation(s: &mut String)` — appends `" Welcome!!!"` via mutable reference.
4. `print_final(s: &String)` — prints via immutable reference.

Chain them in `main` for the name `"Aneesh"`.

**Expected output:**
```
HELLO, ANEESH! Welcome!!!
```

*Pay close attention to which functions take ownership, which borrow mutably, and which borrow immutably. Each choice has a reason.*

---

### Challenge 5.3 — The Slice Gauntlet

Write a function `analyze(data: &[i32]) -> (i32, i32, bool)` that returns `(min, max, is_sorted)` for a given slice. It should work without modifying the input.

Write a second function `normalize(data: &mut [i32])` that subtracts the minimum value from every element (so the smallest element becomes `0`). You'll need to find the min first, then mutate — think about how borrowing affects this.

In `main`:
1. Analyze `[5, 3, 8, 1, 9]`, print results.
2. Normalize it, print the array.
3. Analyze again, print results.

**Expected output:**
```
Analysis: min=1, max=9, sorted=false
After normalize: [4, 2, 7, 0, 8]
Analysis: min=0, max=8, sorted=false
```

---

### Challenge 5.4 — Dangling Reference Detective

Each of the following functions has a problem. **Without running the code**, identify why each won't compile, state which ownership/borrowing rule is violated, and write a corrected version.

```rust
// Problem A
fn make_string() -> &String {
    let s = String::from("hello");
    &s
}

// Problem B
fn append_and_read(s: &mut String) -> &str {
    s.push_str(" world");
    &s
}

// Problem C
fn first_and_mutate(s: &mut String) -> &str {
    let first = &s[..1];
    s.push_str("!");
    first
}
```

For each, explain:
1. What the compiler would say.
2. Why the rule exists (what memory bug it prevents).
3. A working alternative.

---

## Notes

- These challenges use `String`, `&str`, `&T`, `&mut T`, and slices — all core Chapter 4 material.
- You'll need `.to_string()`, `.to_uppercase()`, `String::from()`, `.push_str()`, `.as_bytes()`, and `.len()` — all covered in the book.
- The challenges deliberately avoid `Vec`, `match`, `Option`, `Result`, and anything beyond Chapter 4.
- Challenge 5.4 is meant to be done on paper first. The compiler will confirm your reasoning afterward.
- If a challenge feels trivially easy, ask yourself: "Could I explain *why* this works to someone else?" If not, you haven't fully absorbed it yet.
