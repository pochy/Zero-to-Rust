#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq, Eq)]
enum Expr {
    Number(i64),
    Binary {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
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

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_mul_div()?;

        loop {
            let op = match self.peek() {
                Some(Token::Plus) => Op::Add,
                Some(Token::Minus) => Op::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_mul_div()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_mul_div(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        loop {
            let op = match self.peek() {
                Some(Token::Star) => Op::Mul,
                Some(Token::Slash) => Op::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_primary()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Some(Token::Number(value)) => Ok(Expr::Number(value)),
            Some(Token::LeftParen) => {
                let expr = self.parse_expression()?;
                match self.advance() {
                    Some(Token::RightParen) => Ok(expr),
                    _ => Err("missing ')'".to_string()),
                }
            }
            Some(token) => Err(format!("expected expression, got {token:?}")),
            None => Err("expected expression, got end of input".to_string()),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        self.position += usize::from(token.is_some());
        token
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }
}

fn parse(input: &str) -> Result<Expr, String> {
    let tokens = lex(input)?;
    let mut parser = Parser::new(tokens);
    let expr = parser.parse_expression()?;
    if !parser.is_at_end() {
        return Err("unexpected trailing token".to_string());
    }
    Ok(expr)
}

fn eval(expr: &Expr) -> Result<i64, String> {
    match expr {
        Expr::Number(value) => Ok(*value),
        Expr::Binary { op, left, right } => {
            let left = eval(left)?;
            let right = eval(right)?;
            match op {
                Op::Add => Ok(left + right),
                Op::Sub => Ok(left - right),
                Op::Mul => Ok(left * right),
                Op::Div if right == 0 => Err("division by zero".to_string()),
                Op::Div => Ok(left / right),
            }
        }
    }
}

fn run(input: &str) -> Result<i64, String> {
    let expr = parse(input)?;
    eval(&expr)
}

fn main() {
    for input in ["1 + 2 * 3", "(1 + 2) * 3", "10 / 2 + 4", "10 / 0"] {
        println!("{input} => {:?}", run(input));
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn respects_precedence() {
        assert_eq!(run("1 + 2 * 3"), Ok(7));
        assert_eq!(run("(1 + 2) * 3"), Ok(9));
        assert_eq!(run("10 / 2 + 4"), Ok(9));
    }

    #[test]
    fn reports_runtime_error() {
        assert!(run("10 / 0").is_err());
    }
}
