<!--
###############################################################################
## Variables and Mutability
###############################################################################
-->
Question : Beginner - Basics - How do you declare an immutable variable in Rust?
Answer   :

```rust
fn main() {
    let x = 5;
    println!("x = {}", x);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

By default, variables in Rust are **immutable**. The `let` keyword declares a **variable binding**.

Read this <a href="https://www.40tude.fr/docs/06_programmation/rust/004_mutability/mutability_us.html" target="_blank">post </a> on <a href="https://www.40tude.fr/docs/06_programmation/rust/" target="_blank">40tude.fr</a>.

Read more in <a href="https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html" target="_blank">TRPL - Variables and Mutability</a>.



<!--
###############################################################################
## Variables and Mutability
###############################################################################
-->
Question : Beginner - Basics - How do you declare a mutable variable in Rust?
Answer   :

```rust
fn main() {
    let mut x = 5;
    println!("x = {}", x);
    x = 10;
    println!("x = {}", x);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Use the `mut` keyword after `let` to make a variable mutable.

Read more in <a href="https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html" target="_blank">TRPL - Variables and Mutability</a>.



<!--
###############################################################################
## Variables and Mutability
###############################################################################
-->
Question : Beginner - Basics - What is shadowing and how does it differ from mutability?
Answer   :

```rust
fn main() {
    let x = 5;
    let x = x + 1;        // Shadowing: new variable with same name
    let x = "hello";      // Can even change type!

    println!("x = {}", x);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

**Shadowing** creates a new variable with the same name. Unlike `mut`:
- You can change the type
- The variable remains immutable after each binding
- Requires `let` keyword each time

Read more in <a href="https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing" target="_blank">TRPL - Shadowing</a>.



<!--
###############################################################################
## Constants
###############################################################################
-->
Question : Beginner - Basics - How do you declare a constant in Rust?
Answer   :

```rust
const MAX_POINTS: u32 = 100_000; // constants are typed

fn main() {
    println!("Max points: {}", MAX_POINTS);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Constants use `const` instead of `let`, **must** have a type annotation, and can only be set to a constant expression (evaluated at compile time). Convention: `SCREAMING_SNAKE_CASE`.

Read more in <a href="https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#constants" target="_blank">TRPL - Constants</a>.



<!--
###############################################################################
## Scalar Types - Integers
###############################################################################
-->
Question : Beginner - Basics - What are the integer types in Rust?
Answer   :

| Signed | Unsigned | Size |
|--------|----------|------|
| `i8`   | `u8`     | 8-bit |
| `i16`  | `u16`    | 16-bit |
| `i32`  | `u32`    | 32-bit |
| `i64`  | `u64`    | 64-bit |
| `i128` | `u128`   | 128-bit |
| `isize`| `usize`  | arch-dependent |

```rust
fn main() {
    let a: i32 = -42;
    let b: u64 = 1_000_000;
    let c: usize = 10;  // Used for indexing

    println!("a={}, b={}, c={}", a, b, c);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Default integer type is `i32`. Use underscores for readability.

Read more in <a href="https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types" target="_blank">TRPL - Integer Types</a>.



<!--
###############################################################################
## Scalar Types - Floats
###############################################################################
-->
Question : Beginner - Basics - What are the floating-point types in Rust?
Answer   :

```rust
fn main() {
    let x = 2.0;        // f64 (default)
    let y: f32 = 3.14;  // f32

    println!("x={}, y={}", x, y);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Rust has two floating-point types:
- `f64` (64-bit, double precision) : **default**
- `f32` (32-bit, single precision)

Both are signed and follow IEEE-754 standard.

Read more in <a href="https://doc.rust-lang.org/book/ch03-02-data-types.html#floating-point-types" target="_blank">TRPL - Floating-Point Types</a>.



<!--
###############################################################################
## Scalar Types - Boolean
###############################################################################
-->
Question : Beginner - Basics - How do you use booleans in Rust?
Answer   :

```rust
fn main() {
    let t = true;
    let f: bool = false;

    if t {
        println!("It's true!");
    }

    println!("t={}, f={}", t, f);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

The `bool` type has two values: `true` and `false`. Booleans are one byte in size.

Read more in <a href="https://doc.rust-lang.org/book/ch03-02-data-types.html#the-boolean-type" target="_blank">TRPL - The Boolean Type</a>.



<!--
###############################################################################
## Scalar Types - Character
###############################################################################
-->
Question : Beginner - Basics - What is the `char` type in Rust?
Answer   :

```rust
fn main() {
    let c = 'z';
    let heart = '❤';
    let emoji = '🦀';

    println!("{} {} {}", c, heart, emoji);
    println!("Size of char: {} bytes", std::mem::size_of::<char>());
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

The `char` type represents a **Unicode Scalar Value** and is 4 bytes in size. Use single quotes (double quotes are for strings).

Read more in <a href="https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type" target="_blank">TRPL - The Character Type</a>.



<!--
###############################################################################
## Compound Types - Tuples
###############################################################################
-->
Question : Beginner - Basics - How do you create and use `tuples` in Rust?
Answer   :

```rust
fn main() {
    let tup: (i32, f64, char) = (500, 6.4, 'y');

    // Destructuring
    let (x, y, z) = tup;
    println!("x={}, y={}, z={}", x, y, z);

    // Access by index
    println!("First: {}, Second: {}", tup.0, tup.1);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`tuple` groups values of different types. They have a fixed length. Access elements via destructuring or dot notation with index.

Read more in <a href="https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type" target="_blank">TRPL - The Tuple Type</a>.



<!--
###############################################################################
## Compound Types - Arrays
###############################################################################
-->
Question : Beginner - Basics - How do you create and use `array`s in Rust?
Answer   :

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];  // With type annotation
    let arr3 = [3; 5];                     // [3, 3, 3, 3, 3]

    println!("First: {}", arr[0]);
    println!("Length: {}", arr.len());
    println!("arr3: {:?}", arr3);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Arrays have a **fixed length** and all elements must be the same type. For dynamic sizing, use `Vec<T>` instead.

Read more in <a href="https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type" target="_blank">TRPL - The Array Type</a>.



<!--
###############################################################################
## Functions
###############################################################################
-->
Question : Beginner - Basics - How do you define a function with parameters in Rust?
Answer   :

```rust
fn greet(name: &str, times: i32) {
    for _ in 0..times {
        println!("Hello, {}!", name);
    }
}

fn main() {
    greet("Rustacean", 3);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Use `fn` keyword. Parameter types are **mandatory**. Convention: `snake_case` for function names.

Read more in <a href="https://doc.rust-lang.org/book/ch03-03-how-functions-work.html" target="_blank">TRPL - Functions</a>.



<!--
###############################################################################
## Functions - Return Values
###############################################################################
-->
Question : Beginner - Basics - How do you return a value from a function in Rust?
Answer   :

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = implicit return
}

fn subtract(a: i32, b: i32) -> i32 {
    return a - b;  // Explicit return also works
}

fn main() {
    println!("5 + 3 = {}", add(5, 3));
    println!("5 - 3 = {}", subtract(5, 3));
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Declare return type with `->`. The last expression without a semicolon is the return value. Use `return` for early returns.

Read more in <a href="https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions-with-return-values" target="_blank">TRPL - Functions with Return Values</a>.



<!--
###############################################################################
## Control Flow - if/else
###############################################################################
-->
Question : Beginner - Basics - How does `if/else` work in Rust?
Answer   :

```rust
fn main() {
    let number = 7;

    if number < 5 {
        println!("less than 5");
    } else if number == 5 {
        println!("equals 5");
    } else {
        println!("greater than 5");
    }

    // if as expression
    let result = if number > 5 { "big" } else { "small" };
    println!("Number is {}", result);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Condition must be a `bool` (no implicit conversion). `if` is an expression and can return values.

Read more in <a href="https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions" target="_blank">TRPL - if Expressions</a>.



<!--
###############################################################################
## Control Flow - loop
###############################################################################
-->
Question : Beginner - Basics - How do you use the `loop` keyword in Rust?
Answer   :

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // Return value from loop
        }
    };

    println!("Result: {}", result);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`loop` creates an infinite loop. Use `break` to exit (optionally with a value). Use `continue` to skip to next iteration.

Read more in <a href="https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops" target="_blank">TRPL - Repetition with Loops</a>.



<!--
###############################################################################
## Control Flow - while
###############################################################################
-->
Question : Beginner - Basics - How do you use a `while` loop in Rust?
Answer   :

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!");
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`while` loops while the condition is `true`. The condition must be a `bool`.

Read more in <a href="https://doc.rust-lang.org/book/ch03-05-control-flow.html#conditional-loops-with-while" target="_blank">TRPL - Conditional Loops with while</a>.



<!--
###############################################################################
## Control Flow - for
###############################################################################
-->
Question : Beginner - Basics - How do you use a `for` loop in Rust?
Answer   :

```rust
fn main() {
    // Iterate over a range
    for i in 0..5 {
        println!("i = {}", i);  // 0, 1, 2, 3, 4
    }

    // Inclusive range
    for i in 1..=3 {
        println!("i = {}", i);  // 1, 2, 3
    }

    // Iterate over array
    let arr = [10, 20, 30];
    for element in arr {
        println!("Value: {}", element);
    }
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`for` loops over iterators. Use `..` for exclusive range, `..=` for inclusive.

Read more in <a href="https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for" target="_blank">TRPL - Looping Through a Collection with for</a>.



<!--
###############################################################################
## Printing and Formatting
###############################################################################
-->
Question : Beginner - Basics - What are the common print macros in Rust?
Answer   :

```rust
fn main() {
    let name = "Rust";
    let version = 1.75;

    print!("No newline");
    println!(" - With newline");

    println!("Hello, {}!", name);           // Display
    println!("Version: {:.1}", version);    // Formatting
    println!("{:?}", (1, 2, 3));            // Debug
    println!("{:#?}", vec![1, 2, 3]);       // Pretty debug

    // Positional and named
    println!("{0} loves {1}, {0} is great", name, "safety");
    println!("{lang} is fun", lang = "Rust");
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`println!` adds newline, `print!` doesn't. Use `{}` for Display, `{:?}` for Debug formatting.

Read more in <a href="https://doc.rust-lang.org/std/fmt/index.html" target="_blank">std::fmt documentation</a>.



<!--
###############################################################################
## Comments
###############################################################################
-->
Question : Beginner - Basics - What types of comments does Rust support?
Answer   :

```rust
// This is a line comment

/* This is a
   block comment */

/// This is a doc comment for the following item
/// It supports **Markdown**!
fn documented_function() {
    //! This is an inner doc comment
    //! It documents the enclosing item
}

fn main() {
    // TODO: Comments are your friends
    println!("Comments don't affect execution");
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Use `///` for documentation comments (parsed by `rustdoc`). Doc comments support Markdown formatting.

Read more in <a href="https://doc.rust-lang.org/book/ch03-04-comments.html" target="_blank">TRPL - Comments</a> and <a href="https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html" target="_blank">rustdoc - How to write documentation</a>.