use std::env;
use std::error::Error;

use perf_log_rust::{analyze_path, summary_json};

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("usage: perf_log_rust <logs.jsonl>")?;
    let summary = analyze_path(path)?;
    println!("{}", summary_json(&summary)?);
    Ok(())
}
