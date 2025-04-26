// Helper function to get type name
fn get_type<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn string_literal() {
    println!("\n---------------String Literal-----------------");

    println!("\n--- Empty String Literal Creation---");
    let empty_string_literal: &str = "";
    println!("Empty String Literal: {}", empty_string_literal);
    println!("Type: {}", get_type(&empty_string_literal));
    println!("Bytes: {:?}", empty_string_literal.as_bytes());
    println!("Memory Address: {:p}", empty_string_literal);

    println!("\n---String Literal Creation---");
    let string_literal: &str = "Hello, world!";
    println!("String Literal: {}", string_literal);
    println!("Type: {}", get_type(&string_literal));
    println!("Bytes: {:?}", string_literal.as_bytes());
    println!("Memory Address: {:p}", string_literal);
    println!("Length: {}", string_literal.len());

    println!("\n---Convert to String Object---");
    let string_object = string_literal.to_string();
    println!("String Object: {}", string_object);
    println!("Type: {}", get_type(&string_object));
    println!("Bytes: {:?}", string_object.as_bytes());
    println!("Memory Address: {:p}", string_object.as_str());

    println!("\n---Slicing---");
    let slice = &string_literal[0..5];
    println!("Slice: {}", slice);

    println!("\n---Indexing---");
    let first_char = string_literal.chars().nth(0);
    println!("First character: {}", first_char.unwrap());
    let first_byte = string_literal.bytes().nth(0);
    println!("First byte: {}", first_byte.unwrap());

    println!("\n---Iteration---");

    println!("\n--Iteration over characters--");
    for c in string_literal.chars() {
        println!("{}", c);
    }

    println!("\n--Iteration over bytes--");
    for b in string_literal.bytes() {
        println!("{}", b);
    }

    println!("\n--Iteration over indices and characters--");
    for (i, c) in string_literal.chars().enumerate() {
        println!("{}: {}", i, c);
    }

    // String Conversion
    println!("\n---String Conversion---");

    // 1. &str to String (3 common ways)
    let string_literal = "Hello 🦀";
    let owned_string1 = string_literal.to_string();
    let owned_string2 = String::from(string_literal);
    let owned_string3: String = string_literal.into();
    println!("Owned String 1: {}", owned_string1);
    println!("Owned String 2: {}", owned_string2);
    println!("Owned String 3: {}", owned_string3);

    // 2. Bytes to String (UTF-8 validation required)
    println!("\n--Bytes to String--");
    let bytes = vec![72, 101, 108, 108, 111, 32, 240, 159, 166, 128]; // "Hello 🦀" in bytes
    let bytes_to_string = String::from_utf8(bytes).expect("Invalid UTF-8 bytes");
    println!("Bytes to String: {}", bytes_to_string);

    // 3. String to Bytes
    println!("\n--String to Bytes--");
    let string_to_bytes = owned_string1.as_bytes(); // No allocation needed
    println!("String to Bytes: {:?}", string_to_bytes);

    // 4. Number to String
    println!("\n--Number to String--");
    let number = 42;
    let number_to_string = number.to_string();
    println!("Number to String: {}", number_to_string);

    // 5. String to Number (parsing)
    println!("\n--String to Number--");
    let string_num = "42";
    let parsed_num: i32 = string_num.parse().expect("Not a number!");
    println!("String to Number: {}", parsed_num);
}

#[allow(unused)]
fn string_object() {
    println!("\n---------------String Object-----------------");

    println!("\n--- Empty String Object Creation---");
    let empty_string_object = String::new();
    println!("Empty String Object: {}", empty_string_object);
    println!("Type: {}", get_type(&empty_string_object));
    println!("Bytes: {:?}", empty_string_object.as_bytes());
    println!("Memory Address: {:p}", empty_string_object.as_str());

    println!("\n---String Object Creation---");
    let string_object = String::from("Hello, world!");
    println!("{}", string_object);
    println!("Type: {}", get_type(&string_object));
    println!("Bytes: {:?}", string_object.as_bytes());
    println!("Memory Address: {:p}", string_object.as_str());
    println!("Capacity: {}", string_object.capacity());

    println!("\n---Convert to String Literal---");
    let string_literal = string_object.as_str();
    println!("String Literal: {}", string_literal);
    println!("Type: {}", get_type(&string_literal));
    println!("Bytes: {:?}", string_literal.as_bytes());
    println!("Memory Address: {:p}", string_literal);

    println!("\n---Slicing---");
    let slice = &string_object[0..5];
    println!("Slice: {}", slice);

    println!("\n---Indexing---");
    let first_char = string_object.chars().nth(0);
    println!("First character: {}", first_char.unwrap());
    let first_byte = string_object.bytes().nth(0);
    println!("First byte: {}", first_byte.unwrap());

    println!("\n---Iteration---");
    println!("\n--Iteration over characters--");
    for c in string_object.chars() {
        println!("{}", c);
    }

    println!("\n--Iteration over bytes--");
    for b in string_object.bytes() {
        println!("{}", b);
    }

    println!("\n--Iteration over indices and characters--");
    for (i, c) in string_object.chars().enumerate() {
        println!("{}: {}", i, c);
    }
    
    // String Operations
    println!("\n---String Operations---");
    let mut string_object = String::from("Hello, world!");

    println!("\n--Push--");
    string_object.push('!');
    println!("String Object: {}", string_object);
    
    
}

fn main() {
    string_literal();
    println!("\n✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦\n");
    string_object();
}
