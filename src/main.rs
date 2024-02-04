use std::env::args;

use anyhow::Result;

mod codegen;
mod error_reporter;
mod lexer;
mod parser;

fn main() -> Result<()> {
    let c_code = args().nth(1).expect("Please provide a expr");

    println!("{}", codegen::generate(&c_code)?);

    Ok(())
}
