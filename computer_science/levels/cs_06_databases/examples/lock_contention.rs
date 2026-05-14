use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let counter = Arc::new(Mutex::new(0_u64));
    let mut handles = Vec::new();

    for worker_id in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            let mut guard = counter.lock().expect("lock poisoned");
            let waited = start.elapsed();

            thread::sleep(Duration::from_millis(50));
            *guard += 1;
            println!("worker {worker_id} waited {:?}", waited);
        }));
    }

    for handle in handles {
        handle.join().expect("worker panicked");
    }

    println!("final counter: {}", *counter.lock().expect("lock poisoned"));
}
