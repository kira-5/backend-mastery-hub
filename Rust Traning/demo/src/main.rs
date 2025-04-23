use std::sync::atomic::{AtomicU32, Ordering};

// Global Variables
static COUNTER: AtomicU32 = AtomicU32::new(0);

fn main() {
    println!("\n---------------Numbers-----------------");

    // Integer Types
    println!("\n---Integer Types---");
    let i8_num: i8 = -128; // 8-bit signed integer (-128 to 127)
    let u8_num: u8 = 255; // 8-bit unsigned integer (0 to 255)
    let i16_num: i16 = -32768; // 16-bit signed integer
    let u16_num: u16 = 65535; // 16-bit unsigned integer
    let i32_num: i32 = -2147483648; // 32-bit signed integer (default)
    let u32_num: u32 = 4294967295; // 32-bit unsigned integer
    let i64_num: i64 = -9223372036854775808; // 64-bit signed integer
    let u64_num: u64 = 18446744073709551615; // 64-bit unsigned integer
    let isize_num: isize = -9223372036854775808; // Architecture-dependent signed
    let usize_num: usize = 18446744073709551615; // Architecture-dependent unsigned

    println!("i8: {} (size: {} bytes)", i8_num, std::mem::size_of::<i8>());
    println!("u8: {} (size: {} bytes)", u8_num, std::mem::size_of::<u8>());
    println!(
        "i16: {} (size: {} bytes)",
        i16_num,
        std::mem::size_of::<i16>()
    );
    println!(
        "u16: {} (size: {} bytes)",
        u16_num,
        std::mem::size_of::<u16>()
    );
    println!(
        "i32: {} (size: {} bytes)",
        i32_num,
        std::mem::size_of::<i32>()
    );
    println!(
        "u32: {} (size: {} bytes)",
        u32_num,
        std::mem::size_of::<u32>()
    );
    println!(
        "i64: {} (size: {} bytes)",
        i64_num,
        std::mem::size_of::<i64>()
    );
    println!(
        "u64: {} (size: {} bytes)",
        u64_num,
        std::mem::size_of::<u64>()
    );
    println!(
        "isize: {} (size: {} bytes)",
        isize_num,
        std::mem::size_of::<isize>()
    );
    println!(
        "usize: {} (size: {} bytes)",
        usize_num,
        std::mem::size_of::<usize>()
    );

    // Floating Point Types
    println!("\n---Floating Point Types---");
    let f32_num: f32 = 3.14159; // 32-bit floating point
    let f64_num: f64 = 3.141592653589793; // 64-bit floating point (default)

    println!(
        "f32: {} (size: {} bytes)",
        f32_num,
        std::mem::size_of::<f32>()
    );
    println!(
        "f64: {} (size: {} bytes)",
        f64_num,
        std::mem::size_of::<f64>()
    );

    // Number Operations
    println!("\n---Number Operations---");
    let a = 10;
    let b = 3;
    println!("Addition: {} + {} = {}", a, b, a + b);
    println!("Subtraction: {} - {} = {}", a, b, a - b);
    println!("Multiplication: {} * {} = {}", a, b, a * b);
    println!("Division: {} / {} = {}", a, b, a / b);
    println!("Remainder: {} % {} = {}", a, b, a % b);

    // Type Conversion
    println!("\n---Type Conversion---");
    let int_num = 42;
    let float_num = int_num as f64;
    println!("Integer to Float: {} -> {}", int_num, float_num);

    println!("\n---------------Strings-----------------");

    // String Literals (&str)
    println!("\n---String Literals (&str)---");
    let str_literal: &str = "Hello, Rust!";
    println!("Type: {}", std::any::type_name::<&str>());
    println!("Value: {}", str_literal);
    println!("Size: {} bytes", std::mem::size_of_val(str_literal));
    println!("Length: {} bytes", str_literal.len());
    println!("Chars count: {}", str_literal.chars().count());

    // String Slices
    println!("\n---String Slices---");
    let full_str = "Hello, World!";
    let slice = &full_str[0..5];
    println!("Full string: {}", full_str);
    println!("Slice [0..5]: {}", slice);

    // Owned Strings (String)
    println!("\n---Owned Strings (String)---");
    let mut owned_str = String::from("Hello");
    println!("Type: {}", std::any::type_name::<String>());
    println!("Initial value: {}", owned_str);
    println!("Size: {} bytes", std::mem::size_of_val(&owned_str));
    println!("Length: {} bytes", owned_str.len());

    // String Operations
    println!("\n---String Operations---");
    owned_str.push_str(", Rust!"); // Append
    println!("After push_str: {}", owned_str);

    owned_str.push('!'); // Append single char
    println!("After push: {}", owned_str);

    let concatenated = owned_str + " Welcome!";
    println!("After concatenation: {}", concatenated);

    // UTF-8 Strings
    println!("\n---UTF-8 Strings---");
    let chinese = "你好，世界！";
    let emoji = "Hello 👋 World 🌍";
    println!("Chinese: {}", chinese);
    println!("Chinese length in bytes: {}", chinese.len());
    println!("Chinese chars count: {}", chinese.chars().count());
    println!("Emoji: {}", emoji);
    println!("Emoji length in bytes: {}", emoji.len());
    println!("Emoji chars count: {}", emoji.chars().count());

    // String Methods
    println!("\n---String Methods---");
    let sample = "   Hello, Rust!   ";
    println!("Original: '{}'", sample);
    println!("Trimmed: '{}'", sample.trim());
    println!("Uppercase: {}", sample.to_uppercase());
    println!("Lowercase: {}", sample.to_lowercase());
    println!("Replace: {}", sample.replace("Rust", "World"));
    println!("Contains 'Rust': {}", sample.contains("Rust"));

    // Global Variables
    println!("\n---Global Variables---");
    COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("Counter: {}", COUNTER.load(Ordering::SeqCst));

    // Copy
    println!("\n---Copy---");
    
    let copied_num = 10;
    let copied_num_2 = copied_num; // Ownership is not transferred, both original_num and copied_num have ownership of their data coz of primitive types. Primitive types have copy traits.
    println!("copied_num: {}", copied_num);
    println!("copied_num_2: {}", copied_num_2);
    
    let original_str = String::from("Hello");
    let copied_str = original_str; // Ownership is transferred to copied_str
    // println!("Original: {}", original_str); // This will throw an error coz original_str is moved to copied_str
    println!("Copied: {}", copied_str);
    

    // Clone
    // println!("\n---Clone---");
    // let original = String::from("Hello");
    // let copied = original.clone(); // Ownership is not transferred, both original and copied point to the same memory
    // println!("Original: {}", original);
    // println!("Copied: {}", copied);
    
    // References of variables
    println!("\n---References of variables---");
    let a = 10;
    let b: &i32 = &a;
    println!("a: {}", a);
    println!("b: {}", b);
    
    // References of values
    println!("\n---References of values---");
    let a: &i32 = &10;
    println!("a: {}", a);
    
    
}
