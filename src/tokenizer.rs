use core::fmt;

use anyhow::Result;

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
pub enum Token {
    Symbol(Symbol),
    Num(i64),
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

    let mut chars = s.chars().peekable();

    while let Some(char) = chars.next() {
        match char {
            ' ' | '\n' | '\r' => {
                continue;
            }
            '+' => tokens.0.push(Token::Symbol(Symbol::Add)),
            '-' => tokens.0.push(Token::Symbol(Symbol::Sub)),
            '0'..='9' => {
                let mut numbers = String::new();
                numbers.push(char);

                while let Some(&next_char) = chars.peek() {
                    if next_char.is_ascii_digit() {
                        numbers.push(next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.0.push(Token::Num(numbers.parse().unwrap()))
            }
            c => {
                panic!("Invalid token: {}", c)
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

        assert_eq!(actual.next(), Some(Token::Num(1)));
        assert_eq!(actual.next(), Some(Token::Symbol(Symbol::Add)));
        assert_eq!(actual.next(), Some(Token::Num(2)));
        assert_eq!(actual.next(), Some(Token::Symbol(Symbol::Sub)));
        assert_eq!(actual.next(), Some(Token::Num(10)));
        assert_eq!(actual.next(), None);

        Ok(())
    }
}
