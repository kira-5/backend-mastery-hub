1. Expression
    - This is important.
    
- &str and String have header, so data got save in data segement and pointer got save in stack.

**Compound Types** 
2. Arrays

    - A fixed-sized collection of elements denoted by [T; N] where T is the element type and N is the compile-time constant size of the array.
    - Arrays are fixed size coz it is stack allocated.
    - Arrays are stored in the stack.
    - Arrays are homogenous.(Contains only one type of data)
    - Arrays are not dynamic.
    - Arrays are not flexible.
    - Arrays are not scalable.
    - Values in the array element can be updated or modified but cannot be deleted.
    - Memory blocks in the array are in sequential order.
    - To identify array elements we use a unique integer known as a subscript.
    - Fixed-size, same-type elements
    - Arrays didn't have header.

    - Array Creation:

        - Simply a list with each element [a, b, c].
        - Repeat expression [N, X]. This will create an array with N copies of X.

        ```rust
        let array1 = [1, 2, 3];
        let array2 = [4; 3];

        println!("{:?}", array1);
        println!("{:?}", array2);
        ```

    - Loops on Array:
        - We can iterate on the array by using the `for` loop.
        - We can also use the `iter` method to iterate on the array. This is borrowing coz it used `&self` inside `iter()`.
        - We can also use the `iter_mut` method to iterate on the array. This is mutable borrowing coz it used `&mut self` inside `iter_mut()`.
        - We can also use the `enumerate` method to iterate on the array.
        ```rust
        for i in array1 {
            println!("{}", i);
        }
        ```
        ```rust
        let len = array1.len();
        for index in 0..len {
            println!("{}-{}", index, array1[index]);
        }
        ```
        ```rust
            for val in array1.iter() {
                println!("{}", val);
            }
        ```
        ```rust
            for val in array1.iter_mut() {
                *val = 2 * *val;
                println!("{}", val);
            }
        ```
        ```rust
            for (index, val) in array1.iter().enumerate() {
                println!("index is :{} and value is :{}", index, val);
            }
        ```
    - Mutable Arrays / Updating Arrays:
        - We can create a mutable array by using the `mut` keyword.
        - We can update the value of the array element.
        ```rust
            let mut array1 = [1, 2, 3];
            array1[0] = 4;
            println!("{:?}", array1);
            let len = array1.len();
            for i in 0..len {
                array1[i] = 2 * array1[i];
                println!("{}-{}", i, array1[i]);
            }
        ```
    - Passing Arrays as Parameters to Functions:

        - We can pass an array as a parameter either by a call by value or call by reference to a function.

        ```rust
        // Call by value
        fn main() {
            let gfg_array = [1,2,3,4];
            change_array(gfg_array);

            println!("Original array {:?}",gfg_array);
        }
        fn change_array(mut gfg_array:[i32;4]){
            for i in 0..4 {
                gfg_array[i] = 0;
            }
            println!("Changed array {:?}",gfg_array);
        }
        ```

        ```rust
        // Call by reference
        fn main() {
            let gfg_array = [1,2,3,4];
            change_array(&gfg_array);
        }
        fn change_array(gfg_array:&[i32]){
            for i in 0..4 {
                gfg_array[i] = 0;
            }
            println!("Changed array {:?}",gfg_array);
        }
        ```
    
    - Array as a slice
        - We can pass an array as a slice to a function.
        - We can also use the `slice` method to get a slice of the array.
        ```rust
            let slice = &array1[0..2];
            println!("{:?}", slice);

            fn print_slice(slice: &[i32]) {
                println!("{:?}", slice);
            }

            print_slice(&array1);
        ```
        
    - WHy use slice?
        - Write function always with slice not array

3. Vectors

    - growable array (vector)
    - Vectors are dynamic size.
    - Vectors are contiguous resizable array type.
    - Vectors are stored in the heap.
    - Vectors are homogenous.(Contains only one type of data)
    - Vectors are flexible.
    - Vectors are scalable.
    - Vectors in Rust have O(1) indexing and push and pop operations in vector also take O(1) complexity.
    - Vectors ensure they never allocate more than isize::MAX bytes.
    - Its capacity defines the actual allocated space on the heap of this vector that is in the form of 2^n.
    -

    - Creating Vectors:

        - Vector can be created by using `Vec::new()` or `vec![]`.

        ```rust
            let v : Vec<i64> = Vec::new();

            println!("{ }",v.len());
        ```

        ```rust
            // using macro
            let v = vec![1, 2, 3];
            println!("{:?}", v);
        ```

        ```rust
            // using repeat expression
            let v = vec![4; 3];
            println!("{:?}", v);
        ```

    - Accessing a Vector:
        - We can access a vector by using the index i.e. using subscript operator: .
        - We can also use the `get` method to access a vector.
        ```rust
            let v = vec![1, 2, 3];
            println!("{}", v[0]);
        ```
        ```rust
            let v = vec![1, 2, 3];
            println!("{:?}", v.get(0));
        ```
    - Iterating on the vector:
        - We can iterate on the vector by using the `for` loop.
        - We can also use the `iter` method to iterate on the vector.
        - We can also use the `iter_mut` method to iterate on the vector.
        - We can also use the `enumerate` method to iterate on the vector.
        ```rust
            let v = vec![1, 2, 3];
            for i in v {
                println!("{}", i);
            }
        ```
        ```rust
            let v = vec![1, 2, 3];
            for i in 0..v.len() {
                println!("{}", v[i]);
            }
        ```
        ```rust
            let v = vec![1, 2, 3];
            for i in v.iter() {
                println!("{}", i);
            }
        ```
        ```rust
            let v = vec![1, 2, 3];
            for i in v.iter_mut() {
                *i = 2 * *i;
                println!("{}", i);
            }
        ```
        ```rust
            let v = vec![1, 2, 3];
            for (index, i) in v.iter().enumerate() {
                println!("index is :{} and value is :{}", index, i);
            }
        ```
    - Updating a Vector:
        - We can update a vector by using the `push` method.
        - We can also use the `pop` method to remove the last element from the vector.
        ```rust
            let mut v = vec![1, 2, 3];
            v.push(4);
        ```
    - Removing Elements from Vector:
        - Removing Elements from Vector: Using `pop` method.
        - Removing Elements from Vector by Index: Using `remove` method.
        - Removing Elements from Vector by Value: Using `retain` method.
        - Removing Elements from Vector by Range: Using `drain` method.
        ```rust
            let mut v = vec![1, 2, 3];
            v.pop();
        ```
        ```rust
            let mut v = vec![1, 2, 3];
            v.remove(0);
        ```
        ```rust
            let mut v = vec![1, 2, 3];
            v.retain(|x| *x != 2);
        ```
        ```rust
            let mut v = vec![1, 2, 3];
            v.drain(0..2);
        ```
        
    - Vector as a slice

4. Tuples

    - Tuples are fixed size collection of elements of different types.
    - A tuple in rust is a finite heterogeneous compound data type, which means it can hold elements of different types.
    - Tuples are stored in the stack.
    - In tuples there is no inbuilt method to add elements into a tuple.
    - We can use the index to get the value of a tuple, and we also can not iterate over a tuple using for loop.
    - tuples are a sequence in Rust.
    - Tuples are not dynamic.
    - Tuples are not flexible.
    - Tuples are not scalable.
    - Tuples are not homogenous.
    - Fixed-size, heterogenous types
    - Searching in tuple is not possible, Not typically applicable — you use pattern matching, but not like searching a list.

    - Creating Tuples:
        - We can create a tuple by using the `()` operator.
        - We can also use the `tuple::new` method to create a tuple.
        ```rust
            let tuple1 = (1, 2, 3);
            let tuple2 = tuple::new(1, 2, 3);
            let tuple3: (i32, char, f64, bool) = (1, 'a', 3.14, true);
        ```
    - Accessing a Tuple:
        - We can access a tuple by using the index i.e. using subscript operator: .
        - We can also use the `get` method to access a tuple.
        ```rust
            let tuple1 = (1, 2, 3);
            println!("{}", tuple1.0);
        ```
    - Destructuring a Tuple:

        - We can destruct a tuple by using the `let` keyword.

        ```rust
            let (a, b, c) = tuple1;
        ```

    - Loops:
        - We can iterate over a tuple by using the `for` loop.
        ```rust
            for i in tuple1 {
                println!("{}", i);
            }
        ```

5. HashMap

    - HashMap<K, V> is a collection of key-value pairs.
    - Keys are unique no duplicates allowed in the key but the value can be duplicated.
    - We can check for key in the HashMap by using the `contains_key` method. but cannot check for value.

    - Creating HashMap:

        - We can create a HashMap by using the `HashMap::new()` method.
        - We can also use the `hashmap::new` method to create a HashMap.

        ```rust
            let mut map = HashMap::new();

            // inserting records
            map.insert("Data Structures", "90");
            map.insert("Algorithms", "99");
            map.insert("Interview Preparations", "100");
            map.insert("FAANG", "97");
            map.insert("CP", "99");

            println!("{:?}", map);
        ```

    - Accessing HashMap:

        - We can access a HashMap by using the `get` method.
        - Use get( & key) for getting the value.

        ```rust
            println!("{}", map.get(&"Data Structures")); // This will throw error

            let value= map.get(&"Algorithms");
            println!("value={:?}",value);
        ```

    - Check for a Key:
        - We can check for a key by using the `contains_key` method.
        ```rust
            println!("{}", map.contains_key(&"Data Structures"));
        ```
    - Iterating over a HashMap:

        - We can iterate over a HashMap by using the `for` loop.
        - We can also use the `iter` method to iterate over a HashMap.
        - We can also use the `iter_mut` method to iterate over a HashMap.
        - We can also use the `enumerate` method to iterate over a HashMap.

        ```rust
            for (key, value) in map {
                println!("{}: {}", key, value);
            }
        ```

        ```rust
             let len = map.len();
            for index in 0..len {
                println!("{}: {}", map.keys().nth(index).unwrap(), map.values().nth(index).unwrap());
            }
        ```

        ```rust
            for (key, value) in map.iter() {
                println!("{}: {}", key, value);
            }
        ```

        ```rust
            for (key, value) in map.iter_mut() {
                println!("{}: {}", key, value);
            }
        ```

        ```rust
            for (index, (key, value)) in map.iter().enumerate() {
                println!("index is :{} and key is :{} and value is :{}", index, key, value);
            }
        ```

    - Removing Elements from HashMap:
        - We can remove elements from HashMap by using the `remove` method.
        - Use the remove(key) method to remove from HashMap.
        ```rust
            map.remove(&"Data Structures");
        ```

6. HashSet

    - HashSet<T> is a collection of unique unordered values.

    - Creating HashSet:
        - We can create a HashSet by using the `HashSet::new()` method.
        - We can also use the `hashset::new` method to create a HashSet.
        ```rust
            let mut set = HashSet::new();
        ```
    - Accessing HashSet:
        - We can access a HashSet by using the `get` method.
        - Use get( & value) for getting the value.
        ```rust
            println!("{}", set.get(&"Data Structures")); // This will throw error
        ```
    - Iterating over a HashSet:
        - We can iterate over a HashSet by using the `for` loop.
        - We can also use the `iter` method to iterate over a HashSet.
        - We can't use the `iter_mut` method to iterate over a HashSet.
        - We can also use the `enumerate` method to iterate over a HashSet.
    - Removing Elements from HashSet:
        - We can remove elements from HashSet by using the `remove` method.
        - Use the remove(value) method to remove from HashSet.
        ```rust
            set.remove(&"Data Structures");
        ```
    - Removing Elements from HashSet by Value:
        - We can remove elements from HashSet by using the `retain` method.
        - Use the retain(|value|) method to remove from HashSet.
        ```rust
            set.retain(|x| *x != "Algorithms");
        ```

* * *

### ✅ **7. Difference between Array and Vector**

    - Array is a fixed size collection of elements of the same type.
    - Vector is a growable collection of elements of the same type.
    - Array is stored in the stack.
    - Vector is stored in the heap.
    - Array is not flexible.
    - Vector is flexible.
    - When to use array and when to use vector:
        - Use array when you know the size of the collection.
        - Use vector when you don't know the size of the collection.

| Feature | Array | Vector |
| --- | --- | --- |
| Size | Fixed | Growable |
| Memory | Typically on the **stack** (if small) | **Heap** |
| Flexibility | Not flexible | Flexible |
| Use case | When size is **known at compile time** | When size is **dynamic or unknown** |

> 💡 **Note:** Arrays can also be stored on the heap if boxed (`Box<[T; N]>`), but commonly reside on the stack.

* * *
        
### ✅ **8. Difference between HashMap and HashSet**
    - HashMap is a collection of key-value pairs.
    - HashSet is a collection of unique unordered values.
    - HashMap is stored in the heap.
    - HashSet is stored in the stack.
    - HashMap is not ordered.
    - HashSet is not ordered.
    - When to use HashMap and when to use HashSet:
        - Use HashMap when you need to store key-value pairs.
        - Use HashSet when you need to store unique unordered values.
        
| Feature | HashMap | HashSet |
| --- | --- | --- |
| Structure | Key-Value pairs | Unique values only |
| Storage | Stored on the **heap** | Also stored on the **heap** |
| Ordering | Not ordered | Not ordered |
| Use case | When key-value mapping is needed | When unique value storage is needed |

> 💡 **Note:** HashMap and HashSet are both stored on the heap.

* * *

### ✅ **9. Time Complexity of Compound Types in Rust**

#### **String**

* Insertion:
    
    * End → **O(1)** (amortized, because it's backed by a Vec<u8>)
        
    * Beginning/Middle → **O(n)** (shifting required)
        
* Deletion: **O(n)** (as it may involve shifting and UTF-8 correctness)
    
* Search: **O(n)** (pattern matching)
    

#### **Array** (`[T; N]`)

* Insertion: **O(n)** (shifting required)
    
* Deletion: **O(n)** (shifting required)
    
* Search: **O(n)** (linear scan)
    

#### **Vector** (`Vec<T>`)

* Insertion:
    
    * End → **O(1)** (amortized)
        
    * Beginning/Middle → **O(n)**
        
* Deletion:
    
    * End → **O(1)**
        
    * Beginning/Middle → **O(n)** (shifting required)
        
* Search: **O(n)** (linear scan)
    

#### **Tuple**

* Access: **O(1)** (index known at compile time)
    

> 🧠 No dynamic insertion/deletion/search in tuples.

#### **HashMap**

* Insertion: **O(1)** (amortized average case), **O(n)** worst-case (collisions)
    
* Deletion: **O(1)** (amortized average case)
    
* Search: **O(1)** (average case), **O(n)** worst-case (collisions)
    

#### **HashSet**

* Insertion: **O(1)** (average case)
    
* Deletion: **O(1)** (average case)
    
* Search: **O(1)** (average case)
    

* * *
