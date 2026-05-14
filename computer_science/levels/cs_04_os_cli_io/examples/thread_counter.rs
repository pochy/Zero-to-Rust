use std::thread;

fn main() {
    let chunks = vec![
        vec![1_u64, 2, 3, 4],
        vec![5_u64, 6, 7, 8],
        vec![9_u64, 10, 11, 12],
    ];

    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| {
            thread::spawn(move || {
                let sum: u64 = chunk.iter().sum();
                println!("partial sum: {sum}");
                sum
            })
        })
        .collect();

    let total: u64 = handles
        .into_iter()
        .map(|handle| handle.join().expect("worker panicked"))
        .sum();

    println!("total: {total}");
}
