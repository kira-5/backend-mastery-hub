# Rust Traning 24 April 2025

## Heap Memory

### c/c++

**Allocation:**

- malloc
- calloc
- realloc
- free

**Deallocation:**

- free

### Java

**Allocation:**

- new

**Deallocation**

- GC

### Rust

**Allocation:**

- Box or variable

**Deallocation:**

- borrow checker
  - Ownership
  - Borrowing (& mut)
  - Lifetime

## Pointers box dangling



## Dangling Pointer

-

## Lifetime

- Lifetime are on the reference, not on the variable
- It will not chnage code, only tell compiler about the lifetime
- Lifetime didn't extend the time of the variable
- Wherever use reference, use lifetime - Thumb rule

```rust
Struct Person {
    name: &str,
    email: &str,
    sm: &Vec<String>,
}

{
  p = &p
}
// Here we hav eto define lifetime for p and tell compiler that till when p alive keep alive name,email, sm. i.e. refernce within reference

```

## Box

- Box is a smart pointer
- Box store on stack
- Box consists pointer that store in heap memory
- Dangling pointer solve by Box pointer


## named Struct


## Tuple Struct

## unit Struct

## empty STruct


## Traits

## Static Dispatch

## Dynamic Dispatch

## Chain of methods