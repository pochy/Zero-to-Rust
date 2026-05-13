use std::collections::HashMap;
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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

impl Command {
    fn mutates_state(&self) -> bool {
        matches!(self, Command::Set { .. } | Command::Delete { .. })
    }
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
    TooManyParts,
    UnknownCommand(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "empty command"),
            ParseError::MissingKey => write!(f, "missing key"),
            ParseError::MissingValue => write!(f, "missing value"),
            ParseError::InvalidTtl(value) => write!(f, "invalid ttl: {}", value),
            ParseError::TooManyParts => write!(f, "too many parts"),
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

    fn set(&mut self, key: String, value: String, ttl_secs: Option<u64>, now: SystemTime) {
        let expires_at = ttl_secs.and_then(|seconds| now.checked_add(Duration::from_secs(seconds)));
        self.data.insert(key, Entry { value, expires_at });
    }

    fn get(&mut self, key: &str, now: SystemTime) -> Option<&str> {
        self.remove_if_expired(key, now);
        self.data.get(key).map(|entry| entry.value.as_str())
    }

    fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    fn exists(&mut self, key: &str, now: SystemTime) -> bool {
        self.remove_if_expired(key, now);
        self.data.contains_key(key)
    }

    fn ttl(&mut self, key: &str, now: SystemTime) -> Option<Option<u64>> {
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

    fn keys(&mut self, now: SystemTime) -> Vec<String> {
        let keys: Vec<String> = self.data.keys().cloned().collect();
        for key in &keys {
            self.remove_if_expired(key, now);
        }
        let mut keys: Vec<String> = self.data.keys().cloned().collect();
        keys.sort();
        keys
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

#[derive(Debug)]
pub struct Metrics {
    started_at: SystemTime,
    commands: u64,
    errors: u64,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            started_at: SystemTime::now(),
            commands: 0,
            errors: 0,
        }
    }
}

#[derive(Debug)]
pub struct AppState {
    store: Store,
    metrics: Metrics,
    wal_path: PathBuf,
}

pub type SharedState = Arc<Mutex<AppState>>;

impl AppState {
    pub fn new(wal_path: impl Into<PathBuf>) -> Self {
        Self {
            store: Store::default(),
            metrics: Metrics::default(),
            wal_path: wal_path.into(),
        }
    }

    pub fn restore(wal_path: impl Into<PathBuf>, now: SystemTime) -> io::Result<Self> {
        let wal_path = wal_path.into();
        let store = restore_store_from_wal(&wal_path, now)?;
        Ok(Self {
            store,
            metrics: Metrics::default(),
            wal_path,
        })
    }

    fn handle_command(&mut self, command: Command, now: SystemTime) -> Response {
        if command.mutates_state()
            && let Err(error) = append_wal_command(&self.wal_path, &command)
        {
            self.metrics.errors += 1;
            return Response::Error(format!("wal write failed: {}", error));
        }

        self.metrics.commands += 1;
        self.store.execute(command, now)
    }

    fn render_metrics(&self) -> String {
        let started = self
            .metrics
            .started_at
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or_default();
        format!(
            "commands {}\nerrors {}\nstarted_at_unix {}\n",
            self.metrics.commands, self.metrics.errors, started
        )
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
        "GET" => parse_key_command(trimmed, |key| Command::Get { key }),
        "DEL" => parse_key_command(trimmed, |key| Command::Delete { key }),
        "EXISTS" => parse_key_command(trimmed, |key| Command::Exists { key }),
        "TTL" => parse_key_command(trimmed, |key| Command::Ttl { key }),
        "QUIT" => Ok(Command::Quit),
        other => Err(ParseError::UnknownCommand(other.to_string())),
    }
}

fn parse_set(input: &str) -> Result<Command, ParseError> {
    let mut parts = input.splitn(3, char::is_whitespace);
    let _ = parts.next();
    let key = parts.next().ok_or(ParseError::MissingKey)?;
    let value = parts.next().ok_or(ParseError::MissingValue)?;
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
    build: impl FnOnce(String) -> Command,
) -> Result<Command, ParseError> {
    let mut parts = input.split_whitespace();
    let _ = parts.next();
    let key = parts.next().ok_or(ParseError::MissingKey)?;
    if parts.next().is_some() {
        return Err(ParseError::TooManyParts);
    }
    Ok(build(key.to_string()))
}

pub fn process_line(state: &SharedState, line: &str, now: SystemTime) -> Response {
    let command = match parse_command(line) {
        Ok(command) => command,
        Err(error) => {
            if let Ok(mut state) = state.lock() {
                state.metrics.errors += 1;
            }
            return Response::Error(error.to_string());
        }
    };

    match state.lock() {
        Ok(mut state) => state.handle_command(command, now),
        Err(_) => Response::Error("state lock poisoned".to_string()),
    }
}

pub fn append_wal_command(path: &Path, command: &Command) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    write_wal_command(&mut file, command)
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

pub fn restore_store_from_wal(path: &Path, now: SystemTime) -> io::Result<Store> {
    if !path.exists() {
        return Ok(Store::default());
    }

    let file = OpenOptions::new().read(true).open(path)?;
    let mut store = Store::default();
    for line in BufReader::new(file).lines() {
        let line = line?;
        let command = parse_command(&line)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        if command.mutates_state() {
            let _ = store.execute(command, now);
        }
    }
    Ok(store)
}

pub fn run_tcp_server(addr: &str, state: SharedState) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        let stream = stream?;
        let state = Arc::clone(&state);
        thread::spawn(move || {
            if let Err(error) = handle_client(stream, state) {
                eprintln!("client error: {}", error);
            }
        });
    }
    Ok(())
}

pub fn handle_client(mut stream: TcpStream, state: SharedState) -> io::Result<()> {
    let reader_stream = stream.try_clone()?;
    let reader = BufReader::new(reader_stream);

    for line in reader.lines() {
        let line = line?;
        let response = process_line(&state, &line, SystemTime::now());
        writeln!(stream, "{}", response.to_wire())?;
        if response == Response::Bye {
            break;
        }
    }

    Ok(())
}

pub fn run_admin_server(addr: &str, state: SharedState) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        let response = handle_admin_request(&mut stream, &state)?;
        stream.write_all(response.as_bytes())?;
    }
    Ok(())
}

pub fn handle_admin_request(stream: &mut impl Read, state: &SharedState) -> io::Result<String> {
    let mut buffer = [0_u8; 1024];
    let bytes = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..bytes]);
    let first_line = request.lines().next().unwrap_or_default();
    let path = first_line.split_whitespace().nth(1).unwrap_or("/");

    let body = match path {
        "/health" => "ok\n".to_string(),
        "/metrics" => match state.lock() {
            Ok(state) => state.render_metrics(),
            Err(_) => "errors 1\n".to_string(),
        },
        "/keys" => match state.lock() {
            Ok(mut state) => {
                let keys = state.store.keys(SystemTime::now());
                format!("{}\n", keys.join("\n"))
            }
            Err(_) => "state lock poisoned\n".to_string(),
        },
        _ => "not found\n".to_string(),
    };

    let status = if path == "/health" || path == "/metrics" || path == "/keys" {
        "200 OK"
    } else {
        "404 Not Found"
    };

    Ok(format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n{}",
        status,
        body.len(),
        body
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn test_state() -> SharedState {
        let path = std::env::temp_dir().join(format!(
            "zero_to_rust_final_{}.wal",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        Arc::new(Mutex::new(AppState::new(path)))
    }

    #[test]
    fn parses_setex_with_value_containing_spaces() {
        assert_eq!(
            parse_command("SETEX token 10 abc def").unwrap(),
            Command::Set {
                key: "token".to_string(),
                value: "abc def".to_string(),
                ttl_secs: Some(10)
            }
        );
    }

    #[test]
    fn set_get_and_delete_flow() {
        let state = test_state();
        let now = UNIX_EPOCH;

        assert_eq!(process_line(&state, "SET name Rust", now), Response::Ok);
        assert_eq!(
            process_line(&state, "GET name", now),
            Response::Value("Rust".to_string())
        );
        assert_eq!(process_line(&state, "DEL name", now), Response::Ok);
        assert_eq!(process_line(&state, "GET name", now), Response::NotFound);
    }

    #[test]
    fn ttl_expires_lazily() {
        let state = test_state();
        let now = UNIX_EPOCH;
        let later = now + Duration::from_secs(2);

        assert_eq!(process_line(&state, "SETEX token 1 abc", now), Response::Ok);
        assert_eq!(process_line(&state, "GET token", later), Response::NotFound);
    }

    #[test]
    fn wal_replays_mutations() {
        let path = std::env::temp_dir().join(format!(
            "zero_to_rust_final_replay_{}.wal",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        append_wal_command(
            &path,
            &Command::Set {
                key: "name".to_string(),
                value: "Rust".to_string(),
                ttl_secs: None,
            },
        )
        .unwrap();
        append_wal_command(
            &path,
            &Command::Delete {
                key: "missing".to_string(),
            },
        )
        .unwrap();

        let mut store = restore_store_from_wal(&path, UNIX_EPOCH).unwrap();
        assert_eq!(
            store.execute(
                Command::Get {
                    key: "name".to_string()
                },
                UNIX_EPOCH
            ),
            Response::Value("Rust".to_string())
        );

        let _ = fs::remove_file(path);
    }

    #[test]
    fn admin_health_response_is_http() {
        let state = test_state();
        let request = b"GET /health HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let response = handle_admin_request(&mut Cursor::new(request), &state).unwrap();

        assert!(response.starts_with("HTTP/1.1 200 OK"));
        assert!(response.ends_with("ok\n"));
    }
}
