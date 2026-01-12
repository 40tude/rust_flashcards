<!--
###############################################################################
## Cargo - Basic Commands
###############################################################################
-->
Question : Beginner - Cargo & Tooling - How do you setup your Rust dev environment?
Answer   :

Here I suppose Windows 11, VSCode and Git are correctly installed.

```powershell
cd $env:USERPROFILE/downloads

# Download the official Rust installer
Invoke-WebRequest -Uri https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe -OutFile rustup-init.exe

# Run the Rust installer
./rustup-init.exe

rustc --version # Check Rust compiler version
rustup update   # Later if a new version is available (every 6 weeks)

```

```powershell
code --install-extension rust-lang.rust-analyzer  # rust analyzer
code --install-extension vadimcn.vscode-lldb      # CodeLLDB for debug
code --install-extension fill-labs.dependi        # Optional: Indicates if project's dependencies are up to date (or not)
code --install-extension tamasfe.even-better-toml # Optional: Nicer `.toml` files

code --list-extensions # check
```

Read this <a href="https://www.40tude.fr/docs/06_programmation/rust/005_my_rust_setup_win11/my_rust_setup_win11.html" target="_blank">post </a> on <a href="https://www.40tude.fr/docs/06_programmation/rust/" target="_blank">40tude.fr</a>.





<!--
###############################################################################
## Cargo - Basic Commands
###############################################################################
-->
Question : Beginner - Cargo & Tooling - What are the essential Cargo commands?
Answer   :

```powershell
# Create a new project
cargo new my_project        # Binary (default)
cargo new my_lib --lib      # Library

# Build
cargo build                 # Debug build (target/debug/)
cargo build --release       # Optimized build (target/release/)

# Run
cargo run                   # Build and run
cargo run --release         # Build optimized and run

# Check (faster than build, no executable)
cargo check                 # Verify code compiles

# Test
cargo test                  # Run all tests

# Clean
cargo clean                 # Remove target/ directory
```

```rust
// Minimal main.rs to verify cargo works
fn main() {
    println!("Cargo commands work!");
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

| Command | Purpose |
|---------|---------|
| `cargo new` | Create project |
| `cargo build` | Compile |
| `cargo run` | Compile and execute |
| `cargo check` | Fast syntax check |
| `cargo test` | Run tests |

Read more in <a href="https://doc.rust-lang.org/book/ch01-03-hello-cargo.html" target="_blank">TRPL - Hello, Cargo!</a>.



<!--
###############################################################################
## Cargo - Cargo.toml
###############################################################################
-->
Question : Beginner - Cargo & Tooling - What is Cargo.toml and what are its main sections?
Answer   :

```toml
# Cargo.toml - Package manifest

[package]
name = "my_project"
version = "0.1.0"
edition = "2024"           # Rust edition (2015, 2018, 2021, 2024)
authors = ["Obi-Wan Kenobi <you@example.com>"]
description = "A short description"
license = "MIT"

[dependencies]
serde = "1.0"              # From crates.io
rand = { version = "0.8", features = ["small_rng"] }

[dev-dependencies]         # Only for tests/examples
pretty_assertions = "1.0"

[build-dependencies]       # For build scripts
cc = "1.0"

[[bin]]                    # Multiple binaries
name = "other_binary"
path = "src/bin/other.rs"
```

```rust
fn main() {
    println!("Check Cargo.toml for project configuration!");
}
```
---
<!-- Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>. -->

`Cargo.toml` is the manifest file. Key sections:
- `[package]`: metadata
- `[dependencies]`: runtime dependencies
- `[dev-dependencies]`: test-only dependencies

Read more in <a href="https://doc.rust-lang.org/cargo/reference/manifest.html" target="_blank">Cargo Book - The Manifest Format</a>.



<!--
###############################################################################
## Cargo - Dependencies
###############################################################################
-->
Question : Beginner - Cargo & Tooling - How do you add and manage dependencies?
Answer   :

```toml
# In Cargo.toml

[dependencies]
# Simple version (from crates.io)
serde = "1.0"

# With features
serde = { version = "1.0", features = ["derive"] }

# Specific version constraints
rand = "0.8.5"         # Exactly compatible with 0.8.5
rand = ">=0.8, <0.9"   # Range
rand = "~0.8.5"        # Allows 0.8.x but not 0.9

# From git
my_crate = { git = "https://github.com/user/repo" }
my_crate = { git = "https://github.com/user/repo", branch = "main" }

# Local path
my_local = { path = "../my_local_crate" }
```

```rust
// After adding to Cargo.toml, use in code:
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // cargo add serde --features derive
    println!("Dependencies are managed in Cargo.toml");
}
```
---
<!-- Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>. -->
<!-- Add dependencies with `cargo add crate_name` or edit `Cargo.toml`. Use `Cargo.lock` for reproducible builds (commit it for binaries). -->

Read more in <a href="https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html" target="_blank">Cargo Book - Specifying Dependencies</a>.



<!--
###############################################################################
## Cargo - rustfmt
###############################################################################
-->
Question : Beginner - Cargo & Tooling - How do you format Rust code with rustfmt?
Answer   :

**In the console:**

```powershell
# Format entire project
cargo fmt

# Check formatting without changing files
cargo fmt -- --check

# Format specific file
rustfmt src/main.rs
```


**Before rustfmt:**
```rust
fn main(){let x=5;let y=10;if x<y{println!("x is less");}else{println!("x is greater or equal");}}
```

**After rustfmt:**
```rust
fn main() {
    let x = 5;
    let y = 10;
    if x < y {
        println!("x is less");
    } else {
        println!("x is greater or equal");
    }
}
```
---
Copy, paste and run the "before" code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

* `cargo fmt` is available on the right hand side in the `TOOLS` menu. Formats code according to Rust style guidelines.
* In your local project, configure with `rustfmt.toml` in project root.
* Read this [page](https://rust-lang.github.io/rustfmt/?version=main&search=)
* Use `--check` in CI to verify formatting.

Read more in <a href="https://github.com/rust-lang/rustfmt" target="_blank">rustfmt GitHub</a>.



<!--
###############################################################################
## Cargo - Clippy
###############################################################################
-->
Question : Beginner - Cargo & Tooling - What is Clippy and how do you use it?
Answer   :

```powershell
# Run clippy
cargo clippy

# Treat warnings as errors (useful for CI)
cargo clippy -- -D warnings

# Fix automatically where possible
cargo clippy --fix
```

```rust
// Clippy catches common mistakes and suggests improvements

fn main() {
    // Clippy warning: using `if let` with `Some` on `Option`
    let opt = Some(5);

    // Bad (clippy will warn)
    if opt.is_some() {
        println!("{}", opt.unwrap());
    }

    // Good (clippy suggestion)
    if let Some(val) = opt {
        println!("{}", val);
    }

    // Clippy catches: redundant clones, inefficient patterns,
    // possible bugs, style issues, and much more

    let v = vec![1, 2, 3];

    // Bad: manual iteration to find length
    let mut count = 0;
    for _ in &v {
        count += 1;
    }

    // Good: use len()
    let count = v.len();
    println!("Count: {}", count);
}
```
---

Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.
* `clippy` is available on the right hand side, in the `TOOLS` menu.

**Clippy** is a linter with hundreds of checks for correctness, performance, style, and more. Run it regularly!

Read more in <a href="https://doc.rust-lang.org/clippy/" target="_blank">Clippy Documentation</a>.



<!--
###############################################################################
## Cargo - Documentation
###############################################################################
-->
Question : Beginner - Cargo & Tooling - How do you write and generate documentation?
Answer   :

```rust
//! Crate-level documentation (at top of lib.rs)
//!
//! This crate provides math utilities.

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// This function doesn't panic.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// A point in 2D space.
pub struct Point {
    /// The x coordinate
    pub x: f64,
    /// The y coordinate
    pub y: f64,
}

fn main() {
    // This code demonstrates doc comments syntax
    // Doc tests only run with `cargo test` in a real project

    let result = add(2, 3);
    assert_eq!(result, 5);
    println!("2 + 3 = {}", result);

    let p = Point { x: 1.0, y: 2.0 };
    println!("Point: ({}, {})", p.x, p.y);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

**Note:** Doc tests only work in a real Cargo project, not in Rust Playground.

Use `///` for item docs, `//!` for module/crate docs. Code in doc comments runs as tests! Common sections: Examples, Panics, Errors, Safety.

Read more in <a href="https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments" target="_blank">TRPL - Documentation Comments</a>.



<!--
###############################################################################
## Cargo - Useful Commands
###############################################################################
-->
Question : Beginner - Cargo & Tooling - What are other useful Cargo commands?
Answer   :

```powershell
# Dependency management
cargo update              # Update dependencies (within semver)
cargo tree                # Show dependency tree
cargo add serde           # Add dependency (cargo-edit)
cargo remove serde        # Remove dependency

# Information
cargo --version           # Cargo version
cargo search serde        # Search crates.io
cargo info serde          # Show crate info

# Development
cargo watch -x run        # Auto-rebuild on changes (cargo-watch)
cargo bench               # Run benchmarks
cargo publish             # Publish to crates.io

# Toolchain management (rustup)
rustup update             # Update Rust
rustup show               # Show installed toolchains
rustc --version           # Compiler version
```

```rust
fn main() {
    // Print versions
    println!("Check versions with:");
    println!("  cargo --version");
    println!("  rustc --version");
    println!("  rustup show");
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

| Command            | Purpose |
|--------------------|---------|
| `cargo tree`       | Visualize dependencies |
| `cargo update`     | Update Cargo.lock |
| `cargo add/remove` | Manage dependencies |
| `cargo bench`      | Run benchmarks |
| `rustup update`    | Update Rust toolchain |

Read more in <a href="https://doc.rust-lang.org/cargo/commands/index.html" target="_blank">Cargo Commands</a>.