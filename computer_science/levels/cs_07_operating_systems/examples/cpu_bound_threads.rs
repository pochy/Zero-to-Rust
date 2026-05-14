use std::thread;
use std::time::Instant;

fn count_primes_until(limit: u64) -> u64 {
    (2..=limit).filter(|number| is_prime(*number)).count() as u64
}

fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }
    let mut divisor = 2;
    while divisor * divisor <= number {
        if number % divisor == 0 {
            return false;
        }
        divisor += 1;
    }
    true
}

fn main() {
    let limit = 40_000;

    let start = Instant::now();
    let single = count_primes_until(limit);
    let single_elapsed = start.elapsed();

    let start = Instant::now();
    let handles: Vec<_> = (0..4)
        .map(|worker| {
            thread::spawn(move || {
                let start = worker * limit / 4 + 2;
                let end = (worker + 1) * limit / 4 + 1;
                (start..=end).filter(|number| is_prime(*number)).count() as u64
            })
        })
        .collect();

    let parallel: u64 = handles
        .into_iter()
        .map(|handle| handle.join().expect("worker panicked"))
        .sum();
    let parallel_elapsed = start.elapsed();

    println!("single count: {single}");
    println!("single elapsed: {:?}", single_elapsed);
    println!("parallel count: {parallel}");
    println!("parallel elapsed: {:?}", parallel_elapsed);
}
