use crate::lexer::{Symbol, Token, TokenKind, Tokens};

use core::fmt;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Node {
    Num(i64),
    ArithOp {
        value: ArithOp,
        lhs: NodeChild,
        rhs: NodeChild,
    },
}

type NodeChild = Box<Node>;

impl Node {
    fn num(value: i64) -> Self {
        Self::Num(value)
    }

    fn arith_op(value: ArithOp, lhs: Node, rhs: Node) -> Self {
        Self::ArithOp {
            value,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
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
                panic!("Invalid token passed.")
            }
        }
    }
}

pub fn parse(tokens: Tokens) -> Node {
    let mut tokens = tokens.into_iter().peekable();

    expr(&mut tokens)
}

fn expr<I>(tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = Token>,
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
                return node;
            }
        }
    }

    node
}

fn mul<I>(tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = Token>,
{
    let mut node = primary(tokens);

    while let Some(token) = tokens.peek() {
        match token.value {
            TokenKind::Symbol(Symbol::Mul | Symbol::Div) => {
                node = Node::arith_op(
                    ArithOp::from(&tokens.next().unwrap().value),
                    node,
                    primary(tokens),
                )
            }
            _ => {
                return node;
            }
        }
    }

    node
}

fn primary<I>(tokens: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = Token>,
{
    match tokens.next() {
        Some(Token {
            value: TokenKind::Num(num),
            ..
        }) => Node::num(num),
        Some(Token {
            value: TokenKind::Symbol(Symbol::LParen),
            ..
        }) => {
            let node = expr(tokens);

            if let Some(Token {
                value: TokenKind::Symbol(Symbol::RParen),
                ..
            }) = tokens.next()
            {
                // Do nothing
            } else {
                panic!("Must be )");
            }

            node
        }
        _ => {
            panic!("Unexpected token")
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::lexer;

    use super::*;

    #[test]
    fn test_ok_parse_single() -> Result<()> {
        let tokens = lexer::tokenize("1")?;
        let actual = parse(tokens);

        assert_eq!(actual, Node::num(1));

        Ok(())
    }

    #[test]
    fn test_ok_parse_complex() -> Result<()> {
        let tokens = lexer::tokenize("(1 + 2) * 3 - 4 / 5")?;
        let actual = parse(tokens);

        assert_eq!(
            actual,
            Node::arith_op(
                ArithOp::Sub,
                Node::arith_op(
                    ArithOp::Mul,
                    Node::arith_op(ArithOp::Add, Node::num(1), Node::num(2)),
                    Node::num(3),
                ),
                Node::arith_op(ArithOp::Div, Node::num(4), Node::num(5))
            )
        );

        Ok(())
    }
}
