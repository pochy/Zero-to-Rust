use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq)]
struct MatchLine {
    path: PathBuf,
    line_number: usize,
    line: String,
}

fn search_file(path: &Path, pattern: &str) -> Result<Vec<MatchLine>, io::Error> {
    let content = fs::read_to_string(path)?;
    let mut matches = Vec::new();

    for (index, line) in content.lines().enumerate() {
        if line.contains(pattern) {
            matches.push(MatchLine {
                path: path.to_path_buf(),
                line_number: index + 1,
                line: line.to_string(),
            });
        }
    }

    Ok(matches)
}

fn main() -> Result<(), io::Error> {
    let mut args = env::args().skip(1);
    let pattern = match args.next() {
        Some(pattern) => pattern,
        None => {
            eprintln!("usage: mini_grep <pattern> <path>");
            return Ok(());
        }
    };
    let path = match args.next() {
        Some(path) => PathBuf::from(path),
        None => {
            eprintln!("usage: mini_grep <pattern> <path>");
            return Ok(());
        }
    };

    for item in search_file(&path, &pattern)? {
        println!("{}:{}: {}", item.path.display(), item.line_number, item.line);
    }

    Ok(())
}
