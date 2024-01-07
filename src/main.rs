use std::env::args;

use anyhow::{bail, Result};

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

    let (num, rest) = utils::exstract_head_int(x);

    match num {
        Some(num) => res += &format!("        mov rax, {}\n", num),
        None => bail!("Specify first num"),
    }

    let mut rest = rest;
    while let Some(expr) = rest {
        let op = match expr.chars().next().unwrap() {
            '+' => "add",
            '-' => "sub",
            op => {
                bail!("Invalid op: {}", op)
            }
        };

        let (num, rest_) = utils::exstract_head_int(&expr[1..]);
        rest = rest_;

        match num {
            Some(num) => res += &format!("        {} rax, {}\n", op, num),
            None => {
                bail!("Invalid expr")
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
