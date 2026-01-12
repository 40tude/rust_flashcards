
<!--
###############################################################################
## Option - Definition
###############################################################################
-->
Question : Beginner - Error Handling - What is `Option<T>` and when do you use it?
Answer   :

```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn main() {
    let nums = vec![1, 3, 5, 8, 9];
    let empty: Vec<i32> = vec![1, 3, 5];

    match find_first_even(&nums) {
        Some(n) => println!("Found even: {}", n),
        None => println!("No even number found"),
    }

    match find_first_even(&empty) {
        Some(n) => println!("Found even: {}", n),
        None => println!("No even number found"),
    }
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`Option<T>` represents a value that may or may not exist: `Some(value)` or `None`. It replaces null pointers and forces explicit handling of the "no value" case.

Read this <a href="https://www.40tude.fr/docs/06_programmation/rust/020_some/some_00.html" target="_blank">post </a> on <a href="https://www.40tude.fr/docs/06_programmation/rust/" target="_blank">40tude.fr</a>.

Read more in <a href="https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values" target="_blank">TRPL - The Option Enum</a>.



<!--
###############################################################################
## Option - unwrap and expect
###############################################################################
-->
Question : Beginner - Error Handling - What do `.unwrap()` and `.expect()` do on `Option<T>`?
Answer   :

```rust
fn main() {
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    // unwrap: returns value or panics with generic message
    println!("Unwrapped: {}", some_value.unwrap());

    // expect: returns value or panics with custom message
    println!("Expected: {}", some_value.expect("Should have a value"));

    // These would panic:
    // none_value.unwrap();  // panic: "called `Option::unwrap()` on a `None` value"
    // none_value.expect("This will panic!");  // panic: "This will panic!"

    // Safe alternative: unwrap_or
    println!("With default: {}", none_value.unwrap_or(0));

    // unwrap_or_else: lazy evaluation
    println!("Computed default: {}", none_value.unwrap_or_else(|| 2 + 2));
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`.unwrap()` extracts the value or panics. `.expect()` does the same but with a custom panic message. Use `.unwrap_or()` or `.unwrap_or_else()` for safe defaults.

Read this <a href="https://www.40tude.fr/docs/06_programmation/rust/020_some/some_01.html" target="_blank">post </a> on <a href="https://www.40tude.fr/docs/06_programmation/rust/" target="_blank">40tude.fr</a>.

Read more in <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap" target="_blank">std::option::Option::unwrap</a>.



<!--
###############################################################################
## Option - map and and_then
###############################################################################
-->
Question : Beginner - Error Handling - How do you transform `Option<T>` values with `.map()` and `.and_then()`?
Answer   :

```rust
fn main() {
    let some_string = Some("42");
    let none_string: Option<&str> = None;

    // map: transform the inner value (if Some)
    let length: Option<usize> = some_string.map(|s| s.len());
    println!("Length: {:?}", length);  // Some(2)

    let none_length: Option<usize> = none_string.map(|s| s.len());
    println!("None length: {:?}", none_length);  // None

    // and_then (flatMap): when the transform returns an Option<T>
    let parsed: Option<i32> = some_string.and_then(|s| s.parse().ok());
    println!("Parsed: {:?}", parsed);  // Some(42)

    let bad_parse: Option<i32> = Some("hello").and_then(|s| s.parse().ok());
    println!("Bad parse: {:?}", bad_parse);  // None
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`.map()` transforms `Some(x)` to `Some(f(x))`, leaving `None` unchanged. `.and_then()` (aka flatMap) is for when your function also returns an `Option<T>`.

Read this <a href="https://www.40tude.fr/docs/06_programmation/rust/020_some/some_02.html" target="_blank">post </a> on <a href="https://www.40tude.fr/docs/06_programmation/rust/" target="_blank">40tude.fr</a>.

Read more in <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.map" target="_blank">std::option::Option::map</a>.



<!--
###############################################################################
## Result - Definition
###############################################################################
-->
Question : Beginner - Error Handling - What is `Result<T, E>` and when do you use it?
Answer   :

```rust
use std::num::ParseIntError;

fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

fn main() {
    let good = parse_number("42");
    let bad = parse_number("hello");

    match good {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match bad {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`Result<T, E>` represents success (`Ok(value)`) or failure (`Err(error)`). Use it for operations that can fail with meaningful error information.

Read this <a href="https://www.40tude.fr/docs/06_programmation/rust/016_errors/errors_00.html#the-resultt-e-type-handling-recoverable-errors" target="_blank">post </a> on <a href="https://www.40tude.fr/docs/06_programmation/rust/" target="_blank">40tude.fr</a>.

Read more in <a href="https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html" target="_blank">TRPL - Recoverable Errors with Result</a>.



<!--
###############################################################################
## Result - unwrap and expect
###############################################################################
-->
Question : Beginner - Error Handling - How do `.unwrap()` and `.expect()` work on `Result<T, E>`?
Answer   :

```rust
fn main() {
    let good: Result<i32, &str> = Ok(42);
    let bad: Result<i32, &str> = Err("something went wrong");

    // unwrap: returns value or panics with error
    println!("Good: {}", good.unwrap());

    // expect: panics with custom message + error
    // bad.expect("Failed to get value");  // Would panic

    // Safe alternatives
    println!("With default: {}", bad.unwrap_or(0));
    println!("Computed: {}", bad.unwrap_or_else(|_e| 99));

    // unwrap_or_default: uses Default trait
    let result: Result<String, &str> = Err("error");
    println!("Default string: '{}'", result.unwrap_or_default());
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Like `Option`, `Result` has `.unwrap()`, `.expect()`, `.unwrap_or()`, and `.unwrap_or_else()`. There's also `.unwrap_or_default()` for types implementing `Default`.

Read this <a href="https://www.40tude.fr/docs/06_programmation/rust/016_errors/errors_00.html" target="_blank">post </a> on <a href="https://www.40tude.fr/docs/06_programmation/rust/" target="_blank">40tude.fr</a>.

Read more in <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap" target="_blank">std::result::Result::unwrap</a>.



<!--
###############################################################################
## The ? Operator
###############################################################################
-->
Question : Beginner - Error Handling - How does the `?` operator work?
Answer   :

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;  // Returns early if Err
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;  // Returns early if Err
    Ok(contents)
}

// Equivalent without ?:
fn read_file_verbose(path: &str) -> Result<String, io::Error> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}

fn main() {
    match read_file_contents("nonexistent.txt") {
        Ok(contents) => println!("Contents: {}", contents),
        Err(e) => println!("Error: {}", e),
    }
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

The `?` operator unwraps `Ok` or returns `Err` early. It's shorthand for match + early return. The function must return a compatible `Result` type.

Read this <a href="https://www.40tude.fr/docs/06_programmation/rust/016_errors/errors_00.html#propagating-errors-with--operator" target="_blank">post </a> on <a href="https://www.40tude.fr/docs/06_programmation/rust/" target="_blank">40tude.fr</a>.

Read more in <a href="https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator" target="_blank">TRPL - The ? Operator</a>.



<!--
###############################################################################
## ? with Option
###############################################################################
-->
Question : Beginner - Error Handling - Can you use `?` with `Option<T>`?
Answer   :

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn add_last_numbers(a: &str, b: &str) -> Option<i32> {
    let x: i32 = a.parse().ok()?;  // Convert Result to Option, then ?
    let y: i32 = b.parse().ok()?;
    Some(x + y)
}

fn main() {
    println!("{:?}", last_char_of_first_line("hello\nworld"));  // Some('o')
    println!("{:?}", last_char_of_first_line(""));  // None

    println!("{:?}", add_last_numbers("10", "20"));  // Some(30)
    println!("{:?}", add_last_numbers("10", "abc")); // None
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

**Yes!** `?` works with `Option` too: it returns `None` early if the value is `None`. The function must return `Option`. Use `.ok()` to convert `Result` to `Option`.

Read more in <a href="https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator" target="_blank">TRPL - The ? Operator</a>.



<!--
###############################################################################
## panic! Macro
###############################################################################
-->
Question : Beginner - Error Handling - When should you use `panic!()`?
Answer   :

```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!");
    }
    a / b
}

fn main() {
    println!("10 / 2 = {}", divide(10, 2));

    // This would panic:
    // println!("10 / 0 = {}", divide(10, 0));

    // Other ways to panic
    // unreachable!();       // For code that should never execute
    // unimplemented!();     // For not-yet-implemented code
    // todo!();              // Same as unimplemented!, clearer intent
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`panic!()` crashes the program immediately. Use it for **unrecoverable** errors or programming bugs. For recoverable errors, use `Result` instead.

Related macros: `unreachable!()`, `unimplemented!()`, `todo!()`.

Read this <a href="https://www.40tude.fr/docs/06_programmation/rust/016_errors/errors_01.html#to-panic-or-not-to-panic" target="_blank">post </a> on <a href="https://www.40tude.fr/docs/06_programmation/rust/" target="_blank">40tude.fr</a>.


Read more in <a href="https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html" target="_blank">TRPL - Unrecoverable Errors with panic!</a>.



<!--
###############################################################################
## Converting Between Option and Result
###############################################################################
-->
Question : Beginner - Error Handling - How do you convert between `Option<T>` and `Result<T, E>`?
Answer   :

```rust
fn main() {
    // Option to Result
    let opt: Option<i32> = Some(42);
    let res: Result<i32, &str> = opt.ok_or("no value");
    println!("Option->Result: {:?}", res);

    let none: Option<i32> = None;
    let res2: Result<i32, &str> = none.ok_or("no value");
    println!("None->Result: {:?}", res2);

    // Result to Option
    let ok: Result<i32, &str> = Ok(42);
    let err: Result<i32, &str> = Err("error");

    println!("Ok->Option: {:?}", ok.ok());   // Some(42)
    println!("Err->Option: {:?}", err.ok()); // None

    // Keep error info
    println!("Ok->err(): {:?}", ok.err());   // None
    println!("Err->err(): {:?}", err.err()); // Some("error")
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

| Conversion | Method |
|------------|--------|
| `Option` → `Result` | `.ok_or(err)` or `.ok_or_else(\|\| err)` |
| `Result` → `Option` (keep Ok) | `.ok()` |
| `Result` → `Option` (keep Err) | `.err()` |

Read more in <a href="https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or" target="_blank">std::option::Option::ok_or</a>.



<!--
###############################################################################
## The Main Function with Result
###############################################################################
-->
Question : Beginner - Error Handling - Can `main()` return a `Result<T, E>`?
Answer   :

```rust
use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File contents: {}", contents);
    Ok(())
}

// Or with a more flexible error type:
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Can use ? with any error type
//     Ok(())
// }

// Or even better if you plan to transition from experimentation to production
// pub type Error = Box<dyn std::error::Error>;
// pub type Result<T> = std::result::Result<T, Error>;

// fn main() -> Result<()> {
//     println!("Hello, world!");
//     Ok(())
// }
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

**Yes!** `main()` can return `Result<(), E>`. If it returns `Err`, Rust prints the error and exits with a non-zero code. Use `Box<dyn Error>` for flexibility.

Read this <a href="https://www.40tude.fr/docs/06_programmation/rust/016_errors/errors_01.html#custom-error-types-and-error-handling-in-larger-programs" target="_blank">post </a> on <a href="https://www.40tude.fr/docs/06_programmation/rust/" target="_blank">40tude.fr</a>.

Read more in <a href="https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result" target="_blank">TRPL - Recoverable Errors with Result</a>.




<!--
###############################################################################
##
###############################################################################
-->
Question : Beginner - Error Handling - What could be the bare minimal code with rock solid error handling?
Answer   :


```rust
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
```

---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Read this <a href="https://www.40tude.fr/docs/06_programmation/rust/016_errors/errors_02.html" target="_blank">article</a> on <a href="https://www.40tude.fr/docs/06_programmation/rust/" target="_blank">40tude.fr</a>.
