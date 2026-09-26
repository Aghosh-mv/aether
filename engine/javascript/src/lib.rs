#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    String(String),
    True,
    False,
    Null,
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    Less,
    Greater,
    EqualEqual,
    LParen,
    RParen,
    End,
}

fn lex(source: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;
    let mut tokens = Vec::new();
    while i < chars.len() {
        match chars[i] {
            c if c.is_whitespace() => i += 1,
            c if c.is_ascii_digit() => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                tokens.push(Token::Number(
                    source[chars[..start].iter().map(|c| c.len_utf8()).sum()
                        ..chars[..i].iter().map(|c| c.len_utf8()).sum()]
                        .parse()
                        .map_err(|_| "invalid number")?,
                ));
            }
            '\'' | '"' => {
                let quote = chars[i];
                i += 1;
                let start = i;
                while i < chars.len() && chars[i] != quote {
                    i += 1;
                }
                if i == chars.len() {
                    return Err("unterminated string".into());
                }
                tokens.push(Token::String(chars[start..i].iter().collect()));
                i += 1;
            }
            '+' => {
                tokens.push(Token::Plus);
                i += 1
            }
            '-' => {
                tokens.push(Token::Minus);
                i += 1
            }
            '*' => {
                tokens.push(Token::Star);
                i += 1
            }
            '/' => {
                tokens.push(Token::Slash);
                i += 1
            }
            '!' => {
                tokens.push(Token::Bang);
                i += 1
            }
            '<' => {
                tokens.push(Token::Less);
                i += 1
            }
            '>' => {
                tokens.push(Token::Greater);
                i += 1
            }
            '=' if chars.get(i + 1) == Some(&'=') => {
                tokens.push(Token::EqualEqual);
                i += 2
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1
            }
            _ => {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_alphabetic() {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                tokens.push(match word.as_str() {
                    "true" => Token::True,
                    "false" => Token::False,
                    "null" => Token::Null,
                    _ => return Err(format!("unexpected identifier: {word}")),
                });
            }
        }
    }
    tokens.push(Token::End);
    Ok(tokens)
}

pub fn evaluate(source: &str) -> Result<Value, String> {
    let tokens = lex(source)?;
    let mut parser = Parser {
        tokens,
        position: 0,
    };
    let value = parser.expression()?;
    if parser.peek() != &Token::End {
        return Err("unexpected trailing input".into());
    }
    Ok(value)
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}
impl Parser {
    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }
    fn take(&mut self) -> Token {
        let token = self.tokens[self.position].clone();
        self.position += 1;
        token
    }
    fn expression(&mut self) -> Result<Value, String> {
        let mut value = self.term()?;
        loop {
            value = match self.peek() {
                Token::Plus => {
                    self.take();
                    add(value, self.term()?)?
                }
                Token::Minus => {
                    self.take();
                    number_op(value, self.term()?, |a, b| a - b)?
                }
                Token::Less => {
                    self.take();
                    compare(value, self.term()?, |a, b| a < b)?
                }
                Token::Greater => {
                    self.take();
                    compare(value, self.term()?, |a, b| a > b)?
                }
                Token::EqualEqual => {
                    self.take();
                    Value::Boolean(value == self.term()?)
                }
                _ => break,
            };
        }
        Ok(value)
    }
    fn term(&mut self) -> Result<Value, String> {
        let mut value = self.primary()?;
        loop {
            value = match self.peek() {
                Token::Star => {
                    self.take();
                    number_op(value, self.primary()?, |a, b| a * b)?
                }
                Token::Slash => {
                    self.take();
                    number_op(value, self.primary()?, |a, b| a / b)?
                }
                _ => break,
            };
        }
        Ok(value)
    }
    fn primary(&mut self) -> Result<Value, String> {
        match self.take() {
            Token::Minus => number_op(Value::Number(0.0), self.primary()?, |a, b| a - b),
            Token::Bang => Ok(Value::Boolean(!truthy(&self.primary()?))),
            Token::Number(n) => Ok(Value::Number(n)),
            Token::String(s) => Ok(Value::String(s)),
            Token::True => Ok(Value::Boolean(true)),
            Token::False => Ok(Value::Boolean(false)),
            Token::Null => Ok(Value::Null),
            Token::LParen => {
                let value = self.expression()?;
                if self.take() != Token::RParen {
                    return Err("expected ')'".into());
                }
                Ok(value)
            }
            _ => Err("expected expression".into()),
        }
    }
}
fn number_op(
    left: Value,
    right: Value,
    operation: impl FnOnce(f64, f64) -> f64,
) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(operation(a, b))),
        _ => Err("arithmetic requires numbers".into()),
    }
}
fn add(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
        (Value::String(a), Value::String(b)) => Ok(Value::String(a + b.as_str())),
        _ => Err("+ requires two numbers or two strings".into()),
    }
}
fn compare(
    left: Value,
    right: Value,
    operation: impl FnOnce(f64, f64) -> bool,
) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(operation(a, b))),
        _ => Err("comparison requires numbers".into()),
    }
}
fn truthy(value: &Value) -> bool {
    match value {
        Value::Boolean(value) => *value,
        Value::Null => false,
        Value::Number(value) => *value != 0.0,
        Value::String(value) => !value.is_empty(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn observes_precedence_and_parentheses() {
        assert_eq!(evaluate("2 + 3 * 4"), Ok(Value::Number(14.0)));
        assert_eq!(evaluate("(2 + 3) * 4"), Ok(Value::Number(20.0)));
    }
    #[test]
    fn evaluates_literals_and_strings() {
        assert_eq!(evaluate("true"), Ok(Value::Boolean(true)));
        assert_eq!(evaluate("'a' + 'b'"), Ok(Value::String("ab".into())));
    }
    #[test]
    fn rejects_invalid_programs() {
        assert!(evaluate("2 + nope").is_err());
        assert!(evaluate("(2 + 3").is_err());
    }
    #[test]
    fn evaluates_unary_and_comparison_operators() {
        assert_eq!(evaluate("-4 + 5"), Ok(Value::Number(1.0)));
        assert_eq!(evaluate("3 < 4"), Ok(Value::Boolean(true)));
        assert_eq!(evaluate("!false"), Ok(Value::Boolean(true)));
    }
}
