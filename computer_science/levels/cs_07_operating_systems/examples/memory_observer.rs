use std::thread;
use std::time::Duration;

fn main() {
    let megabytes = std::env::args()
        .nth(1)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(128);

    let bytes = megabytes * 1024 * 1024;
    let mut data = vec![0_u8; bytes];

    for index in (0..data.len()).step_by(4096) {
        data[index] = 1;
    }

    println!("allocated and touched {megabytes} MiB");
    println!("process id: {}", std::process::id());
    println!("sleeping for 3 seconds; inspect this process with ps or top");

    thread::sleep(Duration::from_secs(3));

    let checksum: u64 = data.iter().map(|byte| u64::from(*byte)).sum();
    println!("checksum: {checksum}");
}
