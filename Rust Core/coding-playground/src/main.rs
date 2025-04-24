
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
    
    println!("{}", string_literal[0]);

    // println!("\n---String Literal Length---");
    // println!("Length of string literal: {}", string_literal.len());
}

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
    
    println!("\n---Indexing---");
    println!("First character: {}", string_literal[0]);

}

fn main() {
    string_literal();
    println!("\n✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦✦\n");
    string_object();
}
