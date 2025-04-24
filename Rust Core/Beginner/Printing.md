# Printing

Rust provides several macros for printing output to the console. The most commonly used ones are `println!`, `print!`, `eprintln!`, and `eprint!`.

In Rust, there are differences between using `{:?}` and `{}` in `println!` for printing a string literal (or any value):

### 1\. **`println!("{}", string_literal);`**

*   This uses the **`Display`** trait (`std::fmt::Display`).
    
*   It is meant for **user-friendly, human-readable output**.
    
*   Works well for types like `String`, `&str`, numbers, etc.
    
*   Example:
    
    ```
    let string_literal = "Hello, world!";
    println!("{}", string_literal); // Output: Hello, world!
    ```
    

### 2\. **`println!("{:?}", string_literal);`**

*   This uses the **`Debug`** trait (`std::fmt::Debug`).
    
*   It is meant for **debugging and development**, showing a more technical representation.
    
*   For strings, it adds **quotes** around the output.
    
*   Example:
    
    ```
    let string_literal = "Hello, world!";
    println!("{:?}", string_literal); // Output: "Hello, world!"
    ```
    

### Key Differences:

| Format | Trait Used | Purpose | Output Example (`"Hello"`) |
| --- | --- | --- | --- |
| `{}` | `Display` | User-friendly | `Hello` |
| `{:?}` | `Debug` | Debugging | `"Hello"` |

### When to Use Which?

*   Use `{}` when you want clean output (e.g., for end users).
    
*   Use `{:?}` when debugging (e.g., to inspect values, including strings with quotes).
    

### Bonus: `{:#?}` (Pretty Debug)

For complex types, `{:#?}` formats the output in a more readable way (with indentation):

```
println!("{:#?}", some_struct); // Pretty-printed debug output
```

Would you like an example with a custom type implementing both traits? 😊

## Response:
Here's a structured markdown (`.md`) file with notes on printing in Rust:

````
# Printing in Rust

Rust provides several macros for printing output to the console. The most commonly used ones are `println!`, `print!`, `eprintln!`, and `eprint!`.

## 1. Basic Printing Macros

### `print!` and `println!`
- **`print!`**: Prints text to stdout (no newline).
- **`println!`**: Prints text to stdout with a newline.

```rust
print!("Hello, "); // No newline
println!("world!"); // Adds newline
````

### `eprint!` and `eprintln!`

*   **`eprint!`**: Prints text to stderr (no newline).
    
*   **`eprintln!`**: Prints text to stderr with a newline.
    

```
eprintln!("Error: Something went wrong!"); // Debugging/errors
```

* * *

2\. Formatting Output
---------------------

### Placeholder Syntax

*   `{}` → Uses the `Display` trait (human-readable).
    
*   `{:?}` → Uses the `Debug` trait (debugging output).
    
*   `{:#?}` → Pretty-printed debug output.
    

```
let name = "Alice";
println!("Hello, {}!", name); // Hello, Alice!
println!("Debug: {:?}", name); // Debug: "Alice"
```

### Formatting Numeric Values

```
let num = 42;
println!("Decimal: {}", num); // 42
println!("Hex: {:x}", num);  // 2a
println!("Binary: {:b}", num); // 101010
```

### Positional and Named Arguments

```
println!("{0} loves {1} and {1} loves {0}", "Alice", "Bob");
println!("{name} is {age} years old", name = "Dave", age = 30);
```

* * *

3\. Advanced Formatting
-----------------------

### Padding and Alignment

```
println!("{:>10}", "right");  // "     right"
println!("{:^10}", "center"); // "  center  "
println!("{:<10}", "left");   // "left      "
```

### Precision for Floating Points

```
let pi = 3.141592;
println!("{:.2}", pi); // "3.14" (2 decimal places)
```

### Custom Debug Formatting

```
#[derive(Debug)]
struct Point { x: i32, y: i32 }

let p = Point { x: 10, y: 20 };
println!("Debug: {:?}", p); // Debug: Point { x: 10, y: 20 }
println!("Pretty Debug: {:#?}", p);
/*
Pretty Debug: Point {
    x: 10,
    y: 20,
}
*/
```

* * *

4\. Common Traits for Printing
------------------------------

| Trait (`std::fmt`) | Format Specifier | Purpose |
| --- | --- | --- |
| `Display` | `{}` | User-friendly output |
| `Debug` | `{:?}` | Debugging output |
| `Octal` | `{:o}` | Octal representation |
| `LowerHex` | `{:x}` | Lowercase hex |
| `UpperHex` | `{:X}` | Uppercase hex |
| `Binary` | `{:b}` | Binary representation |

* * *

5\. Implementing `Display` for Custom Types
-------------------------------------------

```
use std::fmt;

struct Person {
    name: String,
    age: u8,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({} years old)", self.name, self.age)
    }
}

fn main() {
    let person = Person { name: String::from("Alice"), age: 25 };
    println!("{}", person); // "Alice (25 years old)"
}
```

* * *

Summary
-------

*   Use `println!` for stdout, `eprintln!` for stderr.
    
*   `{}` → `Display` (clean output), `{:?}` → `Debug` (debugging).
    
*   Supports advanced formatting (padding, precision, hex/binary).
    
*   Custom types can implement `Display` or `Debug` for printing.
   