use std::collections::HashMap;

fn parse_cookie_header(header: &str) -> HashMap<String, String> {
    header
        .split(';')
        .filter_map(|part| {
            let (key, value) = part.trim().split_once('=')?;
            Some((key.to_string(), value.to_string()))
        })
        .collect()
}

fn main() {
    let header = "session_id=abc123; theme=dark; logged_in=true";
    let cookies = parse_cookie_header(header);

    println!("session_id: {:?}", cookies.get("session_id"));
    println!("theme: {:?}", cookies.get("theme"));
    println!("logged_in: {:?}", cookies.get("logged_in"));
}

#[cfg(test)]
mod tests {
    use super::parse_cookie_header;

    #[test]
    fn parses_cookie_pairs() {
        let cookies = parse_cookie_header("session_id=abc123; theme=dark");
        assert_eq!(
            cookies.get("session_id").map(String::as_str),
            Some("abc123")
        );
        assert_eq!(cookies.get("theme").map(String::as_str), Some("dark"));
    }
}
