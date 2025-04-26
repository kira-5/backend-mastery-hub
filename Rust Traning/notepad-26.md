# Rust Traning 26 April 2025

## Threading

- Rust provides powerful tools for concurrent programming through its threading model.
- When yu create a thread, it should be create at system level i.e. native thread, not at the application level.
- What value to pass to thread, it should be a pass by value, not pass by reference.
- The movement you write `spawn(|| {})` is a move closure, it will start the thread and move the value to the thread.

**Key Points:**

- You're properly spawning threads with thread::spawn
- You're using join() to wait for threads to complete
- You're handling the Result from join() properly
- You're using Duration for sleeps

Main Thread        Thread 1          Thread 2
|----------------|----------------|----------------|
| Start          | Spawned        | Spawned        |
| Print message  | Print 0        | Print 11       |
| Join Thread 1  | Sleep 500ms    | Sleep 500ms    |
| (blocks)       | Print 1        | Print 12       |
|                | ...            | ...            |
| Join Thread 2  | Print 9        | Print 19       |
| (blocks)       | Complete       | Complete       |
| Print results  |                |                |
| Complete       |                |                |

**Important Points:**

1. No Data Races

- Even though threads access println! simultaneously, it's thread-safe
- Rust's ownership system prevents true data races

2. Sleep Behavior

- thread::sleep yields execution to other threads
  - This is why you see interleaved output

3. Join Order Matters

- The main thread waits for handle1 completely before waiting for handle2
- But both child threads are running concurrently before joining

### Basic Thread Creation

```rust
  use std::thread;

  fn main() {
      // Spawn a new thread
      let handle = thread::spawn(|| {
          thread::sleep(Duration::from_secs(1));
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
- Use `Arc<T>` for shared ownership across threads
- Combine `Mutex<T>` with `Arc<T>` for shared mutable state
- Consider higher-level abstractions like the `rayon` crate for parallel iterators
- Be mindful of thread overhead - creating thousands of threads is usually not efficient
  
## Error Handling

## async and await

## Actix Web
