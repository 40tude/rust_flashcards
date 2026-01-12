<!--
###############################################################################
##
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - Write a short code to demonstrate how ownership is transferred. What is moving?


Answer   :

```rust
fn main() {
    let my_name = String::from("Philippe");
    let my_age: u8 = 42;

    println!("In main:");
    println!("  my_name (stack) address: {:p}", &my_name);
    println!("  my_name (heap) address:  {:p}", my_name.as_ptr());
    println!("  my_age address:          {:p}", &my_age);

    show_addresses(my_name, my_age);
}

fn show_addresses(name: String, age: u8) {
    println!("\nIn show_addresses:");
    println!("  name (stack) address: {:p}", &name);
    println!("  name (heap) address:  {:p}", name.as_ptr());
    println!("  age address:          {:p}", &age);
}
```
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

```
In main:
  my_name (stack) address: 0x7ffec7cd0218
  my_name (heap) address:  0x5fcc251ced00
  my_age address:          0x7ffec7cd0237

In show_addresses:
  name (stack) address: 0x7ffec7cd0380
  name (heap) address:  0x5fcc251ced00
  age address:          0x7ffec7cd0077
```


---

* The `String` struct itself (which contains a pointer, length, and capacity: 24 bytes on 64-bit) gets copied to the new stack frame.
* However, the heap data (the actual "Philippe" bytes) doesn't move at all.
* We can check the heap address is identical in both functions: proving the actual string data never moved.
* Only ownership (and the stack metadata) transferred.

It's not the data that moves, but the ownership.














<!--
###############################################################################
## Ownership - The Rules
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - What are the three ownership rules in Rust?
Answer   :

The three ownership rules are:

1. Each value in Rust has an **owner**
2. There can only be **one owner** at a time
3. When the owner goes out of scope, the value is **dropped**

```rust
fn main() {
    {
        let s = String::from("hello");  // s is the owner
        println!("{}", s);
    }  // s goes out of scope, memory is freed

    // println!("{}", s);  // Error: s no longer exists
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Read more in <a href="https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules" target="_blank">TRPL - Ownership Rules</a>.



<!--
###############################################################################
## Ownership - Scope
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - What is a scope and how does it relate to ownership?
Answer   :

```rust
fn main() {
    // s is not valid here, it's not yet declared

    {
        let s = "hello";   // s is valid from this point
        println!("{}", s); // s is still valid
    }                      // scope ends, s is no longer valid

    // println!("{}", s);  // Error: s is out of scope
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

A **scope** is the range within a program where an item is valid. When a variable goes out of scope, Rust automatically calls `drop` to free the memory.

Read more in <a href="https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variable-scope" target="_blank">TRPL - Variable Scope</a>.



<!--
###############################################################################
## Ownership - Stack vs Heap
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - What is the difference between stack and heap allocation?
Answer   :

```rust
fn main() {
    // Stack: fixed size, fast, copied by value
    let x = 5;
    let y = x;  // Copy: both x and y are valid
    println!("x={}, y={}", x, y);

    // Heap: dynamic size, slower, ownership transferred
    let s1 = String::from("hello");
    let s2 = s1;  // Move: s1 is no longer valid
    // println!("{}", s1);  // Error: value moved
    println!("s2={}", s2);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

**Stack**: Fixed-size data, fast allocation, values are copied.
**Heap**: Dynamic-size data, requires allocation, ownership is moved.

Read this <a href="https://www.40tude.fr/docs/06_programmation/rust/004_mutability/mutability_us.html#a-first-detour-to-understand-what-happens-in-memory" target="_blank">post </a> on <a href="https://www.40tude.fr/docs/06_programmation/rust/" target="_blank">40tude.fr</a>.

Read more in <a href="https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap" target="_blank">TRPL - The Stack and the Heap</a>.



<!--
###############################################################################
## Ownership - Move Semantics
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - What is a move in Rust?
Answer   :

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is MOVED to s2

    // println!("{}", s1);  // Error: borrow of moved value
    println!("{}", s2);     // OK: s2 now owns the data
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

A **move** transfers ownership from one variable to another. The original variable becomes invalid. This prevents double-free errors and ensures memory safety without garbage collection.

Read more in <a href="https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move" target="_blank">TRPL - Move</a>.



<!--
###############################################################################
## Ownership - Clone
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - How do you create a deep copy of heap data?
Answer   :

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Deep copy of heap data

    println!("s1={}, s2={}", s1, s2);  // Both are valid!
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

The `clone()` method creates a **deep copy** of the data, including heap-allocated memory. Both variables remain valid. This is explicit and can be expensive.

Read more in <a href="https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-clone" target="_blank">TRPL - Clone</a>.



<!--
###############################################################################
## Ownership - Copy Trait
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - What types implement the Copy trait?
Answer   :

```rust
fn main() {
    let x = 5;
    let y = x;  // Copy, not move!
    println!("x={}, y={}", x, y);  // Both valid

    let a = true;
    let b = a;  // Also a copy
    println!("a={}, b={}", a, b);

    let t = (1, 2);
    let u = t;  // Copy (tuple of Copy types)
    println!("t={:?}, u={:?}", t, u);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Types that implement `Copy` are copied instead of moved:
- All integer types (`i32`, `u64`, etc.)
- `bool`
- `char`
- Floating-point types (`f32`, `f64`)
- Tuples containing only `Copy` types

Read more in <a href="https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy" target="_blank">TRPL - Stack-Only Data: Copy</a>.



<!--
###############################################################################
## Ownership - Functions
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - How does ownership work with functions?
Answer   :

```rust
fn takes_ownership(s: String) {
    println!("Got: {}", s);
}  // s is dropped here

fn makes_copy(x: i32) {
    println!("Got: {}", x);
}  // x goes out of scope, nothing special (it's Copy)

fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);  // Error: s was moved

    let x = 5;
    makes_copy(x);
    println!("x is still: {}", x);  // OK: x was copied
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Passing a value to a function moves or copies it, just like assignment. Heap data is moved, stack data implementing `Copy` is copied.

Read more in <a href="https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions" target="_blank">TRPL - Ownership and Functions</a>.



<!--
###############################################################################
## Ownership - Return Values
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - How can a function transfer ownership back to the caller?
Answer   :

```rust
fn create_string() -> String {
    let s = String::from("hello");
    s  // Ownership is transferred to the caller
}

fn take_and_give_back(s: String) -> String {
    s  // Return the same string, transferring ownership back
}

fn main() {
    let s1 = create_string();
    println!("s1: {}", s1);

    let s2 = String::from("world");
    let s3 = take_and_give_back(s2);
    // println!("{}", s2);  // Error: s2 was moved
    println!("s3: {}", s3);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Returning a value from a function transfers ownership to the caller. This pattern can get tedious: that's why we have **references**.

Read more in <a href="https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope" target="_blank">TRPL - Return Values and Scope</a>.



<!--
###############################################################################
## References - Immutable Borrowing
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - What is a reference and how do you create one?
Answer   :

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope but doesn't drop what it refers to

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // Pass a reference

    println!("The length of '{}' is {}", s1, len);  // s1 still valid!
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

A **reference** (`&`) allows you to refer to a value without taking ownership. This is called **borrowing**. The original owner keeps ownership.

Read more in <a href="https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html" target="_blank">TRPL - References and Borrowing</a>.



<!--
###############################################################################
## References - Mutable Borrowing
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - How do you create a mutable reference?
Answer   :

```rust
fn add_world(s: &mut String) {
    s.push_str(", world!");
}

fn main() {
    let mut s = String::from("hello");  // Must be mut
    add_world(&mut s);                   // Pass mutable reference

    println!("{}", s);  // "hello, world!"
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Use `&mut` to create a **mutable reference**. The variable itself must be declared `mut`. Mutable references allow modifying the borrowed value.

Read more in <a href="https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references" target="_blank">TRPL - Mutable References</a>.



<!--
###############################################################################
## References - One Mutable Reference Rule
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - Can you have multiple mutable references to the same data?
Answer   :

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;  // Error: cannot borrow `s` as mutable more than once

    println!("{}", r1);

    // After r1 is no longer used, we can create another mutable reference
    let r2 = &mut s;
    println!("{}", r2);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

**No.** You can have only **one mutable reference** to a value at a time. This prevents data races at compile time.

Read more in <a href="https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references" target="_blank">TRPL - Mutable References</a>.



<!--
###############################################################################
## References - Mixing Mutable and Immutable
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - Can you mix mutable and immutable references?
Answer   :

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;     // OK
    let r2 = &s;     // OK: multiple immutable refs allowed
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // OK: no immutable refs are active
    println!("{}", r3);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

You **cannot** have a mutable reference while immutable references exist. The rule: **either** one mutable reference **or** any number of immutable references, but not both.

Read more in <a href="https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references" target="_blank">TRPL - Mutable References</a>.



<!--
###############################################################################
## References - Non-Lexical Lifetimes (NLL)
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - What are Non-Lexical Lifetimes (NLL)?
Answer   :

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 and r2's lifetimes END here (last use)

    // This works because r1/r2 are no longer "alive"
    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

**NLL** means a reference's lifetime ends at its last use, not at the end of its scope. This makes the borrow checker smarter and more permissive.

Read more in <a href="https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#non-lexical-lifetimes" target="_blank">Rust Blog - Non-Lexical Lifetimes</a>.



<!--
###############################################################################
## References - Dangling References
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - What is a dangling reference and how does Rust prevent it?
Answer   :

```rust
// This would NOT compile:
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // Error: returns reference to dropped value
// }

// Correct: return the owned value instead
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // Ownership is transferred out
}

fn main() {
    let s = no_dangle();
    println!("{}", s);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

A **dangling reference** points to memory that has been freed. Rust prevents this at compile time: you cannot return a reference to a local variable.

Read more in <a href="https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references" target="_blank">TRPL - Dangling References</a>.



<!--
###############################################################################
## Slices - String Slices
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - What is a string slice (`&str`)?
Answer   :

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];   // "hello"
    let world = &s[6..11];  // "world"

    // Shorthand
    let hello2 = &s[..5];   // From start
    let world2 = &s[6..];   // To end
    let whole = &s[..];     // Entire string

    println!("{} {} {} {} {}", hello, world, hello2, world2, whole);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

A **string slice** (`&str`) is a reference to a portion of a `String`. It doesn't own the data. String literals are also `&str` (slices of static memory).

Read more in <a href="https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices" target="_blank">TRPL - String Slices</a>.



<!--
###############################################################################
## Slices - &str vs String
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - What is the difference between `String` and `&str`?
Answer   :

```rust
fn greet(name: &str) {  // Accept both String and &str
    println!("Hello, {}!", name);
}

fn main() {
    // String: owned, heap-allocated, growable
    let owned = String::from("Alice");

    // &str: borrowed slice, can be from String or literal
    let literal: &str = "Bob";
    let slice: &str = &owned[..];

    greet(&owned);   // &String coerces to &str
    greet(literal);  // Already &str
    greet(slice);    // Already &str
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

| `String` | `&str` |
|----------|--------|
| Owned | Borrowed |
| Heap-allocated | Points to existing data |
| Growable | Fixed view |
| Use when you need ownership | Use for function parameters |

Read more in <a href="https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices" target="_blank">TRPL - String Slices</a>.



<!--
###############################################################################
## Slices - Array Slices
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - How do you create a slice of an array or vector?
Answer   :

```rust
fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let vec = vec![10, 20, 30, 40, 50];

    // Slice of array
    let arr_slice = &arr[1..4];  // [2, 3, 4] from index 1 included to 4 excluded, aka [1, 4)
    println!("arr_slice: {:?}, sum: {}", arr_slice, sum(arr_slice));

    // Slice of vector
    let vec_slice = &vec[..3];   // [10, 20, 30], try []..=3]
    println!("vec_slice: {:?}, sum: {}", vec_slice, sum(vec_slice));

    // Entire array/vec as slice
    println!("Full arr sum: {}", sum(&arr));
    println!("Full vec sum: {}", sum(&vec));
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Slices work with any contiguous sequence. `&[T]` is a slice of elements of type `T`. Functions accepting `&[T]` work with both arrays and vectors.

Read more in <a href="https://doc.rust-lang.org/book/ch04-03-slices.html#other-slices" target="_blank">TRPL - Other Slices</a>.



<!--
###############################################################################
## Borrowing Rules Summary
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - What are the borrowing rules in Rust?
Answer   :

```rust
fn main() {
    let mut data = vec![1, 2, 3];

    // Rule 1: Multiple immutable borrows OK
    let r1 = &data;
    let r2 = &data;
    println!("{:?} {:?}", r1, r2);

    // Rule 2: Only ONE mutable borrow at a time
    let r3 = &mut data;
    r3.push(4);
    println!("{:?}", r3);

    // Rule 3: Cannot mix mutable and immutable borrows
    // (r1 and r2 are no longer used, so this is fine)
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

**The Borrowing Rules:**
1. You can have **either** any number of immutable references **or** one mutable reference
2. References must always be **valid** (no dangling)

Read more in <a href="https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references" target="_blank">TRPL - The Rules of References</a>.



<!--
###############################################################################
## Common Pattern - Take and Return
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - How do you avoid "use after move" when passing to functions?
Answer   :

```rust
// Option 1: Borrow instead of taking ownership
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

// Option 2: Take ownership and return it
fn add_exclaim(mut s: String) -> String {
    s.push('!');
    s
}

// Option 3: Clone if you need both
fn main() {
    let s = String::from("hello");

    // Borrow: s stays valid
    print_length(&s);
    println!("Still have: {}", s);

    // Move and get back
    let s = add_exclaim(s);
    println!("Modified: {}", s);

    // Clone: keep original, work with copy
    let s2 = s.clone();
    let s3 = add_exclaim(s2);
    println!("Original: {}, New: {}", s, s3);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Three strategies to avoid losing ownership:
1. **Borrow** (`&T` or `&mut T`): most common
2. **Return ownership**: when modification needed
3. **Clone**: when you need independent copies

Read more in <a href="https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html" target="_blank">TRPL - References and Borrowing</a>.



<!--
###############################################################################
## The Drop Trait
###############################################################################
-->
Question : Beginner - Ownership and Borrowing - When and how is memory freed in Rust?
Answer   :

```rust
struct CustomDrop {
    name: String,
}

impl Drop for CustomDrop {
    fn drop(&mut self) {
        println!("Dropping: {}", self.name);
    }
}

fn main() {
    let a = CustomDrop { name: String::from("first") };
    let b = CustomDrop { name: String::from("second") };

    println!("Created a and b");

    {
        let c = CustomDrop { name: String::from("inner") };
        println!("Created c in inner scope");
    }  // c is dropped here

    println!("Back in main");
}  // b then a are dropped (reverse order)
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

The `Drop` trait's `drop` method is called automatically when a value goes out of scope. Variables are dropped in **reverse order** of creation. This is RAII (Resource Acquisition Is Initialization).

Read more in <a href="https://doc.rust-lang.org/book/ch15-03-drop.html" target="_blank">TRPL - Running Code on Cleanup with the Drop Trait</a>.