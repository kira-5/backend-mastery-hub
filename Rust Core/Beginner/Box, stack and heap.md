# Box, stack and heap

## Stack

### Features of the stack:

- Fixed size
- Fast access
- Last in, first out (LIFO)
- Contiguous memory allocation
- No garbage collection
- In Rust, primitive types and local variables of functions are allocated on the stack by default.
- In Rust, memory is allocated on the stack by default. But the allocation is local to a function call, and is limited in size. 
- Memory allocation and deallocation on the stack are automatic and managed by the compiler.

## Heap

### Features of the heap:

- Dynamic size
- Slow access
- First in, first out (FIFO)
- Non-contiguous memory allocation
- Garbage collection
- In Rust, memory is allocated on the heap is explicitly allocated by your program. It is globally accessible. Unlimited size.

## Box

### Features of a Box:

- Box is a smart pointer that allocates memory on the heap.
- Box is a pointer to the heap.
- Box is a fixed size.
- Box is a last in, first out (LIFO)
