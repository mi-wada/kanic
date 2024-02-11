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

    pub fn ident(ident: String, code_location: usize, user_input: &'a str) -> Self {
        Self::new(TokenKind::Ident(ident), code_location, user_input)
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Symbol(Symbol),
    Num(i64),
    Ident(String),
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
    Assign,
    SemiColon,
    Ret,
    If,
    Else,
    While,
    For,
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        match value {
            "<=" => Self::Lte,
            ">=" => Self::Gte,
            "==" => Self::Eq,
            "!=" => Self::Neq,
            "return" => Self::Ret,
            "if" => Self::If,
            "else" => Self::Else,
            "while" => Self::While,
            "for" => Self::For,
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
            '=' => Self::Assign,
            ';' => Self::SemiColon,
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
            '+' | '-' | '*' | '/' | '(' | ')' | ';' => {
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
            '=' => {
                let next_char = chars.peek().map(|&(_, c)| c);

                match (char, next_char) {
                    ('=', Some('=')) => {
                        chars.next();
                        tokens.push(Token::symbol(Symbol::Eq, code_location, s))
                    }
                    _ => {
                        tokens.push(Token::symbol(Symbol::Assign, code_location, s));
                    }
                }
            }
            '!' => {
                let next_char = chars.peek().map(|&(_, c)| c);

                match (char, next_char) {
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
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut str = String::new();
                str.push(char);

                while let Some(&(_, next_char)) = chars.peek() {
                    if next_char.is_ascii_alphanumeric() || next_char == '_' {
                        str.push(next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                match str.as_str() {
                    "return" => tokens.push(Token::symbol(Symbol::Ret, code_location, s)),
                    "if" => tokens.push(Token::symbol(Symbol::If, code_location, s)),
                    "else" => tokens.push(Token::symbol(Symbol::Else, code_location, s)),
                    "while" => tokens.push(Token::symbol(Symbol::While, code_location, s)),
                    "for" => tokens.push(Token::symbol(Symbol::For, code_location, s)),
                    _ => tokens.push(Token::ident(str, code_location, s)),
                }
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
        assert_eq!(actual.next(), None);

        Ok(())
    }

    #[test]
    fn test_ok_assign() -> Result<()> {
        let c_code = "a = 1; bar = 2; car = a + bar; return car;";
        let mut actual = tokenize(c_code)?.into_iter();

        assert_eq!(actual.next(), Some(Token::ident("a".into(), 0, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::Assign, 2, c_code))
        );
        assert_eq!(actual.next(), Some(Token::num(1, 4, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::SemiColon, 5, c_code))
        );
        assert_eq!(actual.next(), Some(Token::ident("bar".into(), 7, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::Assign, 11, c_code))
        );
        assert_eq!(actual.next(), Some(Token::num(2, 13, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::SemiColon, 14, c_code))
        );
        assert_eq!(actual.next(), Some(Token::ident("car".into(), 16, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::Assign, 20, c_code))
        );
        assert_eq!(actual.next(), Some(Token::ident("a".into(), 22, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Add, 24, c_code)));
        assert_eq!(actual.next(), Some(Token::ident("bar".into(), 26, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::SemiColon, 29, c_code))
        );
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Ret, 31, c_code)));
        assert_eq!(actual.next(), Some(Token::ident("car".into(), 38, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::SemiColon, 41, c_code))
        );
        assert_eq!(actual.next(), None);

        Ok(())
    }

    #[test]
    fn test_ok_if() -> Result<()> {
        let c_code = "if (1 < 2) return 3; else return 4;";
        let mut actual = tokenize(c_code)?.into_iter();

        assert_eq!(actual.next(), Some(Token::symbol(Symbol::If, 0, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::LParen, 3, c_code))
        );
        assert_eq!(actual.next(), Some(Token::num(1, 4, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Lt, 6, c_code)));
        assert_eq!(actual.next(), Some(Token::num(2, 8, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::RParen, 9, c_code))
        );
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Ret, 11, c_code)));
        assert_eq!(actual.next(), Some(Token::num(3, 18, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::SemiColon, 19, c_code))
        );
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Else, 21, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Ret, 26, c_code)));
        assert_eq!(actual.next(), Some(Token::num(4, 33, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::SemiColon, 34, c_code))
        );
        assert_eq!(actual.next(), None);

        Ok(())
    }

    #[test]
    fn test_ok_while() -> Result<()> {
        let c_code = "while (1 < 2) return 3;";
        let mut actual = tokenize(c_code)?.into_iter();

        assert_eq!(actual.next(), Some(Token::symbol(Symbol::While, 0, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::LParen, 6, c_code))
        );
        assert_eq!(actual.next(), Some(Token::num(1, 7, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Lt, 9, c_code)));
        assert_eq!(actual.next(), Some(Token::num(2, 11, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::RParen, 12, c_code))
        );
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Ret, 14, c_code)));
        assert_eq!(actual.next(), Some(Token::num(3, 21, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::SemiColon, 22, c_code))
        );
        assert_eq!(actual.next(), None);

        Ok(())
    }

    #[test]
    fn test_ok_for() -> Result<()> {
        let c_code = "for (i = 0; i < 10; i = i + 1) return i;";
        let mut actual = tokenize(c_code)?.into_iter();

        assert_eq!(actual.next(), Some(Token::symbol(Symbol::For, 0, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::LParen, 4, c_code))
        );
        assert_eq!(actual.next(), Some(Token::ident("i".into(), 5, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::Assign, 7, c_code))
        );
        assert_eq!(actual.next(), Some(Token::num(0, 9, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::SemiColon, 10, c_code))
        );
        assert_eq!(actual.next(), Some(Token::ident("i".into(), 12, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Lt, 14, c_code)));
        assert_eq!(actual.next(), Some(Token::num(10, 16, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::SemiColon, 18, c_code))
        );
        assert_eq!(actual.next(), Some(Token::ident("i".into(), 20, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::Assign, 22, c_code))
        );
        assert_eq!(actual.next(), Some(Token::ident("i".into(), 24, c_code)));
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Add, 26, c_code)));
        assert_eq!(actual.next(), Some(Token::num(1, 28, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::RParen, 29, c_code))
        );
        assert_eq!(actual.next(), Some(Token::symbol(Symbol::Ret, 31, c_code)));
        assert_eq!(actual.next(), Some(Token::ident("i".into(), 38, c_code)));
        assert_eq!(
            actual.next(),
            Some(Token::symbol(Symbol::SemiColon, 39, c_code))
        );
        assert_eq!(actual.next(), None);

        Ok(())
    }
}
