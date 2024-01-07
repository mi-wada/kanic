use core::fmt;

use anyhow::Result;

use crate::error_reporter;

#[derive(PartialEq, Debug)]
pub struct Tokens(Vec<Token>);

impl IntoIterator for Tokens {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub value: TokenKind,
    pub metadata: TokenMetadata,
}

impl Token {
    fn new(value: TokenKind, code_location: usize) -> Self {
        Self {
            value,
            metadata: TokenMetadata { code_location },
        }
    }

    pub fn symbol(symbol_kind: Symbol, code_location: usize) -> Self {
        Self::new(TokenKind::Symbol(symbol_kind), code_location)
    }

    pub fn num(num: i64, code_location: usize) -> Self {
        Self::new(TokenKind::Num(num), code_location)
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Symbol(Symbol),
    Num(i64),
}

#[derive(PartialEq, Debug)]
pub struct TokenMetadata {
    // Indicates how many bytes of the source code the token starts from.
    pub code_location: usize,
}

#[derive(PartialEq, Debug)]
pub enum Symbol {
    Add,
    Sub,
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Add,
            "-" => Self::Sub,
            _ => panic!("Invalid symbol"),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Symbol::Add => "add",
                Symbol::Sub => "sub",
            }
        )
    }
}

pub fn tokenize(s: &str) -> Result<Tokens> {
    let mut tokens = Tokens(vec![]);

    let mut chars = s.char_indices().peekable();

    while let Some((code_location, char)) = chars.next() {
        match char {
            ' ' | '\n' | '\r' => {
                continue;
            }
            '+' => tokens.0.push(Token::symbol(Symbol::Add, code_location)),
            '-' => tokens.0.push(Token::symbol(Symbol::Sub, code_location)),
            '0'..='9' => {
                let mut numbers = String::new();
                numbers.push(char);

                while let Some(&(_, next_char)) = chars.peek() {
                    if next_char.is_ascii_digit() {
                        numbers.push(next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens
                    .0
                    .push(Token::num(numbers.parse().unwrap(), code_location))
            }
            _ => {
                error_reporter::report_error(s, code_location, "Invalid token");
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() -> Result<()> {
        let mut actual = tokenize("1 + 2 - 10")?.into_iter();

        assert_eq!(actual.next(), Some(Token::num(1, 0)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Add, 2)));
        assert_eq!(actual.next(), Some(Token::num(2, 4)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Sub, 6)));
        assert_eq!(actual.next(), Some(Token::num(10, 8)));
        assert_eq!(actual.next(), None);

        Ok(())
    }
}
