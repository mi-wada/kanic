use crate::{
    lexer,
    parser::{self, ArithOp, Node},
};
use anyhow::Result;

pub fn generate(c_code: &str) -> Result<String> {
    Ok(String::from(
        "\
.intel_syntax noprefix
.globl main

main:
",
    ) + &to_asem(&parser::parse(lexer::tokenize(c_code)?))?
        + "        pop rax
        ret
")
}

fn to_asem(ast: &Node) -> Result<String> {
    match ast {
        Node::Num(n) => Ok(format!("        push {}\n", n)),
        Node::ArithOp {
            value: arith_op,
            lhs,
            rhs,
        } => match arith_op {
            ArithOp::Add | ArithOp::Sub | ArithOp::Mul => Ok(to_asem(lhs.as_ref())?
                + &to_asem(rhs.as_ref())?
                + &format!(
                    "        pop rdi
        pop rax
        {arith_op} rax, rdi
        push rax
"
                )),
            ArithOp::Div => Ok(to_asem(lhs.as_ref())?
                + &to_asem(rhs.as_ref())?
                + &format!(
                    "        pop rdi
        pop rax
        cqo
        {arith_op} rdi
        push rax
"
                )),
        },
        Node::CmpOp {
            value: cmp_op,
            lhs,
            rhs,
        } => Ok(to_asem(lhs.as_ref())?
            + &to_asem(rhs.as_ref())?
            + &format!(
                "        pop rdi
        pop rax
        cmp rax, rdi
        {cmp_op} al
        movzb rax, al
        push rax
"
            )),
    }
}
