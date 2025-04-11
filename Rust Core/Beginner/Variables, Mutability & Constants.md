# Variables, Mutability, Constants & Type Annotations

## Variables

-   `let` is used to declare a variable.
-   `let` is immutable by default.

```rust
let x = 5;
```

### Variable Shadowing

-   You can declare a new variable with the same name as a previous variable
-   This is called "shadowing" and allows you to reuse variable names

```rust
let x = 5;
let x = x + 1; // x is now 6
let x = x * 2; // x is now 12
```

-   Shadowing lets you change the type of a variable while reusing the same name:

```rust
let spaces = "   ";
let spaces = spaces.len(); // spaces is now a number (3)
```

## Mutability

-   `mut` is used to make a variable mutable.

```rust
let mut x = 5;
```

### Use Cases for Mutability

-   When implementing algorithms that require in-place updates
-   When you need to improve performance by avoiding copying large data structures
-   When working with user input that changes over time
-   For counters and accumulators in loops

```rust
let mut counter = 0;
while counter < 5 {
println!("Counter: {}", counter);
counter += 1;
}
```

## Constants

-   `const` is used to declare a constant.
-   Constants are always immutable (no mut keyword allowed).
-   Constants must be annotated with their type.
-   Constants can be declared in any scope, including the global scope
-   Constants can only be set to a constant expression, not the result of a function call or any value computed at runtime

```rust
const PI: f64 = 3.14159;
const MAX_USERS: u32 = 100_000;
```

### Use Cases for Constants

-   When you need a value that is known at compile time and unlikely to change.
-   When you want to ensure that a value is not changed accidentally.
-   Constants are valid for the entire program runtime within their scope. They're useful for values that multiple parts of your program might need to know about.
-   For configuration values that should be used throughout your program
-   For mathematical constants
-   For defining limits, boundaries, or thresholds
-   For error codes or status codes

```rust
const MAX_CONNECTIONS: u32 = 10_000;
const DATABASE_URL: &str = "postgres://localhost:5432/myapp";
const HTTP_OK: u16 = 200;
const HTTP_NOT_FOUND: u16 = 404;
```

## Type Annotations

-   You can explicitly specify a variable's type using a colon followed by the type:

```rust
let age: u32 = 30;
let pi: f64 = 3.14159;
let is_active: bool = true;
let name: &str = "Alice";
```

# Differences between Variables and Constants

| Variable                                             | Constant                                                    |
| ---------------------------------------------------- | ----------------------------------------------------------- |
| Immutable by default, can be made mutable with `mut` | Always immutable                                            |
| No type annotation required                          | Must be annotated with type                                 |
| Can be shadowed                                      | Cannot be shadowed                                          |
| Can be assigned values computed at runtime           | Can only be assigned constant expressions                   |
| Scoped to the block where they're declared           | Available for the entire program runtime within their scope |
| Can be used with any type                            | Can be used with any type                                   |

## Best Practices

-   Use immutable variables by default
-   Only use `mut` when you need to change a variable's value
-   Use constants for values that are used throughout your program
-   Use shadowing to transform a value while keeping the same name
-   Use descriptive names for variables and constants
