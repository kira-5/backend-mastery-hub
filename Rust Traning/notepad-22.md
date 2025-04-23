RUST_BACKTRACE=1 cargo run : For more information about error.

1. Figure out wheter varaible should be stored in stack and heap.

String:

1. To concat a string, make left variable as a heap allocated variable.

```rust
let str1: &str = "John";
let str2: String = String::from("John");
let str3 = str1.to_string() + &str2;
```

Why left variable should be string?

-   It is coxc of operator overloading.

Ownership:

1. Instead of transfer ownership, we can borrow the variable.

Borrowing:

1. 1 mutable borrow and n number of immutable borrow is allowed.

```rust
let str7 = String::from("Borrowing");
// immutable borrow
let str8 = &str7;
let str9 = &str7;
println!("{}", str8);
println!("{}", str9);
```

```rust
let str10 = String::from("Borrowing");
// mutable borrow
let str11 = &mut str10;
println!("{}", str11);
```

```rust
let str12 = String::from("Borrowing");
// mutable borrow
let str13 = &mut str12;
let str14 = &mut str12; // Will throw error as only 1 mutable borrow is allowed.
println!("{}", str13);
```

```rust
let str12 = String::from("Borrowing");

// mutable borrow
let str13 = &mut str12;

// immutable borrow
let str14 = &str12; // Will throw error as either mutable or immutable borrow is allowed.
println!("{}", str13);
```

Benefits of borrowing:

1. Use same memory instead of creating new one.

Type Casting:

1. Have to explicity type Cast
2. char to int and int to char is possible.

Rust have no null value.

1. Instead of null, it has Option enum.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

```rust
let x: Option<i32> = None;
```

unwrap():

1. Don't use in production.
2. Uf u have to use, use only if u r sure there is value in variable.
3. Instead of unwrap(), use match pattern.

```rust
let x: Option<i32> = None;
println!("{}", x.unwrap());
```

unwrap() will return value if it is Some.

```rust
let x: Option<i32> = Some(10);
println!("{}", x.unwrap());
```

Match Pattern:

1. Match pattern is used to match value of variable.

```rust
let x: Option<i32> = Some(10);
match x {
    Some(y) => println!("{}", y),
    None => println!("No value"),
}
```

Control Flow:

1. if, else if, else
    - If else can be written as expression.

```rust
let x: i32 = 10;
if x > 0 {
    println!("x is greater than 0");
} else if x < 0 {
    println!("x is less than 0");
} else {
    println!("x is 0");
}
```

````rust
let x: i32 = 10;
let y = if x > 0 {
    "x is greater than 0"
} else {
    "x is less than 0"
}

2. if let some - Important

```rust
let x: Option<i32> = Some(10);
if let Some(y) = x {
    println!("{}", y);
}
````

3. enums

-   enums stored in stack.

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let direction = Direction::Up;

match direction {
    Direction::Up => println!("Up"),
    Direction::Down => println!("Down"),
    Direction::Left => println!("Left"),
    Direction::Right => println!("Right"),
}


```

```rust
enum Direction {
    Up(x: i32, y: i32), // struct
    Down(x: i32, y: i32), // struct
    Left(String), // unit struct
    Right(u8, u8, u8), // Tuple
}

let direction = Direction::Up(10, 20);

match direction {
    Direction::Up(x, y) => println!("Up: {}, {}", x, y),
    Direction::Down(x, y) => println!("Down: {}, {}", x, y),
    Direction::Left(s) => println!("Left: {}", s),
    Direction::Right(r, g, b) => println!("Right: {}, {}, {}", r, g, b),
}

```

4. match pattern
    - Don't need to use let kwyword in match pattern to create a varaible.

```rust

let day: u8 = 1;

match day {
    1 => {
        println!("Sunday");
    }
    2 => {
        println!("Monday");
    }
    3 => {
        println!("Tuesday");
    }
    4 => {
        println!("Wednesday");
    }
    5 => {
        println!("Thursday");
    }
    6 => {
        println!("Friday");
    }
    7 => {
        println!("Saturday");
    }
    _ => {
        println!("Invalid day number");
    }
}

```

```rust
let num: u8 = 1;

match num {
    n1 if n1 % 2 == 0 => println!("Even"),
    n2 if n2 % 2 != 0 => println!("Odd"),
    _ => {}

}
```
