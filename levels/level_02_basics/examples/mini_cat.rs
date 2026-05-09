use std::env;
use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("usage: mini_cat <path>");
            return Ok(());
        }
    };

    let content = fs::read_to_string(path)?;
    print!("{}", content);

    Ok(())
}
