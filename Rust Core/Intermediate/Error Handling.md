# Error Handling

---

-   Error handling is primarily done through the `Result` and `Option` types.
-   This allows for safe and explicit handling of errors without the need for exceptions.

## Result

-   `Result` is an enum that has two variants: `Ok` and `Err`.
-   `Ok` is the success variant and contains the return value.
-   `Err` is the error variant and contains the error message.

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

## Option

-   `Option` is an enum that has two variants: `Some` and `None`.
-   `Some` is the success variant and contains the return value.
-   `None` is the error variant and contains the error message.

```rust
fn get_first_element(vec: &Vec<i32>) -> Option<i32> {
    vec.first().map(|x| *x)
}   
```

## ? Operator

-   The `?` operator is used to handle errors.
-   It returns the error if the result is an error.
-   It returns the value if the result is a success.

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    let result = a / b;
    Ok(result)
}

fn main() {
    let result = divide(10, 2);
    println!("{:?}", result);
}
```

## match

-   `match` is used to handle errors.
-   It returns the error if the result is an error.
-   It returns the value if the result is a success.

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    let result = a / b;
    Ok(result)
}

fn main() {
    let result = divide(10, 2);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}