use std::env::args;

use anyhow::{bail, Result};
use tokenizer::{tokenize, Token};

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
        match token {
            Token::Num(n) => res += &format!("        mov rax, {}\n", n),
            _ => {
                panic!("First expr must number")
            }
        }
    }

    while let Some(token) = tokens.next() {
        match token {
            Token::Num(_) => {
                panic!("Unexpedted number");
            }
            Token::Symbol(sym) => {
                if let Some(Token::Num(n)) = tokens.next() {
                    res += &format!("        {} rax, {}\n", sym, n)
                } else {
                    panic!("Next to op must be num")
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
