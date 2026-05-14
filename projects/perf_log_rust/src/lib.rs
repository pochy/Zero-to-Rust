use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LogEvent {
    pub ip: String,
    pub path: String,
    pub status: u16,
    pub bytes: u64,
    pub latency_ms: u64,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct TopIp {
    pub ip: String,
    pub count: u64,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Summary {
    pub total_lines: u64,
    pub ok_lines: u64,
    pub broken_lines: u64,
    pub total_bytes: u64,
    pub avg_latency_ms: f64,
    pub p95_latency_ms: u64,
    pub status_counts: BTreeMap<String, u64>,
    pub path_counts: BTreeMap<String, u64>,
    pub top_10_ips: Vec<TopIp>,
    pub elapsed_ms: f64,
    pub rows_per_second: f64,
}

#[derive(Debug, Default)]
pub struct Aggregator {
    total_lines: u64,
    broken_lines: u64,
    total_bytes: u64,
    total_latency: u64,
    latencies: Vec<u64>,
    status_counts: HashMap<String, u64>,
    path_counts: HashMap<String, u64>,
    ip_counts: HashMap<String, u64>,
}

impl Aggregator {
    pub fn observe_line(&mut self, line: &str) {
        self.total_lines += 1;
        match serde_json::from_str::<LogEvent>(line) {
            Ok(event) => self.observe_event(event),
            Err(_) => self.broken_lines += 1,
        }
    }

    pub fn observe_event(&mut self, event: LogEvent) {
        *self
            .status_counts
            .entry(event.status.to_string())
            .or_insert(0) += 1;
        *self.path_counts.entry(event.path).or_insert(0) += 1;
        *self.ip_counts.entry(event.ip).or_insert(0) += 1;
        self.total_bytes += event.bytes;
        self.total_latency += event.latency_ms;
        self.latencies.push(event.latency_ms);
    }

    pub fn merge(&mut self, other: Aggregator) {
        self.total_lines += other.total_lines;
        self.broken_lines += other.broken_lines;
        self.total_bytes += other.total_bytes;
        self.total_latency += other.total_latency;
        self.latencies.extend(other.latencies);
        merge_counts(&mut self.status_counts, other.status_counts);
        merge_counts(&mut self.path_counts, other.path_counts);
        merge_counts(&mut self.ip_counts, other.ip_counts);
    }

    pub fn finish(mut self, elapsed: std::time::Duration) -> Summary {
        let ok_lines = self.total_lines - self.broken_lines;
        let elapsed_secs = elapsed.as_secs_f64();
        Summary {
            total_lines: self.total_lines,
            ok_lines,
            broken_lines: self.broken_lines,
            total_bytes: self.total_bytes,
            avg_latency_ms: if ok_lines == 0 {
                0.0
            } else {
                round3(self.total_latency as f64 / ok_lines as f64)
            },
            p95_latency_ms: percentile_95(&mut self.latencies),
            status_counts: to_btree(self.status_counts),
            path_counts: to_btree(self.path_counts),
            top_10_ips: top_ips(self.ip_counts),
            elapsed_ms: round3(elapsed_secs * 1000.0),
            rows_per_second: if elapsed_secs == 0.0 {
                0.0
            } else {
                round3(self.total_lines as f64 / elapsed_secs)
            },
        }
    }
}

pub fn analyze_path(path: impl AsRef<Path>) -> io::Result<Summary> {
    let started = Instant::now();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut aggregator = Aggregator::default();
    for line in reader.lines() {
        aggregator.observe_line(&line?);
    }
    Ok(aggregator.finish(started.elapsed()))
}

pub fn summary_json(summary: &Summary) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(summary)
}

fn merge_counts(target: &mut HashMap<String, u64>, source: HashMap<String, u64>) {
    for (key, value) in source {
        *target.entry(key).or_insert(0) += value;
    }
}

fn to_btree(map: HashMap<String, u64>) -> BTreeMap<String, u64> {
    map.into_iter().collect()
}

fn top_ips(map: HashMap<String, u64>) -> Vec<TopIp> {
    let mut pairs: Vec<_> = map.into_iter().collect();
    pairs.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    pairs
        .into_iter()
        .take(10)
        .map(|(ip, count)| TopIp { ip, count })
        .collect()
}

fn percentile_95(values: &mut [u64]) -> u64 {
    if values.is_empty() {
        return 0;
    }
    values.sort_unstable();
    let index = ((values.len() - 1) as f64 * 0.95) as usize;
    values[index]
}

fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_valid_and_broken_lines() {
        let mut aggregator = Aggregator::default();
        aggregator.observe_line(
            r#"{"ts":"2026-05-14T12:00:00Z","ip":"203.0.113.1","method":"GET","path":"/health","status":200,"bytes":2,"latency_ms":1}"#,
        );
        aggregator.observe_line("not json");
        aggregator.observe_line(
            r#"{"ts":"2026-05-14T12:00:01Z","ip":"203.0.113.1","method":"GET","path":"/health","status":200,"bytes":4,"latency_ms":3}"#,
        );

        let summary = aggregator.finish(std::time::Duration::from_secs(1));

        assert_eq!(summary.total_lines, 3);
        assert_eq!(summary.ok_lines, 2);
        assert_eq!(summary.broken_lines, 1);
        assert_eq!(summary.total_bytes, 6);
        assert_eq!(summary.avg_latency_ms, 2.0);
        assert_eq!(summary.status_counts.get("200"), Some(&2));
        assert_eq!(summary.path_counts.get("/health"), Some(&2));
        assert_eq!(
            summary.top_10_ips,
            vec![TopIp {
                ip: "203.0.113.1".to_string(),
                count: 2
            }]
        );
    }
}
