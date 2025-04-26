use std::thread;
use std::time::Duration;

fn threading() {
    // Spawn a new thread
    let handle1 = thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        println!("Hello from a new thread!");
        for i in 0..10 {
            println!("Sub thread-1: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    let handle2 = thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        println!("Hello from a new thread!");
        for i in 11..20 {
            println!("Sub thread-2: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    println!("Hello from the main thread!");

    // Wait for the thread to finish
    // handle1.join().unwrap();
    // handle2.join().unwrap();

    let result1 = handle1.join();
    let result2 = handle2.join();

    match result1 {
        Ok(_) => println!("Thread 1 completed successfully"),
        Err(e) => println!("Error in thread 1: {:?}", e),
    }
    match result2 {
        Ok(_) => println!("Thread 2 completed successfully"),
        Err(e) => println!("Error in thread 2: {:?}", e),
    }
}

fn main() {
    threading();
}
