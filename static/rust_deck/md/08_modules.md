<!--
###############################################################################
## Modules - Inline Definition
###############################################################################
-->
Question : Beginner - Modules & Visibility - How do you define a module inline?
Answer   :

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    // Private by default
    fn secret_formula(x: i32) -> i32 {
        x * 42
    }

    pub fn use_secret(x: i32) -> i32 {
        secret_formula(x)  // Can access private fn within same module
    }
}

fn main() {
    println!("2 + 3 = {}", math::add(2, 3));
    println!("5 - 2 = {}", math::subtract(5, 2));
    // println!("{}", math::secret_formula(1));  // Error: private
    println!("Secret result: {}", math::use_secret(1));
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Use `mod name { ... }` to define a module inline. Items are **private by default**. Use `pub` to make them accessible outside the module.

Read more in <a href="https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html" target="_blank">TRPL - Defining Modules</a>.



<!--
###############################################################################
## Modules - Nested Modules
###############################################################################
-->
Question : Beginner - Modules & Visibility - How do you create nested modules?
Answer   :

```rust
mod outer {
    pub mod inner {
        pub fn greet() {
            println!("Hello from inner!");
        }

        pub mod deep {
            pub fn hello() {
                println!("Hello from deep!");
            }
        }
    }

    // Private module
    mod secret {
        pub fn whisper() {
            println!("This is secret...");
        }
    }

    pub fn use_secret() {
        secret::whisper();  // Can access sibling private module
    }
}

fn main() {
    outer::inner::greet();
    outer::inner::deep::hello();
    // outer::secret::whisper();  // Error: module `secret` is private
    outer::use_secret();  // OK: goes through public function
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Modules can be nested. Each level needs `pub` to be accessible from outside. Use `::` to navigate the module path.

Read more in <a href="https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html" target="_blank">TRPL - Defining Modules</a>.



<!--
###############################################################################
## Modules - use Keyword
###############################################################################
-->
Question : Beginner - Modules & Visibility - How does the use keyword work?
Answer   :

```rust
mod shapes {
    pub mod circle {
        pub const PI: f64 = 3.14159;

        pub fn area(radius: f64) -> f64 {
            PI * radius * radius
        }

        pub fn circumference(radius: f64) -> f64 {
            2.0 * PI * radius
        }
    }
}

// Bring items into scope
use shapes::circle::area;
use shapes::circle::PI;

// Or multiple items at once
use shapes::circle::{circumference, area as circle_area};

// Or everything (use sparingly)
// use shapes::circle::*;

fn main() {
    println!("PI = {}", PI);
    println!("Area: {}", area(2.0));
    println!("Circumference: {}", circumference(2.0));
    println!("Circle area: {}", circle_area(3.0));
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`use` brings items into scope so you don't need the full path. Use `as` for renaming, `{ }` for multiple items, `*` for glob imports (discouraged).

Read more in <a href="https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html" target="_blank">TRPL - Bringing Paths into Scope</a>.



<!--
###############################################################################
## Modules - self, super, crate
###############################################################################
-->
Question : Beginner - Modules & Visibility - What do self, super, and crate mean in module paths?
Answer   :

```rust
mod parent {
    pub fn parent_fn() {
        println!("In parent");
    }

    pub mod child {
        pub fn child_fn() {
            println!("In child");

            // super: go up one level
            super::parent_fn();

            // self: current module (often optional)
            self::helper();
        }

        fn helper() {
            println!("Helper in child");
        }

        pub fn call_sibling() {
            // super to go up, then down to sibling
            super::sibling::sibling_fn();
        }
    }

    pub mod sibling {
        pub fn sibling_fn() {
            println!("In sibling");

            // crate: absolute path from crate root
            crate::parent::parent_fn();
        }
    }
}

fn main() {
    parent::child::child_fn();
    println!("---");
    parent::child::call_sibling();
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

| Keyword | Meaning |
|---------|---------|
| `self` | Current module |
| `super` | Parent module |
| `crate` | Root of current crate |

Read more in <a href="https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html" target="_blank">TRPL - Paths for Referring to an Item</a>.



<!--
###############################################################################
## Visibility - pub Variations
###############################################################################
-->
Question : Beginner - Modules & Visibility - What are the different levels of pub visibility?
Answer   :

```rust
mod outer {
    pub mod inner {
        // Public everywhere
        pub fn public_fn() {
            println!("Fully public");
        }

        // Public within the crate only
        pub(crate) fn crate_fn() {
            println!("Crate-public");
        }

        // Public to parent module only
        pub(super) fn super_fn() {
            println!("Super-public");
        }

        // Public within specific path
        pub(in crate::outer) fn path_fn() {
            println!("Path-public");
        }

        // Private (default)
        fn private_fn() {
            println!("Private");
        }
    }

    pub fn test() {
        inner::public_fn();   // OK
        inner::crate_fn();    // OK
        inner::super_fn();    // OK - we are super
        inner::path_fn();     // OK - we are in path
        // inner::private_fn(); // Error: private
    }
}

fn main() {
    outer::inner::public_fn();  // OK
    outer::inner::crate_fn();   // OK - same crate
    // outer::inner::super_fn(); // Error: not super
    outer::test();
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

| Visibility | Accessible from |
|------------|-----------------|
| (none) | Same module only |
| `pub` | Everywhere |
| `pub(crate)` | Current crate |
| `pub(super)` | Parent module |
| `pub(in path)` | Specified path |

Read more in <a href="https://doc.rust-lang.org/reference/visibility-and-privacy.html" target="_blank">Rust Reference - Visibility and Privacy</a>.



<!--
###############################################################################
## Modules - Struct Visibility
###############################################################################
-->
Question : Beginner - Modules & Visibility - How does visibility work for struct fields?
Answer   :

```rust
mod shapes {
    // Public struct with private field
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
        serial: u64,  // Private field
    }

    impl Rectangle {
        // Constructor needed because serial is private
        pub fn new(width: u32, height: u32) -> Self {
            Self {
                width,
                height,
                serial: rand_serial(),
            }
        }

        pub fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    fn rand_serial() -> u64 {
        42  // Simplified
    }
}

fn main() {
    // Cannot create directly: serial is private
    // let r = shapes::Rectangle { width: 10, height: 20, serial: 1 };

    // Must use constructor
    let rect = shapes::Rectangle::new(10, 20);

    // Can access public fields
    println!("Width: {}", rect.width);
    println!("Area: {}", rect.area());

    // Cannot access private field
    // println!("Serial: {}", rect.serial);  // Error
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Struct fields are **private by default**, even if the struct is `pub`. Each field needs explicit `pub`. Private fields require a constructor.

Read more in <a href="https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public" target="_blank">TRPL - Making Structs and Enums Public</a>.



<!--
###############################################################################
## Modules - Re-exports
###############################################################################
-->
Question : Beginner - Modules & Visibility - How do you re-export items with pub use?
Answer   :

```rust
mod internal {
    pub mod deep {
        pub mod nested {
            pub fn important_function() {
                println!("Called important_function!");
            }

            pub struct ImportantStruct {
                pub value: i32,
            }
        }
    }
}

mod api {
    // Re-export for a cleaner public API
    pub use crate::internal::deep::nested::important_function;
    pub use crate::internal::deep::nested::ImportantStruct;

    // Users don't need to know the internal structure
}

fn main() {
    // Long path (internal structure exposed)
    internal::deep::nested::important_function();

    // Short path (via re-export)
    api::important_function();

    let item = api::ImportantStruct { value: 42 };
    println!("Value: {}", item.value);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`pub use` re-exports items, creating a cleaner public API. Users can access items via the re-export path without knowing internal module structure.

Read more in <a href="https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#re-exporting-names-with-pub-use" target="_blank">TRPL - Re-exporting Names with pub use</a>.



<!--
###############################################################################
## Modules - File Structure
###############################################################################
-->
Question : Beginner - Modules & Visibility - How do modules map to files and folders?
Answer   :

```rust
// In a real project, modules map to files:
//
// src/
// ├── main.rs          (or lib.rs)
// ├── garden.rs        (mod garden)
// └── garden/
//     └── vegetables.rs (mod vegetables inside garden)
//
// main.rs:
//   mod garden;
//
// garden.rs:
//   pub mod vegetables;
//   pub fn describe() { ... }
//
// garden/vegetables.rs:
//   pub fn grow() { ... }

// For this example, inline simulation:
mod garden {
    pub mod vegetables {
        pub fn grow() {
            println!("Growing vegetables!");
        }
    }

    pub fn describe() {
        println!("This is a garden");
    }
}

fn main() {
    garden::describe();
    garden::vegetables::grow();
}

// Modern style (Rust 2018+):
// - garden.rs for garden module
// - garden/vegetables.rs for submodule
//
// Old style (still works):
// - garden/mod.rs for garden module
// - garden/vegetables.rs for submodule
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Modules map to files:
- `mod foo;` → looks for `foo.rs` or `foo/mod.rs`
- Submodules go in a folder: `foo/bar.rs`

Modern style prefers `foo.rs` + `foo/` folder over `foo/mod.rs`.

Read more in <a href="https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html" target="_blank">TRPL - Separating Modules into Different Files</a>.