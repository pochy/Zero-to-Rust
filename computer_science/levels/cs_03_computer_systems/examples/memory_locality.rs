use std::collections::LinkedList;
use std::time::Instant;

fn sum_vec(values: &[u64]) -> u64 {
    values.iter().copied().sum()
}

fn sum_linked_list(values: &LinkedList<u64>) -> u64 {
    values.iter().copied().sum()
}

fn main() {
    let count = 1_000_000;
    let vec_values: Vec<u64> = (0..count).collect();
    let list_values: LinkedList<u64> = (0..count).collect();

    let start = Instant::now();
    let vec_sum = sum_vec(&vec_values);
    let vec_elapsed = start.elapsed();

    let start = Instant::now();
    let list_sum = sum_linked_list(&list_values);
    let list_elapsed = start.elapsed();

    println!("vec sum: {vec_sum}");
    println!("vec elapsed: {:?}", vec_elapsed);
    println!("linked list sum: {list_sum}");
    println!("linked list elapsed: {:?}", list_elapsed);
}
