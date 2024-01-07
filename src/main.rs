use std::env::args;

use anyhow::Result;
use tokenizer::{tokenize, Token, TokenKind};

mod error_reporter;
mod tokenizer;

fn main() -> Result<()> {
    let x = args().nth(1).expect("Please provide a expr");

    println!("{}", to_asem(&x)?);

    Ok(())
}

fn to_asem(x: &str) -> Result<String> {
    let mut res = String::from(
        ".intel_syntax noprefix
.globl main\n
main:\n",
    );

    let mut tokens = tokenize(x)?.into_iter().peekable();

    if let Some(token) = tokens.next() {
        match token.value {
            TokenKind::Num(n) => res += &format!("        mov rax, {}\n", n),
            _ => error_reporter::report(x, token.metadata.code_location, "First expr must number"),
        }
    }

    while let Some(token) = tokens.next() {
        match token.value {
            TokenKind::Num(_) => {
                error_reporter::report(x, token.metadata.code_location, "Unexpedted number");
            }
            TokenKind::Symbol(sym) => {
                let token = tokens.next();
                if let Some(Token {
                    value: TokenKind::Num(n),
                    ..
                }) = token
                {
                    res += &format!("        {} rax, {}\n", sym, n)
                } else {
                    error_reporter::report(
                        x,
                        token.unwrap().metadata.code_location,
                        "The value next to operator must be Number",
                    );
                }
            }
        }
    }

    res += "        ret\n";

    Ok(res)
}
