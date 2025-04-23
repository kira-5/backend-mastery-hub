1. Why rust not c, c++, java?
2. CSP - communicating sequential processes
3. Golang, csp, erlang adapt csp
4. Null pointer --> tony hoare said it's a mistake i developeed null pointer
5. Rust is a system programming language. No functional programming, no oops, no procedural programming.
6. Rust don't have override, it use traits.
7. ; Consider as a statment.
8. no ; Consider as a expression.

Promming Language problem in C/C++/Java:
1. Null pointer dereference --> undefined behavior
2. Dangling pointer --> undefined behavior
3. Double free --> undefined behavior
4. Memory leak --> undefined behavior
5. Data race --> undefined behavior
6. Stack overflow --> stack overflow

Rust solution:
1. Ownership
2. Borrowing
3. Lifetimes

To avoid above problem, Rust use Ownership, Borrowing, Lifetimes. and Rust enforce these rules at compile time.

Ownership:
1. Each value in Rust has a variable that is called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.


Java compile process:

Java --> java compiler --> byte java code --> java interpreter --> java virtual machine(JVM, JIT) --> run on operating system
JVM have garbage collector, that resolve memory leak problem.
50/100 mb size of JVM, 100/1000 mb size of java code. JVM require 256mb ram, 500 cycles of cpu. __> This cause more number of instance require, which cost more money.
java/python take more time to start application if u increase number of instance.
Java is Just In Time compiler.

Compilation process in rust:

Rust --> rustc --> rust code --> LLVM toolchain --> machine code --> run on operating system

Rust use LLVM to compile to machine code. LLVM is a compiler infrastructure, not a compiler.

Rust is ahead of time compiler.

LLVM consists of 3 parts:
1. LLVM Frontend:
2. LLVM IR:
3. LLVM Backend: Target Machine (Triplets)

FF --> IR --> BE --> machine code (Static Binaries) 1 MB - rust wrote its own libssy.



RUST:
1. Macros: 
    - Macros are a way to generate code at compile time.
    - Macros are preprocessor, not a function.
    - Example: println!("Hello, world!");

2. Variables:
    - Variables are immutable by default.
    - Example: let x = 5;
    - Example: let mut x = 5;
    - Example: let x = 5;
    - Example: let x = 5;
    
3. Data Types:
    - Signed Integer: i8, i16, i32, i64, isize
    - Unsigned Integer: u8, u16, u32, u64, usize
    - Floating Point: f32, f64
    - Boolean: bool
    - Character: char
    - &str: string slice --> Data segment 
    - String: String
    - Array: [i32; 5]
    
    - {let number: i32 = 10;}
        - Whenever it's a block, it's a new scope. and its create a stack frame.
        - Stack frame is a data structure that contains the local variables and the return address of the function.
        - Stack frame is created on the stack.
        - Stack frame is destroyed when the function returns.
        - Stack frame is created when the function is called.
        - Stack frame is destroyed when the function returns.
        
    
    - let number: i32 = 10;
        - 4 bytes are allocated on stack
        ? Allocated by whome
            - Compiler
        ? Where it gets allocated?
            - Stack
        ? Stack or Heap?
            - Stack
            
    - let (a: i32, b: i32, c: i32) = (1, 2, 3);
    
    - num1 = 5
    - num2 = num1
        - This is copying the value of num1 to num2.
        
    - const vs let vs static
        - const is a constant value that is declared with the const keyword.
        - let is a variable that is declared with the let keyword.
        - static is a static variable that is declared with the static keyword.
    
        - Const:
            - Stored in stack
            - Compile time constant
            - Cannot be mutable
            - Cannot be dynamic
            - Cannot be heap allocated
            - It is in code segment
        
        - let:
            - Stored in stack
            - Runtime constant
            - Can be mutable
            - Can be dynamic
            - Can be heap allocated
            - It is in data segment
            
        - static:
            - Stored in stack
            - Compile time constant
            - Can be mutable
            - Can be dynamic
            - Can be heap allocated
            - It is in data segment
            
    - Process:
        - Instance of running program.
            
    - program code:
        - Text Section
        - Instruction  from the executable.
            
    - program Counter:
        - Keep track of the next instruction to execute.
        - Program counter is a register that contains the address of the next instruction to execute.
        - It is in code segment
        - It is a pointer to the next instruction to execute.
            
    - Memory Layout:
        
        - Code Segment:
            - Code segment is a global memory.
            - Code segment is allocated on the heap.
            - Code segment is destroyed when the program ends.
            - Code segment is created when the program starts.
            - It never deallocated.
            
        - Data Segment:
            - Data segment is a global memory.
            - Data segment is allocated on the heap.
            - Data segment is destroyed when the program ends.
            - Data segment is created when the program starts.
            - It never deallocated.
        
        - Stack memory
            - Stack memory is allocated on the stack.
            - Stack memory is destroyed when the function returns.
            - Stack memory is created when the function is called.
            - Stack memory is destroyed when the function returns.
            - It deallocated.
            
        - heap memory
            - Heap memory is  a global memory.
            - Heap memory is allocated on the heap.
            - Heap memory is destroyed when the program ends.
            - Heap memory is created when the program starts.
            - It deallocated
            - WHen heap come into picture, it's difficult but use it.
            - Copy traits are not available for heap memory/allocation.
        
        - Why do we need heap memory? 
            - When we need to allocate memory at runtime.
            - When we need to allocate memory more than the stack can handle.
            - When we need to allocate memory for a variable that is not known at compile time.
            - Numbers data type we can prededined but for strings we need to allocate memory at runtime.
                - str1 = "Hello, world!"
                - str2 = "Hello Hello, world!"
                - Why value is dynamic where heap is required.
                
    - Process Control Block:
        - Process Control Block is a data structure that contains the process information.
        - Process ID
        - CPU Scheduling Info
        - Process State
        - IO Status
        - Memory Management Info(Page, Table, etc)
        
        
    - String:
        - A Sequence of bytes
        - The concat of string , always assign new address, it's suitable not to concat string.
        
        - &str:
            - The actual data stored in data segment
            -  Reference and structure stored in stack memory
            - Small size data store in &str
        
        - String:
            - The actual data stored in data segment
            - Reference and structure stored in heap memory
            - Large size data store in String like HTML, JSON, etc.
            
        - If you want memory to be allocated and deallocated automatically, you can use String and not &str.
        
    - Global Variables:
        - Cause Data Races Issues.
        - rust says it's a unsafe code.
        - try to avoaid global variables.
        - Even if u have to use, declare unsafe keyword.
        
        
    - Normal Borrow vs mutable borrow:
        - Normal borrow:
            - The actual data stored in data segment
            - Reference and structure stored in stack memory
            - Small size data store in Normal borrow
        - Mutable borrow:
            - The actual data stored in data segment
            - Reference and structure stored in heap memory
            - Large size data store in Mutable borrow
        

        
    
    
        
        
        
https://www.scaler.com/topics/rust-vs-python/