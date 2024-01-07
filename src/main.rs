use std::env::args;

use anyhow::{bail, Result};
use tokenizer::{tokenize, Token, TokenKind};

mod error_reporter;
mod tokenizer;
mod utils;

fn main() -> Result<()> {
    let x = args().nth(1).expect("Please provide a number");

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
            _ => error_reporter::report_error(
                x,
                token.metadata.code_location,
                "First expr must number",
            ),
        }
    }

    while let Some(token) = tokens.next() {
        match token.value {
            TokenKind::Num(_) => {
                error_reporter::report_error(x, token.metadata.code_location, "Unexpedted number");
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
                    error_reporter::report_error(
                        x,
                        token.unwrap().metadata.code_location,
                        "Next to op must be num",
                    );
                }
            }
        }
    }

    res += "        ret\n";

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
}
