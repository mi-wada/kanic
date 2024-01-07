use std::env::args;

use anyhow::Result;

fn main() -> Result<()> {
    let x = args().nth(1).expect("Please provide a number");

    println!("{}", to_asem(&x)?);

    Ok(())
}

fn to_asem(x: &str) -> Result<String> {
    let x = x.parse::<i64>()?;

    Ok(format!(
        "\
.intel_syntax noprefix
.globl main

main:
        mov rax, {}
        ret
",
        x
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
}
