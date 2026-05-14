use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    let body = format!("received: {}", request_line.trim_end());
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        body.len(),
        body
    );

    stream.write_all(response.as_bytes())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    eprintln!("listening on http://127.0.0.1:7878");
    eprintln!("try: curl -i http://127.0.0.1:7878/hello");

    for stream in listener.incoming().take(1) {
        handle_connection(stream?)?;
    }

    Ok(())
}
