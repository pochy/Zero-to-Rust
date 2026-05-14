use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn grep(keyword: &str, path: &Path) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut matches = 0;

    for line in reader.lines() {
        let line = line?;
        if line.contains(keyword) {
            println!("{line}");
            matches += 1;
        }
    }

    Ok(matches)
}

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(keyword) = args.next() else {
        eprintln!("usage: mini_grep <keyword> <path>");
        std::process::exit(2);
    };
    let Some(path) = args.next() else {
        eprintln!("usage: mini_grep <keyword> <path>");
        std::process::exit(2);
    };

    match grep(&keyword, Path::new(&path)) {
        Ok(matches) => eprintln!("matches: {matches}"),
        Err(error) => {
            eprintln!("mini_grep: {error}");
            std::process::exit(1);
        }
    }
}
