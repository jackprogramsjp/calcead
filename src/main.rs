mod compiler;
mod lexer;
mod nodes;
mod parser;
mod tokens;

extern crate llvm_sys;

use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fs;
use std::io;

fn usage() -> &'static str {
    "usage: calcead [file]"
}

fn main() -> io::Result<()> {
    let mut argv = std::env::args();

    if argv.len() != 2 {
        println!("{}", usage());
        return Ok(());
    }

    let filename = argv.nth(1).unwrap();

    let data = fs::read_to_string(filename.clone())?;
    let lexer = Lexer::new(data);
    let mut tokens = Vec::new();

    for token in lexer {
        match token {
            Ok(t) => tokens.push(t),
            Err(e) => {
                eprintln!("{}", e);
                return Ok(());
            }
        }
    }

    let mut parser = Parser::new(tokens);
    let tree = parser.parse();

    match tree {
        Some(node) => match node {
            Ok(node) => {
                let mut compiler = Compiler::new(Some(&*filename.clone()));
                compiler.compile(node);
                // compiler.dump();
                let new_filename = format!("{}.ll", filename);
                compiler.dump_to_file(&*new_filename)?;
            }
            Err(e) => {
                eprintln!("{}", e);
                return Ok(());
            }
        },
        None => return Ok(()),
    }

    Ok(())
}
