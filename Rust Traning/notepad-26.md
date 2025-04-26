# Rust Traning 26 April 2025

## Threading

- Rust provides powerful tools for concurrent programming through its threading model.

### Basic Thread Creation

```rust
  use std::thread;

  fn main() {
      // Spawn a new thread
      let handle = thread::spawn(|| {
          println!("Hello from a new thread!");
      });

      println!("Hello from the main thread!");
      
      // Wait for the thread to finish
      handle.join().unwrap();
  }
```

### Threads with Move Closures

- When you need to transfer ownership of variables to a thread:

```rust
  use std::thread;

  fn main() {
      let message = String::from("Hello from the spawned thread!");
      
      let handle = thread::spawn(move || {
          println!("{}", message);
      });
      
      handle.join().unwrap();
      
      // Can't use `message` here - ownership was moved
  }
```

### Shared-State Concurrency

- Rust's ownership system helps prevent data races. For shared mutable state, you need synchronization:
  
**Using Mutex (Mutual Exclusion)**

```rust
  use std::sync::{Arc, Mutex};
  use std::thread;

  fn main() {
      let counter = Arc::new(Mutex::new(0));
      let mut handles = vec![];

      for _ in 0..10 {
          let counter = Arc::clone(&counter);
          let handle = thread::spawn(move || {
              let mut num = counter.lock().unwrap();
              *num += 1;
          });
          handles.push(handle);
      }

      for handle in handles {
          handle.join().unwrap();
      }

      println!("Result: {}", *counter.lock().unwrap());
  }
```

**Using Channels for Message Passing:**

```rust
  use std::sync::mpsc; // multiple producer, single consumer
  use std::thread;

  fn main() {
      let (tx, rx) = mpsc::channel();

      thread::spawn(move || {
          let val = String::from("hello");
          tx.send(val).unwrap();
      });

      let received = rx.recv().unwrap();
      println!("Got: {}", received);
  }
```

### Thread Pools

- For more efficient thread management, you can create thread pools:

```rust
  use std::sync::{Arc, Mutex};
  use std::thread;

  struct ThreadPool {
      workers: Vec<Worker>,
      sender: mpsc::Sender<Job>,
  }

  impl ThreadPool {
      pub fn new(size: usize) -> ThreadPool {
          assert!(size > 0);

          let (sender, receiver) = mpsc::channel();
          let receiver = Arc::new(Mutex::new(receiver));

          let mut workers = Vec::with_capacity(size);

          for id in 0..size {
              workers.push(Worker::new(id, Arc::clone(&receiver)));
          }

          ThreadPool { workers, sender }
      }

      pub fn execute<F>(&self, f: F)
      where
          F: FnOnce() + Send + 'static,
      {
          let job = Box::new(f);
          self.sender.send(job).unwrap();
      }
  }
```

### Best Practices

- Prefer message passing over shared state when possible
- Use Arc<T> for shared ownership across threads
- Combine Mutex<T> with Arc<T> for shared mutable state
- Consider higher-level abstractions like the rayon crate for parallel iterators
- Be mindful of thread overhead - creating thousands of threads is usually not efficient
  
## Error Handling

## async and await

## Actix Web
