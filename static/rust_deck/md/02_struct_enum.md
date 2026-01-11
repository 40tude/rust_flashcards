<!--
###############################################################################
## Structs - Definition
###############################################################################
-->
Question : Beginner - Structs & Enums - How do you define and instantiate a struct?
Answer   :

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("User: {} ({})", user1.username, user1.email);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

A **struct** groups related data together. Define with `struct` keyword, instantiate with `StructName { field: value }` syntax. All fields must be initialized.

Read more in <a href="https://doc.rust-lang.org/book/ch05-01-defining-structs.html" target="_blank">TRPL - Defining and Instantiating Structs</a>.



<!--
###############################################################################
## Structs - Mutable Instances
###############################################################################
-->
Question : Beginner - Structs & Enums - How do you modify struct fields?
Answer   :

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let mut user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };

    // Entire instance must be mutable
    user1.email = String::from("newalice@example.com");
    user1.active = false;

    println!("Updated email: {}", user1.email);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

The **entire instance** must be marked `mut`: Rust doesn't allow only certain fields to be mutable.

Read more in <a href="https://doc.rust-lang.org/book/ch05-01-defining-structs.html" target="_blank">TRPL - Defining and Instantiating Structs</a>.



<!--
###############################################################################
## Structs - Field Init Shorthand
###############################################################################
-->
Question : Beginner - Structs & Enums - What is the field init shorthand in structs?
Answer   :

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,   // Shorthand: same as username: username
        email,      // Shorthand: same as email: email
        active: true,
    }
}

fn main() {
    let user = build_user(
        String::from("bob"),
        String::from("bob@example.com"),
    );
    println!("Created user: {}", user.username);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

When variable names match field names, you can use the **shorthand** `field` instead of `field: field`.

Read more in <a href="https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-the-field-init-shorthand" target="_blank">TRPL - Using the Field Init Shorthand</a>.



<!--
###############################################################################
## Structs - Update Syntax
###############################################################################
-->
Question : Beginner - Structs & Enums - How do you create a struct from another struct's values?
Answer   :

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
        sign_in_count: 1,
    };

    // Struct update syntax: ..user1 fills remaining fields
    let user2 = User {
        email: String::from("bob@example.com"),
        ..user1  // Gets username, active, sign_in_count from user1
    };

    // Note: user1.username was MOVED to user2
    // println!("{}", user1.username);  // Error!
    println!("user2: {} ({})", user2.username, user2.email);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

The **struct update syntax** `..other` copies/moves remaining fields from another instance. Be aware: non-Copy fields are **moved**.

Read more in <a href="https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax" target="_blank">TRPL - Struct Update Syntax</a>.



<!--
###############################################################################
## Structs - Tuple Structs
###############################################################################
-->
Question : Beginner - Structs & Enums - What is a tuple struct?
Answer   :

```rust
struct Color(u8, u8, u8);
struct Point(f64, f64, f64);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0, 0.0);

    // Access by index
    println!("Red component: {}", black.0);
    println!("X coordinate: {}", origin.0);

    // Destructuring
    let Color(r, g, b) = black;
    println!("RGB: {}, {}, {}", r, g, b);

    // Color and Point are different types even with same field types
    // let p: Point = black;  // Error: mismatched types
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

**Tuple structs** have named types but unnamed fields. Useful when you want a distinct type without field names. Access fields by index.

Read more in <a href="https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types" target="_blank">TRPL - Tuple Structs</a>.



<!--
###############################################################################
## Structs - Unit-Like Structs
###############################################################################
-->
Question : Beginner - Structs & Enums - What is a unit-like struct?
Answer   :

```rust
struct AlwaysEqual;

trait MyTrait {
    fn do_something(&self);
}

impl MyTrait for AlwaysEqual {
    fn do_something(&self) {
        println!("I'm a unit struct implementing a trait!");
    }
}

fn main() {
    let subject = AlwaysEqual;
    subject.do_something();

    println!("Size of AlwaysEqual: {} bytes", std::mem::size_of::<AlwaysEqual>());
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

**Unit-like structs** have no fields. They're useful for implementing traits on a type without storing data. They have zero size.

Read more in <a href="https://doc.rust-lang.org/book/ch05-01-defining-structs.html#unit-like-structs-without-any-fields" target="_blank">TRPL - Unit-Like Structs</a>.



<!--
###############################################################################
## Structs - Debug Printing
###############################################################################
-->
Question : Beginner - Structs & Enums - How do you print a struct for debugging?
Answer   :

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // Debug format
    println!("rect = {:?}", rect);

    // Pretty debug format
    println!("rect = {:#?}", rect);

    // dbg! macro (prints to stderr with file/line info)
    dbg!(&rect);

    // dbg! returns ownership, so you can use it inline
    let area = dbg!(rect.width * rect.height);
    println!("Area: {}", area);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Derive `Debug` to enable `{:?}` formatting. Use `{:#?}` for pretty-printing. The `dbg!` macro is great for quick debugging.

Read more in <a href="https://doc.rust-lang.org/book/ch05-02-example-structs.html#adding-useful-functionality-with-derived-traits" target="_blank">TRPL - Adding Useful Functionality with Derived Traits</a>.



<!--
###############################################################################
## Structs - Methods (impl)
###############################################################################
-->
Question : Beginner - Structs & Enums - How do you define methods on a struct?
Answer   :

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };

    println!("Area of rect1: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Define methods in an `impl` block. The first parameter is `self` (the instance). Use `&self` for borrowing, `&mut self` for mutable access, `self` for taking ownership.

Read more in <a href="https://doc.rust-lang.org/book/ch05-03-method-syntax.html" target="_blank">TRPL - Method Syntax</a>.



<!--
###############################################################################
## Structs - Associated Functions
###############################################################################
-->
Question : Beginner - Structs & Enums - What is an associated function (constructor)?
Answer   :

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (no self) - often used as constructor
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);     // Call with ::
    let square = Rectangle::square(25);

    println!("rect: {:?}, area: {}", rect, rect.area());
    println!("square: {:?}, area: {}", square, square.area());
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

**Associated functions** don't take `self`: they're called with `::` syntax (like `String::from`). `Self` is an alias for the type. Commonly used for constructors.

Read more in <a href="https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions" target="_blank">TRPL - Associated Functions</a>.



<!--
###############################################################################
## Enums - Definition
###############################################################################
-->
Question : Beginner - Structs & Enums - How do you define and use an enum?
Answer   :

```rust
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn move_player(dir: Direction) {
    match dir {
        Direction::North => println!("Moving up!"),
        Direction::South => println!("Moving down!"),
        Direction::East => println!("Moving right!"),
        Direction::West => println!("Moving left!"),
    }
}

fn main() {
    let dir = Direction::North;
    println!("Direction: {:?}", dir);
    move_player(dir);
    move_player(Direction::East);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

An **enum** defines a type with a fixed set of variants. Each variant is accessed with `EnumName::Variant` syntax.

Read more in <a href="https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html" target="_blank">TRPL - Defining an Enum</a>.



<!--
###############################################################################
## Enums - With Data
###############################################################################
-->
Question : Beginner - Structs & Enums - How do you attach data to enum variants?
Answer   :

```rust
#[derive(Debug)]
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Named fields (like struct)
    Write(String),              // Single value
    ChangeColor(u8, u8, u8),    // Multiple values (like tuple)
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 128, 0);

    println!("{:?}", m1);
    println!("{:?}", m2);
    println!("{:?}", m3);
    println!("{:?}", m4);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Enum variants can hold data: nothing, named fields, single values, or tuples. Each variant can have different data types.

Read more in <a href="https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#enum-values" target="_blank">TRPL - Enum Values</a>.



<!--
###############################################################################
## Enums - Methods
###############################################################################
-->
Question : Beginner - Structs & Enums - Can enums have methods?
Answer   :

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Text: {}", text),
        }
    }

    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }
}

fn main() {
    let msg = Message::Write(String::from("hello"));
    msg.call();
    println!("Is quit? {}", msg.is_quit());

    let quit = Message::Quit;
    quit.call();
    println!("Is quit? {}", quit.is_quit());
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

**Yes!** Enums can have methods defined in `impl` blocks, just like structs. The `matches!` macro is handy for simple variant checks.

Read more in <a href="https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html" target="_blank">TRPL - Defining an Enum</a>.