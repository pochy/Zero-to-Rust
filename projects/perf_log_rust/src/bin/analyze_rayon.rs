use std::env;
use std::error::Error;
use std::fs;
use std::time::Instant;

use perf_log_rust::{Aggregator, summary_json};
use rayon::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("usage: analyze_rayon <logs.jsonl>")?;
    let started = Instant::now();
    let input = fs::read_to_string(path)?;
    let aggregator = input
        .par_lines()
        .fold(Aggregator::default, |mut aggregator, line| {
            aggregator.observe_line(line);
            aggregator
        })
        .reduce(Aggregator::default, |mut left, right| {
            left.merge(right);
            left
        });
    let summary = aggregator.finish(started.elapsed());
    println!("{}", summary_json(&summary)?);
    Ok(())
}
