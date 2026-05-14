use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
}

#[derive(Debug, Default)]
struct Store {
    values: HashMap<String, String>,
}

impl Store {
    fn apply(&mut self, command: &Command) -> String {
        match command {
            Command::Set { key, value } => {
                self.values.insert(key.clone(), value.clone());
                "OK".to_string()
            }
            Command::Get { key } => self
                .values
                .get(key)
                .map_or_else(|| "NOT_FOUND".to_string(), |value| format!("VALUE {value}")),
            Command::Delete { key } => {
                if self.values.remove(key).is_some() {
                    "DELETED".to_string()
                } else {
                    "NOT_FOUND".to_string()
                }
            }
        }
    }
}

fn parse_command(line: &str) -> Result<Command, String> {
    let mut parts = line.trim_end().splitn(3, ' ');
    match parts.next() {
        Some("SET") => {
            let key = parts.next().ok_or_else(|| "SET missing key".to_string())?;
            let value = parts
                .next()
                .ok_or_else(|| "SET missing value".to_string())?;
            Ok(Command::Set {
                key: key.to_string(),
                value: value.to_string(),
            })
        }
        Some("GET") => {
            let key = parts.next().ok_or_else(|| "GET missing key".to_string())?;
            Ok(Command::Get {
                key: key.to_string(),
            })
        }
        Some("DELETE") => {
            let key = parts
                .next()
                .ok_or_else(|| "DELETE missing key".to_string())?;
            Ok(Command::Delete {
                key: key.to_string(),
            })
        }
        Some(other) => Err(format!("unknown command: {other}")),
        None => Err("empty command".to_string()),
    }
}

fn write_wal(path: &Path, command: &Command) -> io::Result<()> {
    if matches!(command, Command::Get { .. }) {
        return Ok(());
    }

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    match command {
        Command::Set { key, value } => writeln!(file, "SET {key} {value}"),
        Command::Delete { key } => writeln!(file, "DELETE {key}"),
        Command::Get { .. } => Ok(()),
    }
}

fn restore(path: &Path) -> io::Result<Store> {
    let mut store = Store::default();
    if !path.exists() {
        return Ok(store);
    }

    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if let Ok(command) = parse_command(&line) {
            store.apply(&command);
        }
    }

    Ok(store)
}

fn main() -> io::Result<()> {
    let wal_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp/cs_capstone_kvs.wal"));

    let start = Instant::now();
    let mut store = restore(&wal_path)?;
    eprintln!(
        "restored {} keys from {:?} in {:?}",
        store.values.len(),
        wal_path,
        start.elapsed()
    );
    eprintln!("commands: SET <key> <value>, GET <key>, DELETE <key>");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        match parse_command(&line) {
            Ok(command) => {
                write_wal(&wal_path, &command)?;
                println!("{}", store.apply(&command));
            }
            Err(error) => println!("ERR {error}"),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_command, Command, Store};

    #[test]
    fn parses_commands() {
        assert_eq!(
            parse_command("SET name Rust"),
            Ok(Command::Set {
                key: "name".to_string(),
                value: "Rust".to_string()
            })
        );
        assert_eq!(
            parse_command("GET name"),
            Ok(Command::Get {
                key: "name".to_string()
            })
        );
    }

    #[test]
    fn applies_store_operations() {
        let mut store = Store::default();
        assert_eq!(
            store.apply(&Command::Set {
                key: "a".to_string(),
                value: "1".to_string()
            }),
            "OK"
        );
        assert_eq!(
            store.apply(&Command::Get {
                key: "a".to_string()
            }),
            "VALUE 1"
        );
        assert_eq!(
            store.apply(&Command::Delete {
                key: "a".to_string()
            }),
            "DELETED"
        );
    }
}
