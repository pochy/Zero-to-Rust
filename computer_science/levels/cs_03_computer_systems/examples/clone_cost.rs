use std::time::Instant;

fn owned_work(input: String) -> usize {
    input.bytes().filter(|byte| *byte == b'a').count()
}

fn borrowed_work(input: &str) -> usize {
    input.bytes().filter(|byte| *byte == b'a').count()
}

fn main() {
    let text = "a".repeat(5_000_000);

    let start = Instant::now();
    let owned_count = owned_work(text.clone());
    let owned_elapsed = start.elapsed();

    let start = Instant::now();
    let borrowed_count = borrowed_work(&text);
    let borrowed_elapsed = start.elapsed();

    println!("owned count: {owned_count}");
    println!("owned elapsed: {:?}", owned_elapsed);
    println!("borrowed count: {borrowed_count}");
    println!("borrowed elapsed: {:?}", borrowed_elapsed);
    println!("original still usable: {}", text.len());
}
