use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

use final_kvs_server::{AppState, run_admin_server, run_tcp_server};

fn main() -> std::io::Result<()> {
    let app_addr = std::env::var("APP_ADDR").unwrap_or_else(|_| "127.0.0.1:4000".to_string());
    let admin_addr = std::env::var("ADMIN_ADDR").unwrap_or_else(|_| "127.0.0.1:4001".to_string());
    let wal_path =
        std::env::var("WAL_PATH").unwrap_or_else(|_| "target/final_kvs_server.wal".to_string());

    let state = Arc::new(Mutex::new(AppState::restore(wal_path, SystemTime::now())?));
    let admin_state = Arc::clone(&state);
    let admin_addr_for_thread = admin_addr.clone();

    thread::spawn(move || {
        if let Err(error) = run_admin_server(&admin_addr_for_thread, admin_state) {
            eprintln!("admin server error: {}", error);
        }
    });

    eprintln!("tcp server listening on {}", app_addr);
    eprintln!("admin server listening on {}", admin_addr);
    run_tcp_server(&app_addr, state)
}
