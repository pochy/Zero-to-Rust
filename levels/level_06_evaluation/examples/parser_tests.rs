#[derive(Debug, PartialEq, Eq)]
enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    Quit,
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
        ["QUIT"] => Ok(Command::Quit),
        _ => Err(format!("invalid command: {}", input.trim())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_set_command() {
        let command = parse_command("SET name Rust").unwrap();

        assert_eq!(
            command,
            Command::Set {
                key: "name".to_string(),
                value: "Rust".to_string()
            }
        );
    }

    #[test]
    fn parse_get_command() {
        let command = parse_command("GET name").unwrap();

        assert_eq!(
            command,
            Command::Get {
                key: "name".to_string()
            }
        );
    }

    #[test]
    fn parse_delete_command() {
        let command = parse_command("DEL name").unwrap();

        assert_eq!(
            command,
            Command::Delete {
                key: "name".to_string()
            }
        );
    }

    #[test]
    fn parse_quit_command() {
        assert_eq!(parse_command("QUIT").unwrap(), Command::Quit);
    }

    #[test]
    fn reject_missing_key() {
        assert!(parse_command("GET").is_err());
    }

    #[test]
    fn reject_unknown_command() {
        assert!(parse_command("UNKNOWN name").is_err());
    }
}
