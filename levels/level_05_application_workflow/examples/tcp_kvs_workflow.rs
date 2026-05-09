use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
}

#[derive(Debug, PartialEq, Eq)]
enum Response {
    Ok,
    Value(String),
    NotFound,
    Error(String),
}

impl Response {
    fn to_wire(&self) -> String {
        match self {
            Response::Ok => "OK".to_string(),
            Response::Value(value) => format!("VALUE {}", value),
            Response::NotFound => "NOT_FOUND".to_string(),
            Response::Error(message) => format!("ERROR {}", message),
        }
    }
}

#[derive(Default)]
struct Store {
    data: HashMap<String, String>,
}

impl Store {
    fn execute(&mut self, command: Command) -> Response {
        match command {
            Command::Set { key, value } => {
                self.data.insert(key, value);
                Response::Ok
            }
            Command::Get { key } => match self.data.get(&key) {
                Some(value) => Response::Value(value.clone()),
                None => Response::NotFound,
            },
            Command::Delete { key } => {
                if self.data.remove(&key).is_some() {
                    Response::Ok
                } else {
                    Response::NotFound
                }
            }
        }
    }
}

fn parse_command(input: &str) -> Result<Command, String> {
    let parts: Vec<&str> = input.trim().splitn(3, ' ').collect();

    match parts.as_slice() {
        ["SET", key, value] if !key.is_empty() => Ok(Command::Set {
            key: (*key).to_string(),
            value: (*value).to_string(),
        }),
        ["GET", key] if !key.is_empty() => Ok(Command::Get {
            key: (*key).to_string(),
        }),
        ["DEL", key] if !key.is_empty() => Ok(Command::Delete {
            key: (*key).to_string(),
        }),
        _ => Err(format!("invalid command: {}", input.trim())),
    }
}

fn handle_line(store: &mut Store, input: &str) -> Response {
    match parse_command(input) {
        Ok(command) => store.execute(command),
        Err(message) => Response::Error(message),
    }
}

fn main() {
    let mut store = Store::default();
    let inputs = [
        "SET name Rust",
        "GET name",
        "GET missing",
        "DEL name",
        "GET name",
    ];

    for input in inputs {
        let response = handle_line(&mut store, input);
        println!("> {}", input);
        println!("< {}", response.to_wire());
    }
}
