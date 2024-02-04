use crate::{
    error_reporter,
    lexer::{Symbol, Token, TokenKind, Tokens},
};

use core::fmt;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Node {
    Num(i64),
    LocalVar {
        // Local variable address = RBP - offset
        offset: usize,
    },
    ArithOp {
        value: ArithOp,
        lhs: NodeChild,
        rhs: NodeChild,
    },
    CmpOp {
        value: CmpOp,
        lhs: NodeChild,
        rhs: NodeChild,
    },
}

type NodeChild = Box<Node>;

impl Node {
    fn num(value: i64) -> Self {
        Self::Num(value)
    }

    fn local_var(offset: usize) -> Self {
        Self::LocalVar { offset }
    }

    fn arith_op(value: ArithOp, lhs: Node, rhs: Node) -> Self {
        Self::ArithOp {
            value,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    fn cmp_op(value: CmpOp, lhs: Node, rhs: Node) -> Self {
        Self::CmpOp {
            value,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CmpOp {
    Lt,
    Lte,
    Eq,
    Neq,
}

impl fmt::Display for CmpOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CmpOp::Lt => "setl",
                CmpOp::Lte => "setle",
                CmpOp::Eq => "sete",
                CmpOp::Neq => "setne",
            }
        )
    }
}

impl From<&TokenKind> for CmpOp {
    fn from(value: &TokenKind) -> Self {
        match value {
            TokenKind::Symbol(Symbol::Lt) => Self::Lt,
            TokenKind::Symbol(Symbol::Lte) => Self::Lte,
            TokenKind::Symbol(Symbol::Eq) => Self::Eq,
            TokenKind::Symbol(Symbol::Neq) => Self::Neq,
            _ => {
                panic!("Invalid token passed: {:?}", value);
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
}

impl fmt::Display for ArithOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ArithOp::Add => "add",
                ArithOp::Sub => "sub",
                ArithOp::Mul => "imul",
                ArithOp::Div => "idiv",
                ArithOp::Assign => "mov",
            }
        )
    }
}

impl From<&TokenKind> for ArithOp {
    fn from(value: &TokenKind) -> Self {
        match value {
            TokenKind::Symbol(Symbol::Add) => Self::Add,
            TokenKind::Symbol(Symbol::Sub) => Self::Sub,
            TokenKind::Symbol(Symbol::Mul) => Self::Mul,
            TokenKind::Symbol(Symbol::Div) => Self::Div,
            _ => {
                panic!("Invalid token passed: {:?}", value);
            }
        }
    }
}

pub fn parse(tokens: Tokens) -> Vec<Node> {
    let mut tokens = tokens.into_iter().peekable();

    program(&mut tokens)
}

fn program<'a, I>(tokens: &mut Peekable<I>) -> Vec<Node>
where
    I: Iterator<Item = Token<'a>>,
{
    let mut nodes = vec![];

    while tokens.peek().is_some() {
        nodes.push(stmt(tokens));
    }

    nodes
}

fn stmt<'a, I>(tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = Token<'a>>,
{
    let node = expr(tokens);

    match tokens.next() {
        Some(Token {
            value: TokenKind::Symbol(Symbol::SemiColon),
            ..
        }) => {
            // Do nothing
        }
        token => {
            invalid_token(token, Some("Must be ;"));
        }
    }

    node
}

fn expr<'a, I>(tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = Token<'a>>,
{
    assign(tokens)
}

fn assign<'a, I>(tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = Token<'a>>,
{
    let mut node = equality(tokens);

    while let Some(token) = tokens.peek() {
        match token.value {
            TokenKind::Symbol(Symbol::Assign) => {
                tokens.next().unwrap();
                node = Node::arith_op(ArithOp::Assign, node, assign(tokens));
            }
            _ => {
                break;
            }
        }
    }

    node
}

fn equality<'a, I>(tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = Token<'a>>,
{
    let mut node = relational(tokens);

    while let Some(token) = tokens.peek() {
        match token.value {
            TokenKind::Symbol(Symbol::Eq | Symbol::Neq) => {
                node = Node::cmp_op(
                    CmpOp::from(&tokens.next().unwrap().value),
                    node,
                    relational(tokens),
                )
            }
            _ => {
                break;
            }
        }
    }

    node
}

fn relational<'a, I>(tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = Token<'a>>,
{
    let mut node = add(tokens);

    while let Some(token) = tokens.peek() {
        match token.value {
            TokenKind::Symbol(Symbol::Lt | Symbol::Lte) => {
                node = Node::cmp_op(
                    CmpOp::from(&tokens.next().unwrap().value),
                    node,
                    add(tokens),
                )
            }
            TokenKind::Symbol(Symbol::Gt) => {
                tokens.next().unwrap();
                node = Node::cmp_op(CmpOp::Lt, add(tokens), node);
            }
            TokenKind::Symbol(Symbol::Gte) => {
                tokens.next().unwrap();
                node = Node::cmp_op(CmpOp::Lte, add(tokens), node);
            }
            _ => {
                break;
            }
        }
    }

    node
}

fn add<'a, I>(tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = Token<'a>>,
{
    let mut node = mul(tokens);

    while let Some(token) = tokens.peek() {
        match token.value {
            TokenKind::Symbol(Symbol::Add | Symbol::Sub) => {
                node = Node::arith_op(
                    ArithOp::from(&tokens.next().unwrap().value),
                    node,
                    mul(tokens),
                )
            }
            _ => {
                break;
            }
        }
    }

    node
}

fn mul<'a, I>(tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = Token<'a>>,
{
    let mut node = unary(tokens);

    while let Some(token) = tokens.peek() {
        match token.value {
            TokenKind::Symbol(Symbol::Mul | Symbol::Div) => {
                node = Node::arith_op(
                    ArithOp::from(&tokens.next().unwrap().value),
                    node,
                    unary(tokens),
                )
            }
            _ => {
                return node;
            }
        }
    }

    node
}

fn unary<'a, I>(tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = Token<'a>>,
{
    match tokens.peek() {
        Some(Token {
            value: TokenKind::Symbol(Symbol::Add),
            ..
        }) => {
            tokens.next().unwrap();
            primary(tokens)
        }
        Some(Token {
            value: TokenKind::Symbol(Symbol::Sub),
            ..
        }) => {
            tokens.next().unwrap();
            Node::arith_op(ArithOp::Sub, Node::num(0), primary(tokens))
        }
        _ => primary(tokens),
    }
}

fn primary<'a, I>(tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = Token<'a>>,
{
    match tokens.next() {
        Some(Token {
            value: TokenKind::Num(num),
            ..
        }) => Node::num(num),
        Some(Token {
            value: TokenKind::Ident(ident),
            ..
        }) => {
            let offset = (ident.chars().next().unwrap() as usize - 'a' as usize + 1) * 8;
            Node::local_var(offset)
        }
        Some(Token {
            value: TokenKind::Symbol(Symbol::LParen),
            ..
        }) => {
            let node = expr(tokens);

            match tokens.next() {
                Some(Token {
                    value: TokenKind::Symbol(Symbol::RParen),
                    ..
                }) => {
                    // Do nothing
                }
                token => {
                    invalid_token(token, Some("Must be )"));
                }
            }

            node
        }
        token => {
            invalid_token(token, None);
        }
    }
}

fn invalid_token(token: Option<Token>, message: Option<&str>) -> ! {
    match token {
        Some(Token { metadata, .. }) => {
            error_reporter::report(
                metadata.user_input,
                metadata.code_location,
                match message {
                    Some(message) => message,
                    None => "Invalid token",
                },
            );
        }
        None => match message {
            Some(message) => panic!("{}", message),
            None => panic!("Unexpected EOF"),
        },
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::lexer;

    use super::*;

    #[test]
    fn test_ok_parse_single() -> Result<()> {
        let tokens = lexer::tokenize("1;")?;
        let actual = parse(tokens);

        assert_eq!(actual, vec![Node::num(1)]);

        Ok(())
    }

    #[test]
    fn test_ok_parse_complex() -> Result<()> {
        let tokens = lexer::tokenize("(+1 + -2) * 3 - 4 / 5;")?;
        let actual = parse(tokens);

        assert_eq!(
            actual,
            vec![Node::arith_op(
                ArithOp::Sub,
                Node::arith_op(
                    ArithOp::Mul,
                    Node::arith_op(
                        ArithOp::Add,
                        Node::num(1),
                        Node::arith_op(ArithOp::Sub, Node::num(0), Node::num(2))
                    ),
                    Node::num(3),
                ),
                Node::arith_op(ArithOp::Div, Node::num(4), Node::num(5))
            )]
        );

        Ok(())
    }

    #[test]
    fn test_ok_with_cmp() -> Result<()> {
        let tokens = lexer::tokenize("(1 + 2 * 3 > 4) != (5 < 6 == 7 >= 8);")?;
        let actual = parse(tokens);

        assert_eq!(
            actual,
            vec![Node::cmp_op(
                CmpOp::Neq,
                Node::cmp_op(
                    CmpOp::Lt,
                    Node::num(4),
                    Node::arith_op(
                        ArithOp::Add,
                        Node::num(1),
                        Node::arith_op(ArithOp::Mul, Node::num(2), Node::num(3))
                    )
                ),
                Node::cmp_op(
                    CmpOp::Eq,
                    Node::cmp_op(CmpOp::Lt, Node::num(5), Node::num(6)),
                    Node::cmp_op(CmpOp::Lte, Node::num(8), Node::num(7),)
                )
            )]
        );

        Ok(())
    }

    #[test]
    fn test_ok_with_assign() -> Result<()> {
        let tokens = lexer::tokenize("a = 1 + 2 * 3; b = a;")?;
        let actual = parse(tokens);

        assert_eq!(
            actual,
            vec![
                Node::arith_op(
                    ArithOp::Assign,
                    Node::local_var(8),
                    Node::arith_op(
                        ArithOp::Add,
                        Node::num(1),
                        Node::arith_op(ArithOp::Mul, Node::num(2), Node::num(3))
                    )
                ),
                Node::arith_op(ArithOp::Assign, Node::local_var(16), Node::local_var(8))
            ]
        );

        Ok(())
    }
}
