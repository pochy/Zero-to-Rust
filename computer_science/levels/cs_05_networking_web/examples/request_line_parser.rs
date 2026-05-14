#[derive(Debug, PartialEq, Eq)]
struct RequestLine<'a> {
    method: &'a str,
    path: &'a str,
    version: &'a str,
}

fn parse_request_line(input: &str) -> Result<RequestLine<'_>, String> {
    let mut parts = input.split_whitespace();
    let method = parts.next().ok_or_else(|| "missing method".to_string())?;
    let path = parts.next().ok_or_else(|| "missing path".to_string())?;
    let version = parts.next().ok_or_else(|| "missing version".to_string())?;

    if parts.next().is_some() {
        return Err("too many parts".to_string());
    }
    if !version.starts_with("HTTP/") {
        return Err("invalid HTTP version".to_string());
    }

    Ok(RequestLine {
        method,
        path,
        version,
    })
}

fn main() {
    for line in ["GET /hello HTTP/1.1", "POST /shorten HTTP/1.1", "BROKEN"] {
        println!("{line:?} => {:?}", parse_request_line(line));
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_request_line, RequestLine};

    #[test]
    fn parses_valid_request_line() {
        assert_eq!(
            parse_request_line("GET /hello HTTP/1.1"),
            Ok(RequestLine {
                method: "GET",
                path: "/hello",
                version: "HTTP/1.1"
            })
        );
    }

    #[test]
    fn rejects_invalid_request_line() {
        assert!(parse_request_line("GET /hello").is_err());
        assert!(parse_request_line("GET /hello FTP/1.0").is_err());
        assert!(parse_request_line("GET /hello HTTP/1.1 extra").is_err());
    }
}
