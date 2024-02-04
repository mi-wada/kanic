use anyhow::Result;

use crate::error_reporter;

#[derive(PartialEq, Debug)]
pub struct Tokens<'a>(Vec<Token<'a>>);

impl<'a> IntoIterator for Tokens<'a> {
    type Item = Token<'a>;
    type IntoIter = std::vec::IntoIter<Token<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> Tokens<'a> {
    fn push(&mut self, token: Token<'a>) {
        self.0.push(token);
    }
}

#[derive(PartialEq, Debug)]
pub struct Token<'a> {
    pub value: TokenKind,
    pub metadata: TokenMetadata<'a>,
}

impl<'a> Token<'a> {
    fn new(value: TokenKind, code_location: usize, user_input: &'a str) -> Self {
        Self {
            value,
            metadata: TokenMetadata {
                code_location,
                user_input,
            },
        }
    }

    pub fn symbol(symbol_kind: Symbol, code_location: usize, user_input: &'a str) -> Self {
        Self::new(TokenKind::Symbol(symbol_kind), code_location, user_input)
    }

    pub fn num(num: i64, code_location: usize, user_input: &'a str) -> Self {
        Self::new(TokenKind::Num(num), code_location, user_input)
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Symbol(Symbol),
    Num(i64),
}

#[derive(PartialEq, Debug)]
pub struct TokenMetadata<'a> {
    // Indicates how many bytes of the source code the token starts from.
    pub code_location: usize,
    pub user_input: &'a str,
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
            "<=" => Self::Lte,
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

pub fn tokenize(s: &str) -> Result<Tokens> {
    let mut tokens = Tokens(vec![]);

    let mut chars = s.char_indices().peekable();

    while let Some((code_location, char)) = chars.next() {
        match char {
            ' ' | '\n' | '\r' => {
                continue;
            }
            '+' | '-' | '*' | '/' | '(' | ')' => {
                tokens.push(Token::symbol(Symbol::from(char), code_location, s))
            }
            '<' | '>' => {
                let next_char = chars.peek().map(|&(_, c)| c);

                match (char, next_char) {
                    ('<', Some('=')) => {
                        chars.next();
                        tokens.push(Token::symbol(Symbol::Lte, code_location, s))
                    }
                    ('>', Some('=')) => {
                        chars.next();
                        tokens.push(Token::symbol(Symbol::Gte, code_location, s))
                    }
                    _ => tokens.push(Token::symbol(Symbol::from(char), code_location, s)),
                }
            }
            '=' | '!' => {
                let next_char = chars.peek().map(|&(_, c)| c);

                match (char, next_char) {
                    ('=', Some('=')) => {
                        chars.next();
                        tokens.push(Token::symbol(Symbol::Eq, code_location, s))
                    }
                    ('!', Some('=')) => {
                        chars.next();
                        tokens.push(Token::symbol(Symbol::Neq, code_location, s))
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

                tokens.push(Token::num(numbers.parse().unwrap(), code_location, s))
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
        let c_code = "(+1 + -2) * 3 - 4 / 5";
        let mut actual = tokenize(c_code)?.into_iter();

        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::LParen, 0, c_code))
        );
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Add, 1, c_code)));
        assert_eq!(actual.next(), Some(Token::num(1, 2, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Add, 4, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Sub, 6, c_code)));
        assert_eq!(actual.next(), Some(Token::num(2, 7, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::RParen, 8, c_code))
        );
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Mul, 10, c_code)));
        assert_eq!(actual.next(), Some(Token::num(3, 12, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Sub, 14, c_code)));
        assert_eq!(actual.next(), Some(Token::num(4, 16, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Div, 18, c_code)));
        assert_eq!(actual.next(), Some(Token::num(5, 20, c_code)));
        assert_eq!(actual.next(), None);

        Ok(())
    }

    #[test]
    fn test_success_with_comparison_operator() -> Result<()> {
        let c_code = "1 < 2 <= 3 > 4 >= 5 == 6 != 7";
        let mut actual = tokenize(c_code)?.into_iter();

        assert_eq!(actual.next(), Some(Token::num(1, 0, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Lt, 2, c_code)));
        assert_eq!(actual.next(), Some(Token::num(2, 4, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Lte, 6, c_code)));
        assert_eq!(actual.next(), Some(Token::num(3, 9, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Gt, 11, c_code)));
        assert_eq!(actual.next(), Some(Token::num(4, 13, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Gte, 15, c_code)));
        assert_eq!(actual.next(), Some(Token::num(5, 18, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Eq, 20, c_code)));
        assert_eq!(actual.next(), Some(Token::num(6, 23, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Neq, 25, c_code)));
        assert_eq!(actual.next(), Some(Token::num(7, 28, c_code)));

        Ok(())
    }
}
