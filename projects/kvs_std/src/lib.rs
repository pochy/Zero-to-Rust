use std::collections::HashMap;
use std::fmt;
use std::io::{self, BufRead, Write};
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Set {
        key: String,
        value: String,
        ttl_secs: Option<u64>,
    },
    Get {
        key: String,
    },
    Delete {
        key: String,
    },
    Exists {
        key: String,
    },
    Ttl {
        key: String,
    },
    Quit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Ok,
    Value(String),
    NotFound,
    Exists(bool),
    Ttl(Option<u64>),
    Error(String),
    Bye,
}

impl Response {
    pub fn to_wire(&self) -> String {
        match self {
            Response::Ok => "OK".to_string(),
            Response::Value(value) => format!("VALUE {}", value),
            Response::NotFound => "NOT_FOUND".to_string(),
            Response::Exists(true) => "TRUE".to_string(),
            Response::Exists(false) => "FALSE".to_string(),
            Response::Ttl(Some(seconds)) => format!("TTL {}", seconds),
            Response::Ttl(None) => "TTL none".to_string(),
            Response::Error(message) => format!("ERROR {}", message),
            Response::Bye => "BYE".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    MissingKey,
    MissingValue,
    InvalidTtl(String),
    UnknownCommand(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "empty command"),
            ParseError::MissingKey => write!(f, "missing key"),
            ParseError::MissingValue => write!(f, "missing value"),
            ParseError::InvalidTtl(value) => write!(f, "invalid ttl: {}", value),
            ParseError::UnknownCommand(value) => write!(f, "unknown command: {}", value),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone)]
struct Entry {
    value: String,
    expires_at: Option<SystemTime>,
}

#[derive(Debug, Default)]
pub struct Store {
    data: HashMap<String, Entry>,
}

impl Store {
    pub fn set(&mut self, key: String, value: String, ttl_secs: Option<u64>, now: SystemTime) {
        let expires_at = ttl_secs.and_then(|seconds| now.checked_add(Duration::from_secs(seconds)));
        self.data.insert(key, Entry { value, expires_at });
    }

    pub fn get(&mut self, key: &str, now: SystemTime) -> Option<&str> {
        self.remove_if_expired(key, now);
        self.data.get(key).map(|entry| entry.value.as_str())
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    pub fn exists(&mut self, key: &str, now: SystemTime) -> bool {
        self.remove_if_expired(key, now);
        self.data.contains_key(key)
    }

    pub fn ttl(&mut self, key: &str, now: SystemTime) -> Option<Option<u64>> {
        self.remove_if_expired(key, now);
        let entry = self.data.get(key)?;
        Some(match entry.expires_at {
            Some(expires_at) => expires_at
                .duration_since(now)
                .ok()
                .map(|duration| duration.as_secs()),
            None => None,
        })
    }

    pub fn execute(&mut self, command: Command, now: SystemTime) -> Response {
        match command {
            Command::Set {
                key,
                value,
                ttl_secs,
            } => {
                self.set(key, value, ttl_secs, now);
                Response::Ok
            }
            Command::Get { key } => match self.get(&key, now) {
                Some(value) => Response::Value(value.to_string()),
                None => Response::NotFound,
            },
            Command::Delete { key } => {
                if self.delete(&key) {
                    Response::Ok
                } else {
                    Response::NotFound
                }
            }
            Command::Exists { key } => Response::Exists(self.exists(&key, now)),
            Command::Ttl { key } => match self.ttl(&key, now) {
                Some(ttl) => Response::Ttl(ttl),
                None => Response::NotFound,
            },
            Command::Quit => Response::Bye,
        }
    }

    fn remove_if_expired(&mut self, key: &str, now: SystemTime) {
        let expired = self
            .data
            .get(key)
            .and_then(|entry| entry.expires_at)
            .is_some_and(|expires_at| expires_at <= now);

        if expired {
            self.data.remove(key);
        }
    }
}

pub fn parse_command(input: &str) -> Result<Command, ParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ParseError::Empty);
    }

    let command = trimmed.split_whitespace().next().unwrap_or_default();
    match command {
        "SET" => parse_set(trimmed),
        "SETEX" => parse_setex(trimmed),
        "GET" => parse_key_command(trimmed, "GET", |key| Command::Get { key }),
        "DEL" => parse_key_command(trimmed, "DEL", |key| Command::Delete { key }),
        "EXISTS" => parse_key_command(trimmed, "EXISTS", |key| Command::Exists { key }),
        "TTL" => parse_key_command(trimmed, "TTL", |key| Command::Ttl { key }),
        "QUIT" => Ok(Command::Quit),
        other => Err(ParseError::UnknownCommand(other.to_string())),
    }
}

fn parse_set(input: &str) -> Result<Command, ParseError> {
    let mut parts = input.splitn(3, char::is_whitespace);
    let _ = parts.next();
    let key = parts.next().ok_or(ParseError::MissingKey)?;
    let value = parts.next().ok_or(ParseError::MissingValue)?;
    if key.is_empty() {
        return Err(ParseError::MissingKey);
    }
    if value.is_empty() {
        return Err(ParseError::MissingValue);
    }
    Ok(Command::Set {
        key: key.to_string(),
        value: value.to_string(),
        ttl_secs: None,
    })
}

fn parse_setex(input: &str) -> Result<Command, ParseError> {
    let mut parts = input.splitn(4, char::is_whitespace);
    let _ = parts.next();
    let key = parts.next().ok_or(ParseError::MissingKey)?;
    let ttl_text = parts.next().ok_or(ParseError::InvalidTtl(String::new()))?;
    let value = parts.next().ok_or(ParseError::MissingValue)?;
    let ttl_secs = ttl_text
        .parse::<u64>()
        .map_err(|_| ParseError::InvalidTtl(ttl_text.to_string()))?;

    Ok(Command::Set {
        key: key.to_string(),
        value: value.to_string(),
        ttl_secs: Some(ttl_secs),
    })
}

fn parse_key_command(
    input: &str,
    expected: &str,
    build: impl FnOnce(String) -> Command,
) -> Result<Command, ParseError> {
    let mut parts = input.split_whitespace();
    let _ = parts.next();
    let key = parts.next().ok_or(ParseError::MissingKey)?;
    if parts.next().is_some() {
        return Err(ParseError::UnknownCommand(expected.to_string()));
    }
    Ok(build(key.to_string()))
}

pub fn write_wal_command(writer: &mut impl Write, command: &Command) -> io::Result<()> {
    match command {
        Command::Set {
            key,
            value,
            ttl_secs: None,
        } => writeln!(writer, "SET {} {}", key, value),
        Command::Set {
            key,
            value,
            ttl_secs: Some(ttl_secs),
        } => writeln!(writer, "SETEX {} {} {}", key, ttl_secs, value),
        Command::Delete { key } => writeln!(writer, "DEL {}", key),
        Command::Get { .. } | Command::Exists { .. } | Command::Ttl { .. } | Command::Quit => {
            Ok(())
        }
    }
}

pub fn restore_from_wal(reader: impl BufRead, now: SystemTime) -> io::Result<Store> {
    let mut store = Store::default();

    for line in reader.lines() {
        let line = line?;
        let command = parse_command(&line)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        match command {
            Command::Set { .. } | Command::Delete { .. } => {
                let _ = store.execute(command, now);
            }
            Command::Get { .. } | Command::Exists { .. } | Command::Ttl { .. } | Command::Quit => {}
        }
    }

    Ok(store)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parses_set_with_value_containing_spaces() {
        assert_eq!(
            parse_command("SET name Rust language").unwrap(),
            Command::Set {
                key: "name".to_string(),
                value: "Rust language".to_string(),
                ttl_secs: None
            }
        );
    }

    #[test]
    fn stores_and_reads_values() {
        let now = SystemTime::UNIX_EPOCH;
        let mut store = Store::default();

        let response = store.execute(
            Command::Set {
                key: "name".to_string(),
                value: "Rust".to_string(),
                ttl_secs: None,
            },
            now,
        );

        assert_eq!(response, Response::Ok);
        assert_eq!(
            store.execute(
                Command::Get {
                    key: "name".to_string()
                },
                now
            ),
            Response::Value("Rust".to_string())
        );
    }

    #[test]
    fn expires_values_lazily() {
        let now = SystemTime::UNIX_EPOCH;
        let later = now + Duration::from_secs(2);
        let mut store = Store::default();

        store.execute(
            Command::Set {
                key: "token".to_string(),
                value: "abc".to_string(),
                ttl_secs: Some(1),
            },
            now,
        );

        assert_eq!(
            store.execute(
                Command::Get {
                    key: "token".to_string()
                },
                later
            ),
            Response::NotFound
        );
    }

    #[test]
    fn wal_replays_state_changes() {
        let now = SystemTime::UNIX_EPOCH;
        let content = b"SET name Rust\nSET lang std\nDEL lang\n";
        let mut store = restore_from_wal(Cursor::new(content), now).unwrap();

        assert_eq!(
            store.execute(
                Command::Get {
                    key: "name".to_string()
                },
                now
            ),
            Response::Value("Rust".to_string())
        );
        assert_eq!(
            store.execute(
                Command::Get {
                    key: "lang".to_string()
                },
                now
            ),
            Response::NotFound
        );
    }

    #[test]
    fn wal_ignores_read_commands_when_writing() {
        let mut output = Vec::new();
        write_wal_command(
            &mut output,
            &Command::Get {
                key: "name".to_string(),
            },
        )
        .unwrap();

        assert!(output.is_empty());
    }
}
