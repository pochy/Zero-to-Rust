use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Default)]
struct Counts {
    bytes: usize,
    lines: usize,
    words: usize,
}

fn count_file(path: &Path) -> io::Result<Counts> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut counts = Counts::default();

    for line in reader.split(b'\n') {
        let line = line?;
        counts.bytes += line.len() + 1;
        counts.lines += 1;
        counts.words += String::from_utf8_lossy(&line).split_whitespace().count();
    }

    Ok(counts)
}

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: mini_wc <path>");
        std::process::exit(2);
    };

    match count_file(Path::new(&path)) {
        Ok(counts) => {
            println!("bytes: {}", counts.bytes);
            println!("lines: {}", counts.lines);
            println!("words: {}", counts.words);
        }
        Err(error) => {
            eprintln!("mini_wc: {error}");
            std::process::exit(1);
        }
    }
}
