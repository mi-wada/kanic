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
        push rbp
        mov rbp, rsp
        sub rsp, 208
",
    ) + &nodes_to_asem(&parser::parse(lexer::tokenize(c_code)?))?
        + "        mov rsp, rbp
        pop rbp
        ret
")
}

fn nodes_to_asem(nodes: &[Node]) -> Result<String> {
    let mut asem = String::new();

    for node in nodes {
        asem += &to_asem(node)?;
        asem += "        pop rax\n";
    }

    Ok(asem)
}

fn to_asem(ast: &Node) -> Result<String> {
    match ast {
        Node::Num(n) => Ok(format!("        push {}\n", n)),
        Node::LocalVar { offset } => Ok(format!("        push [rbp-{offset}]\n")),
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
            ArithOp::Assign => Ok(to_asem(rhs.as_ref())?
                + &{
                    if let Node::LocalVar { offset } = lhs.as_ref() {
                        format!(
                            "        pop rax
        mov [rbp-{offset}], rax
        push rax
"
                        )
                    } else {
                        panic!("lhs of assign must be LocalVar")
                    }
                }),
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
