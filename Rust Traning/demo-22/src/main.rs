fn main() {
    // Memory allocation/ optimization

    // Suppose 1 million concurrent user using this code, then we are wasting 3 million bytes. No need to define age in i32.
    // let age: i32 = 18;

    // Define a variable with a type annotation
    // let age: u8 = 18;

    // String Concatenation
    let str1: &str = "John";

    let str2: String = String::from("John");

    let str3 = str1.to_string() + &str2;

    println!("{}", str3);

    // Ownership transfer
    let str4 = String::from("Hello");
    let (length, str6) = get_length(str4);
    println!("{}", length);
    println!("{}", str6);
    
    // Borrowing
    let str7 = String::from("Borrowing");
    let str8 = &str7;
    println!("{}", str8);
    
    
    // Type Casting
    let int1: i32 = 10;
    let int2: i64 = int1 as i64;
    println!("{}", int2);
    
    let char1: char = 'a';
    let int3: i32 = char1 as i32;
    println!("{}", int3);
    
    let char2:char = '😀';
    let int4: u8 = char2 as u8;
    println!("{}", int4);
    
    // Option enum
    let x: Option<i32> = None;
    println!("{:?}", x);
    
    let char5: Option<char> = Some('a');
    let char6: Option<char> = None;
    println!("{:?}", char5);
    println!("{:?}", char6);
    
    
    let age:i32 = 18; // 4 byte
    // If 1 million users run this concurrently, then we are wasting 3 million bytes. No need to define age in i32.
    let age:u8 = 18; // 1 byte
    
    // Either in heap/stack
}

fn get_length(s: String) -> (usize, String) {
    let length = s.len();
    return (length, s);
}
