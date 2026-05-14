use std::fs;
use std::io;
use std::path::Path;

fn inspect(path: &Path) -> io::Result<()> {
    let bytes = fs::read(path)?;
    println!("byte length: {}", bytes.len());
    println!("first bytes: {:?}", &bytes[..bytes.len().min(8)]);

    match fs::read_to_string(path) {
        Ok(text) => {
            println!("text length: {}", text.len());
            println!("first line: {:?}", text.lines().next().unwrap_or(""));
        }
        Err(error) => {
            println!("not valid UTF-8 text: {error}");
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "START_HERE.md".to_string());

    inspect(Path::new(&path))
}
