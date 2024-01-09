use std::env::args;

use anyhow::Result;
use lexer::{tokenize, Token, TokenKind};
use parser::Node;

use crate::parser::NodeKind;

mod error_reporter;
mod lexer;
mod parser;

fn main() -> Result<()> {
    let x = args().nth(1).expect("Please provide a expr");

    println!("{}", to_asem_entry_point(&x)?);

    Ok(())
}

fn to_asem_entry_point(x: &str) -> Result<String> {
    Ok(String::from(
        "\
.intel_syntax noprefix
.globl main

main:
",
    ) + &to_asem(&parser::parse(lexer::tokenize(x)?))?
        + "        pop rax
        ret
")
}

fn to_asem(ast: &Node) -> Result<String> {
    match ast {
        Node {
            value: NodeKind::Num(n),
            ..
        } => Ok(format!("        push {}\n", n)),
        Node {
            value: NodeKind::ArithOp(arith_op),
            lhs,
            rhs,
        } => Ok(to_asem(lhs.as_ref().unwrap())?
            + &to_asem(rhs.as_ref().unwrap())?
            + &format!(
                "        pop rdi
        pop rax
        {arith_op} rax, rdi
        push rax
"
            )),
    }
}
