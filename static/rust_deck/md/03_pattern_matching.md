<!--
###############################################################################
## Match - Basic Syntax
###############################################################################
-->
Question : Beginner - Pattern Matching - How does the match expression work?
Answer   :

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Dime;
    println!("Value: {} cents", value_in_cents(coin));
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`match` compares a value against patterns and executes the matching arm. It's an **expression** (returns a value). All arms must return the same type.

Read more in <a href="https://doc.rust-lang.org/book/ch06-02-match.html" target="_blank">TRPL - The match Control Flow Construct</a>.



<!--
###############################################################################
## Match - Exhaustiveness
###############################################################################
-->
Question : Beginner - Pattern Matching - What does it mean that match must be exhaustive?
Answer   :

```rust
fn describe_number(n: i32) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "something else",  // Catch-all pattern
    }
}

fn main() {
    println!("{}", describe_number(1));
    println!("{}", describe_number(2));
    println!("{}", describe_number(42));

    // Without the _ arm, this wouldn't compile:
    // match must cover ALL possible values
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`match` must cover **all possible values**. The `_` pattern matches anything and is often used as a catch-all. The compiler enforces exhaustiveness.

Read more in <a href="https://doc.rust-lang.org/book/ch06-02-match.html#matches-are-exhaustive" target="_blank">TRPL - Matches Are Exhaustive</a>.



<!--
###############################################################################
## Match - Binding Values
###############################################################################
-->
Question : Beginner - Pattern Matching - How do you extract data from enum variants in match?
Answer   :

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: rgb({}, {}, {})", r, g, b),
    }
}

fn main() {
    process(Message::Move { x: 10, y: 20 });
    process(Message::Write(String::from("hello")));
    process(Message::ChangeColor(255, 128, 0));
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Patterns can **bind** to parts of matched values. The bound variables are available in that arm's code block.

Read more in <a href="https://doc.rust-lang.org/book/ch06-02-match.html#patterns-that-bind-to-values" target="_blank">TRPL - Patterns That Bind to Values</a>.



<!--
###############################################################################
## Match - Multiple Patterns
###############################################################################
-->
Question : Beginner - Pattern Matching - How do you match multiple patterns in one arm?
Answer   :

```rust
fn describe_char(c: char) -> &'static str {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
        'A' | 'E' | 'I' | 'O' | 'U' => "uppercase vowel",
        '0'..='9' => "digit",
        _ => "other",
    }
}

fn main() {
    println!("'a' is a {}", describe_char('a'));
    println!("'E' is a {}", describe_char('E'));
    println!("'5' is a {}", describe_char('5'));
    println!("'z' is {}", describe_char('z'));
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Use `|` (or) to match multiple patterns. Use `..=` for inclusive ranges. These make patterns more concise.

Read more in <a href="https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#multiple-patterns" target="_blank">TRPL - Multiple Patterns</a>.



<!--
###############################################################################
## Match - Guards
###############################################################################
-->
Question : Beginner - Pattern Matching - What is a match guard?
Answer   :

```rust
fn classify_number(n: i32) -> &'static str {
    match n {
        x if x < 0 => "negative",
        x if x == 0 => "zero",
        x if x % 2 == 0 => "positive even",
        _ => "positive odd",
    }
}

fn main() {
    println!("-5: {}", classify_number(-5));
    println!("0: {}", classify_number(0));
    println!("4: {}", classify_number(4));
    println!("7: {}", classify_number(7));
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

A **match guard** is an extra `if` condition after the pattern. The arm only matches if both the pattern and the guard are true.

Read more in <a href="https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#extra-conditionals-with-match-guards" target="_blank">TRPL - Extra Conditionals with Match Guards</a>.



<!--
###############################################################################
## Match - @ Bindings
###############################################################################
-->
Question : Beginner - Pattern Matching - How do you bind a value while also testing it with @?
Answer   :

```rust
enum Status {
    Active { id: u32 },
    Inactive,
}

fn check_status(status: Status) {
    match status {
        Status::Active { id: id_value @ 1..=100 } => {
            println!("Active with low ID: {}", id_value)
        }
        Status::Active { id } => {
            println!("Active with high ID: {}", id)
        }
        Status::Inactive => println!("Inactive"),
    }
}

fn main() {
    check_status(Status::Active { id: 42 });
    check_status(Status::Active { id: 500 });
    check_status(Status::Inactive);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

The `@` operator lets you **bind** a value to a variable **while** testing it against a pattern. Useful for capturing values that match a range.

Read more in <a href="https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#-bindings" target="_blank">TRPL - @ Bindings</a>.



<!--
###############################################################################
## if let - Syntax
###############################################################################
-->
Question : Beginner - Pattern Matching - When should you use if let instead of match?
Answer   :

```rust
fn main() {
    let some_value: Option<i32> = Some(42);

    // Verbose: match when you only care about one pattern
    match some_value {
        Some(x) => println!("Got: {}", x),
        _ => (),
    }

    // Concise: if let for single pattern matching
    if let Some(x) = some_value {
        println!("Got: {}", x);
    }

    // With else
    let another: Option<i32> = None;
    if let Some(x) = another {
        println!("Got: {}", x);
    } else {
        println!("Got nothing");
    }
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Use `if let` when you only care about **one pattern** and want to ignore the rest. It's syntactic sugar for a `match` with one arm and a catch-all.

Read more in <a href="https://doc.rust-lang.org/book/ch06-03-if-let.html" target="_blank">TRPL - Concise Control Flow with if let</a>.



<!--
###############################################################################
## while let
###############################################################################
-->
Question : Beginner - Pattern Matching - How does while let work?
Answer   :

```rust
fn main() {
    let mut stack = vec![1, 2, 3, 4, 5];

    // Loop while pop() returns Some
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }

    println!("Stack is now empty: {:?}", stack);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`while let` continues looping as long as the pattern matches. When it stops matching (e.g., `None`), the loop ends. Perfect for iterating until exhaustion.

Read more in <a href="https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html#conditional-while-let-loops" target="_blank">TRPL - Conditional while let Loops</a>.



<!--
###############################################################################
## let else
###############################################################################
-->
Question : Beginner - Pattern Matching - What is let-else and when do you use it?
Answer   :

```rust
fn get_count(text: &str) -> Option<u32> {
    text.parse().ok()
}

fn process(input: &str) {
    // let-else: bind or diverge (return, break, panic, etc.)
    let Some(count) = get_count(input) else {
        println!("'{}' is not a valid number, skipping", input);
        return;
    };

    // count is available here, guaranteed to be valid
    println!("Processing count: {}", count);
}

fn main() {
    process("42");
    process("hello");
    process("100");
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`let-else` tries to match a pattern; if it fails, the `else` block must **diverge** (return, break, continue, or panic). Great for early returns on failure.

Read more in <a href="https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html" target="_blank">Rust by Example - let-else</a>.



<!--
###############################################################################
## Destructuring - Structs
###############################################################################
-->
Question : Beginner - Pattern Matching - How do you destructure structs in patterns?
Answer   :

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 10, y: 20 };

    // Destructure with different names
    let Point { x: a, y: b } = p;
    println!("a={}, b={}", a, b);

    // Shorthand when variable names match field names
    let Point { x, y } = p;
    println!("x={}, y={}", x, y);

    // In match
    match p {
        Point { x: 0, y } => println!("On y-axis at y={}", y),
        Point { x, y: 0 } => println!("On x-axis at x={}", x),
        Point { x, y } => println!("At ({}, {})", x, y),
    }
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Destructure structs using `StructName { field: var }` or shorthand `{ field }`. You can match specific values or bind to variables.

Read more in <a href="https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-structs" target="_blank">TRPL - Destructuring Structs</a>.