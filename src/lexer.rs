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

impl Tokens {
    fn push(&mut self, token: Token) {
        self.0.push(token);
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
    Mul,
    Div,
    LParen,
    RParen,
    Lt,
    Lte,
    Gt,
    Gte,
    Eq,
    Neq,
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            "(" => Self::LParen,
            ")" => Self::RParen,
            "<" => Self::Lt,
            "<=" => Self::Lte,
            ">" => Self::Gt,
            ">=" => Self::Gte,
            "==" => Self::Eq,
            "!=" => Self::Neq,
            _ => panic!("Invalid symbol"),
        }
    }
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Add,
            '-' => Self::Sub,
            '*' => Self::Mul,
            '/' => Self::Div,
            '(' => Self::LParen,
            ')' => Self::RParen,
            '<' => Self::Lt,
            '>' => Self::Gt,
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
                Symbol::Mul => "mul",
                Symbol::Div => "div",
                Symbol::LParen => "",
                Symbol::RParen => "",
                Symbol::Lt => "setl",
                Symbol::Lte => "setle",
                Symbol::Gt => panic!("Use Lt"),
                Symbol::Gte => panic!("Use Lte"),
                Symbol::Eq => "sete",
                Symbol::Neq => "setne",
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
            '+' | '-' | '*' | '/' | '(' | ')' => {
                tokens.push(Token::symbol(Symbol::from(char), code_location))
            }
            '<' | '>' => {
                let next_char = chars.peek().map(|&(_, c)| c);

                match (char, next_char) {
                    ('<', Some('=')) => {
                        chars.next();
                        tokens.push(Token::symbol(Symbol::Lte, code_location))
                    }
                    ('>', Some('=')) => {
                        chars.next();
                        tokens.push(Token::symbol(Symbol::Gte, code_location))
                    }
                    _ => tokens.push(Token::symbol(Symbol::from(char), code_location)),
                }
            }
            '=' | '!' => {
                let next_char = chars.peek().map(|&(_, c)| c);

                match (char, next_char) {
                    ('=', Some('=')) => {
                        chars.next();
                        tokens.push(Token::symbol(Symbol::Eq, code_location))
                    }
                    ('!', Some('=')) => {
                        chars.next();
                        tokens.push(Token::symbol(Symbol::Neq, code_location))
                    }
                    _ => {
                        error_reporter::report(s, code_location, "Invalid token");
                    }
                }
            }
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

                tokens.push(Token::num(numbers.parse().unwrap(), code_location))
            }
            _ => {
                error_reporter::report(s, code_location, "Invalid token");
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
        let mut actual = tokenize("(+1 + -2) * 3 - 4 / 5")?.into_iter();

        assert_eq!(actual.next(), Some(Token::symbol(Symbol::LParen, 0)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Add, 1)));
        assert_eq!(actual.next(), Some(Token::num(1, 2)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Add, 4)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Sub, 6)));
        assert_eq!(actual.next(), Some(Token::num(2, 7)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::RParen, 8)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Mul, 10)));
        assert_eq!(actual.next(), Some(Token::num(3, 12)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Sub, 14)));
        assert_eq!(actual.next(), Some(Token::num(4, 16)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Div, 18)));
        assert_eq!(actual.next(), Some(Token::num(5, 20)));
        assert_eq!(actual.next(), None);

        Ok(())
    }

    #[test]
    fn test_success_with_comparison_operator() -> Result<()> {
        let mut actual = tokenize("1 < 2 <= 3 > 4 >= 5 == 6 != 7")?.into_iter();

        assert_eq!(actual.next(), Some(Token::num(1, 0)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Lt, 2)));
        assert_eq!(actual.next(), Some(Token::num(2, 4)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Lte, 6)));
        assert_eq!(actual.next(), Some(Token::num(3, 9)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Gt, 11)));
        assert_eq!(actual.next(), Some(Token::num(4, 13)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Gte, 15)));
        assert_eq!(actual.next(), Some(Token::num(5, 18)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Eq, 20)));
        assert_eq!(actual.next(), Some(Token::num(6, 23)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Neq, 25)));
        assert_eq!(actual.next(), Some(Token::num(7, 28)));

        Ok(())
    }
}
