use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone)]
struct UrlRow {
    id: u64,
    short_code: String,
    original_url: String,
}

fn scan<'a>(rows: &'a [UrlRow], short_code: &str) -> Option<&'a UrlRow> {
    rows.iter().find(|row| row.short_code == short_code)
}

fn main() {
    let rows: Vec<UrlRow> = (0..100_000)
        .map(|id| UrlRow {
            id,
            short_code: format!("code{id}"),
            original_url: format!("https://example.com/{id}"),
        })
        .collect();

    let index: HashMap<String, usize> = rows
        .iter()
        .enumerate()
        .map(|(position, row)| (row.short_code.clone(), position))
        .collect();

    let target = "code99999";

    let start = Instant::now();
    let scanned = scan(&rows, target);
    let scan_elapsed = start.elapsed();

    let start = Instant::now();
    let indexed = index.get(target).map(|position| &rows[*position]);
    let index_elapsed = start.elapsed();

    println!("scan found: {:?}", scanned.map(|row| row.id));
    println!("scan elapsed: {:?}", scan_elapsed);
    println!("index found: {:?}", indexed.map(|row| row.id));
    println!("index elapsed: {:?}", index_elapsed);
    println!(
        "indexed url: {:?}",
        indexed.map(|row| row.original_url.as_str())
    );
}
