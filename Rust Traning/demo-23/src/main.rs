// Inserting in HashMap Rust
use std::collections::HashMap;
use std::collections::HashSet;

// fn change_array(array: &[i32]) {
//     for i in 0..array.len() {
//         array[i] = 0;
//     }
// }

#[allow(dead_code)]
fn arrays() {
    // Arrays
    println!("\n---------------Arrays-----------------");

    println!("\n---Arrays Creation---");
    let mut array1 = [1, 2, 3];
    let array2 = [4; 3];
    let _multi_array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    println!("{:?}", array1);
    println!("{:?}", array2);

    // Accessing Array Elements
    println!("\n---Accessing Array Elements---");
    println!("First element: {}", array1[0]);
    println!("Second element: {}", array1[1]);
    println!("Third element: {}", array1[2]);

    // Loops on Array
    println!("\n---Loops on Array using direct values---");
    for i in array1 {
        println!("{}", i);
    }

    println!("\n---Loops on Array using index---");
    let len = array1.len();
    for index in 0..len {
        array1[index] = 2 * array1[index];
        println!("{}-{}", index, array1[index]);
    }

    println!("\n---Loops on Array using iter---");
    for val in array1.iter() {
        println!("value is :{}", val);
    }

    println!("\n---Loops on Array using iter_mut---");
    for val in array1.iter_mut() {
        *val = 2 * *val;
        println!("value is :{}", val);
    }

    println!("\n---Loops on Array using iter and index---");
    for (index, val) in array1.iter().enumerate() {
        println!("index is :{} and value is :{}", index, val);
    }

    println!("\n---Mutable Arrays---");
    let mut array3 = [1, 2, 3];
    array3[0] = 4;
    println!("{:?}", array3);

    // Passing Arrays as Parameters to Functions
    // println!("\n---Passing Arrays as Parameters to Functions---");
    // let array4 = [1, 2, 3];

    // // Call by value
    // change_array(array4);
    // println!("{:?}", array4);

    // // Call by reference
    // change_array(&array4);
    // println!("{:?}", array4);
}

#[allow(dead_code)]
fn vectors() {
    // Vectors
    println!("\n---------------Vectors-----------------");

    println!("\n---Vectors Creation---");
    let mut vec1 = vec![1, 2, 3];
    let vec2 = vec![4; 3];
    let vec3: Vec<i32> = Vec::new();

    println!("{:?}", vec1);
    println!("{:?}", vec2);
    println!("{:?}", vec3);

    // Accessing Vector Elements
    println!("\n---Accessing Vector Elements---");
    println!("First element: {}", vec1[0]);
    println!("Second element: {}", vec1[1]);
    println!("Third element: {}", vec1[2]);

    // Iterating on the vector
    println!("\n---Iterating on the vector using direct values---");
    for i in &vec1 {
        println!("{}", i);
    }

    println!("\n---Iterating on the vector using index---");
    for index in 0..vec1.len() {
        println!("{}", vec1[index]);
    }

    println!("\n---Iterating on the vector using iter---");
    for val in vec1.iter() {
        println!("{}", val);
    }

    println!("\n---Iterating on the vector using iter_mut---");
    for val in vec1.iter_mut() {
        *val = 2 * *val;
        println!("{}", val);
    }

    println!("\n---Iterating on the vector using iter and index---");
    for (index, val) in vec1.iter().enumerate() {
        println!("index is :{} and value is :{}", index, val);
    }

    // Updating Vector
    println!("\n---Updating Vector---");
    vec1.push(4);
    println!("{:?}", vec1);

    // Removing Elements from Vector
    println!("\n---Removing Elements from Vector---");
    vec1.pop();
    println!("{:?}", vec1);

    // Removing Elements from Vector by Index
    println!("\n---Removing Elements from Vector by Index---");
    vec1.remove(0);
    println!("{:?}", vec1);

    // Removing Elements from Vector by Range
    println!("\n---Removing Elements from Vector by Range---");
    vec1.drain(0..2);
    println!("{:?}", vec1);

    // Removing Elements from Vector by Value
    println!("\n---Removing Elements from Vector by Value---");
    vec1.retain(|x| *x != 2);
    println!("{:?}", vec1);
    
    
}

#[allow(dead_code)]
fn tuples() {
    // Tuples
    println!("\n---------------Tuples-----------------");

    println!("\n---Tuples Creation---");

    let tuple1 = (1, 2, 3);
    let tuple2 = (1, "Hello", 3.14);

    println!("{:?}", tuple1);
    println!("{:?}", tuple2);

    // Accessing Tuple Elements
    println!("\n---Accessing Tuple Elements---");
    println!("First element: {}", tuple1.0);
    println!("Second element: {}", tuple1.1);
    println!("Third element: {}", tuple1.2);

    // Destructuring Tuples
    println!("\n---Destructuring Tuples---");
    let (a, b, c) = tuple1;
    println!("a: {}, b: {}, c: {}", a, b, c);
}

#[allow(dead_code)]
fn hashmap() {
    // HashMap
    println!("\n---------------HashMap-----------------");

    println!("\n---HashMap Creation---");

    let mut map = HashMap::new();

    // inserting records
    map.insert("Data Structures", "90");
    map.insert("Algorithms", "99");
    map.insert("Interview Preparations", "100");
    map.insert("FAANG", "97");
    map.insert("CP", "99");

    println!("{:?}", map);

    // Accessing HashMap using key
    println!("\n---Accessing HashMap using key---");

    // println!("{}", map.get(&"Algorithms")); // This wil throw error
    let value = map.get(&"Algorithms");
    println!("value={:?}", value);

    // Check for a Key
    println!("\n---Check for a Key---");
    println!("{}", map.contains_key(&"Data Structures"));

    // Iterating over a HashMap
    println!("\n---Iterating over a HashMap using direct values---");
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    println!("\n---Iterating over a HashMap using index---");
    let len = map.len();
    for index in 0..len {
        println!(
            "{}: {}",
            map.keys().nth(index).unwrap(),
            map.values().nth(index).unwrap()
        );
    }

    println!("\n---Iterating over a HashMap using iter---");
    for (key, value) in map.iter() {
        println!("{}: {}", key, value);
    }

    println!("\n---Iterating over a HashMap using iter_mut---");
    for (key, value) in map.iter_mut() {
        println!("{}: {}", key, value);
    }

    println!("\n---Iterating over a HashMap using iter and index---");
    for (index, (key, value)) in map.iter().enumerate() {
        println!(
            "index is :{} and key is :{} and value is :{}",
            index, key, value
        );
    }

    // Removing Elements from HashMap
    println!("\n---Removing Elements from HashMap---");
    map.remove(&"Data Structures");
    println!("{:?}", map);
}

#[allow(dead_code)]
fn hashset() {
    // HashSet
    println!("\n---------------HashSet-----------------");

    println!("\n---HashSet Creation---");
    let mut set = HashSet::new();
    set.insert("Data Structures");
    set.insert("Algorithms");
    set.insert("Interview Preparations");
    set.insert("FAANG");
    set.insert("CP");

    println!("{:?}", set);

    // Accessing HashSet Elements
    println!("\n---Accessing HashSet Elements---");
    for val in &set {
        println!("{}", val);
    }

    // Iterating over a HashSet
    println!("\n---Iterating over a HashSet using direct values---");
    for val in &set {
        println!("{}", val);
    }

    println!("\n---Iterating over a HashSet using index---");
    let len = set.len();
    for index in 0..len {
        println!("{}", set.iter().nth(index).unwrap());
    }

    println!("\n---Iterating over a HashSet using iter---");
    for val in set.iter() {
        println!("{}", val);
    }

    println!("\n---Iterating over a HashSet using iter and index---");
    for (index, val) in set.iter().enumerate() {
        println!("index is :{} and value is :{}", index, val);
    }

    // Removing Elements from HashSet
    println!("\n---Removing Elements from HashSet---");
    set.remove(&"Data Structures");
    println!("{:?}", set);

    // Removing Elements from HashSet by Value
    println!("\n---Removing Elements from HashSet by Value---");
    set.retain(|x| *x != "Algorithms");
    println!("{:?}", set);
}

fn main() {
    // arrays();
    // vectors();
    // tuples();
    // hashmap();
    hashset();
}
