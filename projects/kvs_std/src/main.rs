use std::process::ExitCode;
use std::time::SystemTime;

use kvs_std::{Store, parse_command};

fn main() -> ExitCode {
    let input = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    if input.is_empty() {
        eprintln!("usage: kvs_std <COMMAND>");
        eprintln!("examples:");
        eprintln!("  kvs_std SET name Rust");
        eprintln!("  kvs_std GET name");
        return ExitCode::SUCCESS;
    }

    let command = match parse_command(&input) {
        Ok(command) => command,
        Err(error) => {
            eprintln!("ERROR {}", error);
            return ExitCode::from(2);
        }
    };

    let mut store = Store::default();
    let response = store.execute(command, SystemTime::now());
    println!("{}", response.to_wire());

    ExitCode::SUCCESS
}
