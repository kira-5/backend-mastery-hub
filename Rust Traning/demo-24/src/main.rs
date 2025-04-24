fn dangling_pointer(mut n: i32) -> i32 {
    // new n is created
    // Dangling Pointer
    println!("\n---------------Dangling Pointer-----------------");
    n = n * n;
    return n; // This is dangling pointer
}

fn lifetime<'a>(mut n: &'a mut i32) -> &'a i32 {
    // new n is created
    // Lifetime
    println!("\n---------------Lifetime-----------------");
    *n = *n * *n;
    return n; // This is dangling pointer
}

fn box_pointer() {
    println!("\n---------------Box Pointer-----------------");
    let b = Box::new(10);
    println!("b: {}", b);
}

struct Person {
    id: i32,
    name: String,
    email: String,
    sm: Vec<String>,
}

fn struct_() {
    println!("\n---------------Struct-----------------");
    let p: Person = Person {
        id: 101,
        name: String::from("John"),
        email: String::from("john@example.com"),
        sm: Vec::new(),
    };
    println!("p: {}", p); // Save Person in stack

    // Save in heap memory
    let p1: Box<Person> = Box::new(Person {
        id: 102,
        name: String::from("Jane"),
        email: String::from("jane@example.com"),
        sm: Vec::new(),
    });
    println!("p1: {}", p1);
    
    println!("p1: {}", *p1);
}

fn main() {
    // let n = 100;
    // let n1 = dangling_pointer(n);
    // let n2 = lifetime(&mut n);
    // println!("n: {}", n);
    // println!("n1: {}", n1);
    // println!("n2: {}", n2);

    // struct_();
}
