use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let jobs = 8;

    let start = Instant::now();
    for _ in 0..jobs {
        thread::sleep(Duration::from_millis(50));
    }
    let sequential = start.elapsed();

    let start = Instant::now();
    let handles: Vec<_> = (0..jobs)
        .map(|job| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(50));
                println!("finished blocking job {job}");
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("worker panicked");
    }
    let threaded = start.elapsed();

    println!("sequential blocking elapsed: {:?}", sequential);
    println!("threaded blocking elapsed: {:?}", threaded);
}
