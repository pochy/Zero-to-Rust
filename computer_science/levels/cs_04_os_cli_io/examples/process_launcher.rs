use std::process::Command;

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(program) = args.next() else {
        eprintln!("usage: process_launcher <program> [args...]");
        std::process::exit(2);
    };

    let status = Command::new(&program).args(args).status();

    match status {
        Ok(status) => {
            println!("program: {program}");
            println!("success: {}", status.success());
            println!("status: {status}");
            std::process::exit(status.code().unwrap_or(1));
        }
        Err(error) => {
            eprintln!("process_launcher: failed to start {program}: {error}");
            std::process::exit(1);
        }
    }
}
