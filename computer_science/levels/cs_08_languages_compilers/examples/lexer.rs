#[derive(Debug, PartialEq, Eq)]
enum Token {
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
}

fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.peek().copied() {
        match ch {
            '0'..='9' => {
                let mut number = 0_i64;
                while let Some(digit) = chars.peek().and_then(|candidate| candidate.to_digit(10)) {
                    number = number * 10 + i64::from(digit);
                    chars.next();
                }
                tokens.push(Token::Number(number));
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Star);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Slash);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LeftParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RightParen);
            }
            ch if ch.is_whitespace() => {
                chars.next();
            }
            other => return Err(format!("unexpected character: {other}")),
        }
    }

    Ok(tokens)
}

fn main() {
    for input in ["1 + 2 * 3", "(10 / 2) + 4", "1 + ?"] {
        println!("{input:?} => {:?}", lex(input));
    }
}

#[cfg(test)]
mod tests {
    use super::{lex, Token};

    #[test]
    fn tokenizes_expression() {
        assert_eq!(
            lex("1 + 23").unwrap(),
            vec![Token::Number(1), Token::Plus, Token::Number(23)]
        );
    }

    #[test]
    fn rejects_unknown_character() {
        assert!(lex("1 + ?").is_err());
    }
}
