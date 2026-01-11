<!--
###############################################################################
## Vec - Creation
###############################################################################
-->
Question : Beginner - Collections - How do you create a Vec?
Answer   :

```rust
fn main() {
    // Empty vector with type annotation
    let v1: Vec<i32> = Vec::new();

    // With initial values using vec! macro
    let v2 = vec![1, 2, 3, 4, 5];

    // With capacity (pre-allocates memory)
    let mut v3: Vec<i32> = Vec::with_capacity(10);
    v3.push(1);

    // From an iterator
    let v4: Vec<i32> = (1..=5).collect();

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v3: {:?}, capacity: {}", v3, v3.capacity());
    println!("v4: {:?}", v4);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`Vec<T>` is a growable array. Create with `Vec::new()`, `vec![]` macro, `Vec::with_capacity()`, or `.collect()` from an iterator.

Read more in <a href="https://doc.rust-lang.org/book/ch08-01-vectors.html" target="_blank">TRPL - Storing Lists of Values with Vectors</a>.



<!--
###############################################################################
## Vec - Adding and Removing
###############################################################################
-->
Question : Beginner - Collections - How do you add and remove elements from a Vec?
Answer   :

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    // Add elements
    v.push(4);              // Add to end
    v.insert(0, 0);         // Insert at index
    println!("After push/insert: {:?}", v);

    // Remove elements
    let last = v.pop();     // Remove from end, returns Option
    println!("Popped: {:?}, vec: {:?}", last, v);

    let second = v.remove(1);  // Remove at index, returns value
    println!("Removed: {}, vec: {:?}", second, v);

    // Extend with multiple elements
    v.extend([10, 20, 30]);
    println!("After extend: {:?}", v);

    // Clear all
    v.clear();
    println!("After clear: {:?}, len: {}", v, v.len());
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

| Method | Description |
|--------|-------------|
| `push(val)` | Add to end |
| `pop()` | Remove from end, returns `Option<T>` |
| `insert(idx, val)` | Insert at index |
| `remove(idx)` | Remove at index, returns value |
| `extend(iter)` | Add multiple elements |
| `clear()` | Remove all elements |

Read more in <a href="https://doc.rust-lang.org/book/ch08-01-vectors.html#updating-a-vector" target="_blank">TRPL - Updating a Vector</a>.



<!--
###############################################################################
## Vec - Accessing Elements
###############################################################################
-->
Question : Beginner - Collections - How do you access elements in a Vec?
Answer   :

```rust
fn main() {
    let v = vec![10, 20, 30, 40, 50];

    // Indexing (panics if out of bounds)
    let third = v[2];
    println!("Third element: {}", third);

    // Safe access with get() (returns Option)
    match v.get(2) {
        Some(val) => println!("Got: {}", val),
        None => println!("No element at that index"),
    }

    // Out of bounds
    // let bad = v[100];  // Panics!
    let safe = v.get(100);  // Returns None
    println!("Out of bounds: {:?}", safe);

    // First and last
    println!("First: {:?}", v.first());
    println!("Last: {:?}", v.last());

    // Length and emptiness
    println!("Length: {}, is_empty: {}", v.len(), v.is_empty());
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Use `[]` for direct access (panics on invalid index) or `.get()` for safe access (returns `Option<&T>`). Use `.first()` and `.last()` for ends.

Read more in <a href="https://doc.rust-lang.org/book/ch08-01-vectors.html#reading-elements-of-vectors" target="_blank">TRPL - Reading Elements of Vectors</a>.



<!--
###############################################################################
## Vec - Iterating
###############################################################################
-->
Question : Beginner - Collections - How do you iterate over a Vec?
Answer   :

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // Immutable iteration (borrows)
    for val in &v {
        println!("Value: {}", val);
    }
    println!("Vec still usable: {:?}", v);

    // Mutable iteration
    for val in &mut v {
        *val *= 2;  // Dereference to modify
    }
    println!("Doubled: {:?}", v);

    // Consuming iteration (takes ownership)
    let v2 = vec![10, 20, 30];
    for val in v2 {
        println!("Owned: {}", val);
    }
    // println!("{:?}", v2);  // Error: v2 was moved

    // With index
    for (i, val) in v.iter().enumerate() {
        println!("Index {}: {}", i, val);
    }
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

| Loop | Equivalent | Ownership |
|------|------------|-----------|
| `for x in &v` | `v.iter()` | Borrows |
| `for x in &mut v` | `v.iter_mut()` | Mutably borrows |
| `for x in v` | `v.into_iter()` | Consumes |

Read more in <a href="https://doc.rust-lang.org/book/ch08-01-vectors.html#iterating-over-the-values-in-a-vector" target="_blank">TRPL - Iterating Over a Vector</a>.



<!--
###############################################################################
## String - Creation
###############################################################################
-->
Question : Beginner - Collections - How do you create and build a String?
Answer   :

```rust
fn main() {
    // Empty string
    let mut s1 = String::new();

    // From string literal
    let s2 = String::from("hello");
    let s3 = "hello".to_string();

    // Build incrementally
    s1.push_str("Hello");   // Append string slice
    s1.push(' ');           // Append single char
    s1.push_str("world!");
    println!("Built: {}", s1);

    // With capacity
    let mut s4 = String::with_capacity(20);
    s4.push_str("pre-allocated");
    println!("s4: {}, capacity: {}", s4, s4.capacity());

    // From format! macro
    let name = "Rust";
    let s5 = format!("Hello, {}!", name);
    println!("{}", s5);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`String` is a growable, heap-allocated UTF-8 string. Create with `String::new()`, `String::from()`, `.to_string()`, or `format!()`.

Read more in <a href="https://doc.rust-lang.org/book/ch08-02-strings.html" target="_blank">TRPL - Storing UTF-8 Encoded Text with Strings</a>.



<!--
###############################################################################
## String - Concatenation
###############################################################################
-->
Question : Beginner - Collections - How do you concatenate Strings?
Answer   :

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from(" world");

    // Using + operator (takes ownership of s1)
    let s3 = s1 + &s2;  // s1 moved, s2 borrowed
    // println!("{}", s1);  // Error: s1 was moved
    println!("s3: {}", s3);

    // Using format! (doesn't take ownership)
    let a = String::from("tic");
    let b = String::from("tac");
    let c = String::from("toe");
    let s4 = format!("{}-{}-{}", a, b, c);
    println!("s4: {}", s4);
    println!("a still valid: {}", a);  // a, b, c still usable

    // Using push_str
    let mut s5 = String::from("foo");
    s5.push_str("bar");
    println!("s5: {}", s5);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

| Method | Ownership |
|--------|-----------|
| `s1 + &s2` | `s1` moved, `s2` borrowed |
| `format!(...)` | Nothing moved |
| `s.push_str(...)` | Mutates in place |

Read more in <a href="https://doc.rust-lang.org/book/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro" target="_blank">TRPL - Concatenation</a>.



<!--
###############################################################################
## String - Characters and Bytes
###############################################################################
-->
Question : Beginner - Collections - How do you access characters in a String?
Answer   :

```rust
fn main() {
    let s = String::from("héllo 🦀");

    // Cannot index directly: s[0] won't compile!
    // Strings are UTF-8, characters vary in byte length

    // Iterate over chars (Unicode scalar values)
    print!("Chars: ");
    for c in s.chars() {
        print!("{} ", c);
    }
    println!();

    // Iterate over bytes
    print!("Bytes: ");
    for b in s.bytes() {
        print!("{} ", b);
    }
    println!();

    // Get nth char (inefficient, but works)
    let third_char = s.chars().nth(2);
    println!("Third char: {:?}", third_char);

    // Slice by byte indices (must be valid UTF-8 boundaries)
    let hello = &s[0..6];  // "héllo" - careful with byte indices!
    println!("Slice: {}", hello);

    println!("Length in bytes: {}", s.len());
    println!("Length in chars: {}", s.chars().count());
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Strings are UTF-8. You cannot index with `s[0]`. Use `.chars()` for characters, `.bytes()` for raw bytes. String slices must be valid UTF-8 boundaries.

Read more in <a href="https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings" target="_blank">TRPL - Indexing into Strings</a>.



<!--
###############################################################################
## HashMap - Creation and Insertion
###############################################################################
-->
Question : Beginner - Collections - How do you create and populate a HashMap?
Answer   :

```rust
use std::collections::HashMap;

fn main() {
    // Create empty HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Insert key-value pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    // From arrays of tuples
    let teams = vec![
        (String::from("Green"), 30),
        (String::from("Yellow"), 40),
    ];
    let scores2: HashMap<_, _> = teams.into_iter().collect();

    println!("scores: {:?}", scores);
    println!("scores2: {:?}", scores2);

    // Insert only if key doesn't exist
    scores.entry(String::from("Blue")).or_insert(999);  // Won't change
    scores.entry(String::from("White")).or_insert(25);  // Will insert
    println!("After entry: {:?}", scores);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

`HashMap<K, V>` stores key-value pairs. Must `use std::collections::HashMap`. Use `.insert()` to add, `.entry().or_insert()` for conditional insertion.

Read more in <a href="https://doc.rust-lang.org/book/ch08-03-hash-maps.html" target="_blank">TRPL - Storing Keys with Associated Values in Hash Maps</a>.



<!--
###############################################################################
## HashMap - Accessing and Updating
###############################################################################
-->
Question : Beginner - Collections - How do you access and update HashMap values?
Answer   :

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    // Access with get() - returns Option<&V>
    let team = String::from("Blue");
    let score = scores.get(&team);
    println!("Blue score: {:?}", score);

    // Copied value (for Copy types)
    let score_val = scores.get(&team).copied().unwrap_or(0);
    println!("Blue score value: {}", score_val);

    // Overwrite existing value
    scores.insert(String::from("Blue"), 25);
    println!("After overwrite: {:?}", scores);

    // Update based on old value
    let text = "hello world hello rust hello";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word counts: {:?}", word_count);
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

Use `.get(&key)` to access (returns `Option<&V>`). Use `.insert()` to overwrite. The `.entry().or_insert()` pattern is great for counting and updating.

Read more in <a href="https://doc.rust-lang.org/book/ch08-03-hash-maps.html#accessing-values-in-a-hash-map" target="_blank">TRPL - Accessing Values in a Hash Map</a>.



<!--
###############################################################################
## HashMap - Iterating
###############################################################################
-->
Question : Beginner - Collections - How do you iterate over a HashMap?
Answer   :

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    // Iterate over key-value pairs
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // Iterate over keys only
    print!("Keys: ");
    for key in map.keys() {
        print!("{} ", key);
    }
    println!();

    // Iterate over values only
    print!("Values: ");
    for value in map.values() {
        print!("{} ", value);
    }
    println!();

    // Mutable iteration over values
    for value in map.values_mut() {
        *value *= 10;
    }
    println!("After mutation: {:?}", map);

    // Check existence
    println!("Contains 'a'? {}", map.contains_key("a"));
    println!("Contains 'z'? {}", map.contains_key("z"));
}
```
---
Copy, paste and run the code above in <a href="https://play.rust-lang.org/" target="_blank">Rust Playground</a>.

| Method | Returns |
|--------|---------|
| `for (k, v) in &map` | Key-value pairs |
| `.keys()` | Iterator over keys |
| `.values()` | Iterator over values |
| `.values_mut()` | Mutable iterator over values |

Read more in <a href="https://doc.rust-lang.org/book/ch08-03-hash-maps.html#iterating-over-a-hash-map" target="_blank">TRPL - Iterating Over a Hash Map</a>.