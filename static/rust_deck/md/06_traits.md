<!--
###############################################################################
## Traits - Definition
###############################################################################
-->
Question : Beginner - Traits - How do you define a trait in Rust?
Answer   :

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}...", self.title, &self.content[..20.min(self.content.len())])
    }
}

struct Tweet {
    username: String,
    text: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.text)
    }
}

fn main() {
    let article = Article {
        title: String::from("Breaking News"),
        content: String::from("Something important happened today in the world."),
    };
    let tweet = Tweet {
        username: String::from("rustlang"),
        text: String::from("Hello, Rustaceans!"),
    };

    println!("{}", article.summarize());
    println!("{}", tweet.summarize());
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

A **trait** defines shared behavior. Use `trait Name { fn method(&self); }` to define, and `impl Trait for Type` to implement. Each type can have its own implementation.

Read more in <a href="https://doc.rust-lang.org/book/ch10-02-traits.html" target="_blank">TRPL - Traits: Defining Shared Behavior</a>.



<!--
###############################################################################
## Traits - Default Implementation
###############################################################################
-->
Question : Beginner - Traits - How do you provide a default implementation for a trait method?
Answer   :

```rust
trait Greet {
    fn name(&self) -> &str;

    // Default implementation
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }

    fn loud_greet(&self) -> String {
        format!("HELLO, {}!!!", self.name().to_uppercase())
    }
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn name(&self) -> &str {
        &self.name
    }
    // greet() uses default implementation
    // loud_greet() uses default implementation
}

struct Robot {
    id: u32,
}

impl Greet for Robot {
    fn name(&self) -> &str {
        "Robot"
    }

    // Override default
    fn greet(&self) -> String {
        format!("Beep boop. Unit {} online.", self.id)
    }
}

fn main() {
    let person = Person { name: String::from("Alice") };
    let robot = Robot { id: 42 };

    println!("{}", person.greet());       // Uses default
    println!("{}", person.loud_greet());  // Uses default
    println!("{}", robot.greet());        // Uses override
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Traits can have **default implementations**. Types can use the default or override it. Default methods can call other trait methods.

Read more in <a href="https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations" target="_blank">TRPL - Default Implementations</a>.



<!--
###############################################################################
## Traits - Derive
###############################################################################
-->
Question : Beginner - Traits - What does #[derive(...)] do?
Answer   :

```rust
// Automatically implement common traits
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, Default)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();  // Clone
    println!("{:?}", p1); // Debug
    println!("Equal? {}", p1 == p2);  // PartialEq

    let c1 = Color { r: 255, g: 0, b: 0 };
    let c2 = c1;  // Copy (not move!)
    println!("c1: {:?}, c2: {:?}", c1, c2);

    let default_color: Color = Default::default();
    println!("Default: {:?}", default_color);  // Color { r: 0, g: 0, b: 0 }
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`#[derive(...)]` auto-generates trait implementations. Common derivable traits:

| Trait | Purpose |
|-------|---------|
| `Debug` | `{:?}` formatting |
| `Clone` | `.clone()` method |
| `Copy` | Implicit copy instead of move |
| `PartialEq` | `==` and `!=` |
| `Default` | `Default::default()` |

Read more in <a href="https://doc.rust-lang.org/book/appendix-03-derivable-traits.html" target="_blank">TRPL - Appendix C: Derivable Traits</a>.



<!--
###############################################################################
## Traits - Common Traits (Debug, Display)
###############################################################################
-->
Question : Beginner - Traits - What is the difference between Debug and Display traits?
Answer   :

```rust
use std::fmt;

#[derive(Debug)]  // Can be derived
struct Point {
    x: i32,
    y: i32,
}

// Display must be implemented manually
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };

    // Debug: {:?} - for developers
    println!("Debug: {:?}", p);
    println!("Pretty debug: {:#?}", p);

    // Display: {} - for users
    println!("Display: {}", p);

    // to_string() uses Display
    let s: String = p.to_string();
    println!("As string: {}", s);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

| Trait     | Format | Purpose            | Derivable? |
|-----------|--------|--------------------|------------|
| `Debug`   | `{:?}` | Developer output   | Yes        |
| `Display` | `{}`   | User-facing output | No         |

`Display` also enables `.to_string()`.

Read more in <a href="https://doc.rust-lang.org/std/fmt/trait.Display.html" target="_blank">std::fmt::Display</a>.



<!--
###############################################################################
## Traits - Clone and Copy
###############################################################################
-->
Question : Beginner - Traits - What is the difference between Clone and Copy?
Answer   :

```rust
// Copy: implicit, bitwise copy, stack-only types
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// Clone only: explicit, can be expensive
#[derive(Debug, Clone)]
struct Document {
    title: String,
    content: String,
}

fn main() {
    // Copy: assignment copies, original still valid
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;  // Copy happens implicitly
    println!("p1: {:?}, p2: {:?}", p1, p2);  // Both valid!

    // Clone: must call .clone() explicitly
    let doc1 = Document {
        title: String::from("Hello"),
        content: String::from("World"),
    };
    let doc2 = doc1.clone();  // Explicit deep copy
    // let doc3 = doc1;       // This would MOVE, not copy
    println!("doc1: {:?}", doc1);  // Still valid because we cloned
    println!("doc2: {:?}", doc2);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

| Trait | When | Cost | Types |
|-------|------|------|-------|
| `Copy` | Implicit on assignment | Cheap (bitwise) | Simple stack types |
| `Clone` | Explicit `.clone()` | Can be expensive | Any type |

`Copy` requires `Clone`. Types with heap data can't be `Copy`.

Read more in <a href="https://doc.rust-lang.org/std/marker/trait.Copy.html" target="_blank">std::marker::Copy</a>.



<!--
###############################################################################
## Traits - Trait Bounds
###############################################################################
-->
Question : Beginner - Traits - How do you require a trait in a function parameter?
Answer   :

```rust
use std::fmt::Display;

// Syntax 1: impl Trait (simple)
fn print_summary(item: &impl Display) {
    println!("Summary: {}", item);
}

// Syntax 2: Trait bound (more flexible)
fn print_pair<T: Display>(a: &T, b: &T) {
    println!("{} and {}", a, b);
}

// Syntax 3: where clause (cleaner for multiple bounds)
fn complex_print<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Display,
{
    println!("t: {}, u: {}", t, u);
}

fn main() {
    print_summary(&"Hello, world!");
    print_summary(&42);

    print_pair(&1, &2);
    print_pair(&"foo", &"bar");

    complex_print(&String::from("hello"), &123);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Three syntaxes for trait bounds:
- `impl Trait`: simple, for single use
- `<T: Trait>`: when you need the type name
- `where T: Trait`: cleaner for complex bounds

Read more in <a href="https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters" target="_blank">TRPL - Traits as Parameters</a>.



<!--
###############################################################################
## Traits - Multiple Trait Bounds
###############################################################################
-->
Question : Beginner - Traits - How do you require multiple traits?
Answer   :

```rust
use std::fmt::{Debug, Display};

// Using + to combine traits
fn print_info<T: Display + Debug>(item: &T) {
    println!("Display: {}", item);
    println!("Debug: {:?}", item);
}

// Same with where clause
fn compare_and_print<T>(a: &T, b: &T)
where
    T: Display + PartialOrd,
{
    if a < b {
        println!("{} < {}", a, b);
    } else {
        println!("{} >= {}", a, b);
    }
}

fn main() {
    // i32 implements both Display and Debug
    print_info(&42);

    // Strings implement Display and PartialOrd
    compare_and_print(&"apple", &"banana");
    compare_and_print(&10, &5);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Use `+` to require multiple traits: `T: TraitA + TraitB`. This ensures the type implements all specified traits.

Read more in <a href="https://doc.rust-lang.org/book/ch10-02-traits.html#specifying-multiple-trait-bounds-with-the--syntax" target="_blank">TRPL - Multiple Trait Bounds</a>.



<!--
###############################################################################
## Traits - Returning impl Trait
###############################################################################
-->
Question : Beginner - Traits - How do you return a type that implements a trait?
Answer   :

```rust
use std::fmt::Display;

// Return some type that implements Display
fn make_greeting(name: &str) -> impl Display {
    format!("Hello, {}!", name)
}

// Useful for iterators (hides complex types)
fn even_numbers(limit: i32) -> impl Iterator<Item = i32> {
    (0..limit).filter(|x| x % 2 == 0)
}

// Note: can only return ONE concrete type
// This would NOT compile:
// fn either(flag: bool) -> impl Display {
//     if flag {
//         "hello"      // &str
//     } else {
//         42           // i32 - different type!
//     }
// }

fn main() {
    let greeting = make_greeting("Rust");
    println!("{}", greeting);

    let evens: Vec<i32> = even_numbers(10).collect();
    println!("Even numbers: {:?}", evens);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`-> impl Trait` lets you return an opaque type that implements a trait. Useful for hiding complex types like iterators. **Limitation**: must return a single concrete type.

Read more in <a href="https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits" target="_blank">TRPL - Returning Types That Implement Traits</a>.