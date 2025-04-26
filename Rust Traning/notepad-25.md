# Rust Traning 25 April 2025

## Traits

### Drop Trait

#### Features of Drop Trait

- The Drop trait allows you to customize what happens when a value goes out of scope.
- This is similar to destructors in other languages.
- Automatic Calling: The drop method is called automatically when the value goes out of scope.
- No Manual Calling: You cannot call drop directly. Instead, Rust calls it for you.
- Early Dropping: If you need to drop a value early, use std::mem::drop:
- Order of Dropping: Values are dropped in the reverse order of their declaration.

#### Example of Early Dropping

```rust
  let x = MyStruct { name: String::from("early") };
  drop(x); // Explicitly drops x here
  // x is no longer valid after this point
```

#### Basic usage of Drop Trait

```rust
  struct MyStruct {
      name: String,
  }

  impl Drop for MyStruct {
      fn drop(&mut self) {
          println!("Dropping MyStruct with name: {}", self.name);
      }
  }

  fn main() {
      let x = MyStruct { name: String::from("example") };
      // When x goes out of scope here, Drop::drop() will be called automatically
  }
```

#### Drop with Resources

```rust
  struct FileWrapper {
      handle: Option<std::fs::File>,
  }

  impl Drop for FileWrapper {
      fn drop(&mut self) {
          if let Some(file) = self.handle.take() {
              // Perform cleanup like flushing buffers
              println!("Closing file");
              // File is dropped here automatically
          }
      }
  }
```

#### Limitations of Drop Trait

- You can only implement Drop directly on your types, not on trait objects.
- The drop method takes &mut self, not self, because the value is already being destroyed.
- You can't call drop manually to prevent double-free issues.


### Drop Trait


## Closures

- Closures in Rust are anonymous functions that can capture their environment. 
- They are similar to lambda functions in other languages.
- Closures can be used as arguments to functions and stored in variables.

```rust
  let add_one = |x| x + 1;
  println!("5 + 1 = {}", add_one(5)); // Output: 5 + 1 = 6
```

### Basic Closure

```rust
let add = |a: i32, b: i32| a + b;
let result = add(2, 3);
println!("Result: {}", result);
```

### Closure with Parameters
