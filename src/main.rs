mod ast;
mod codegen;
mod lexer;
mod parser;
mod ty;
mod typeck;

use chumsky::Parser;
use codegen::codegen;
use lexer::lexer;
use parser::parser;
use typeck::Typeck;

fn main() {
    let file_name = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(file_name).unwrap();

    match lexer().parse(src) {
        Ok(tt) => {
            println!("{:?}", tt);
            match parser().parse(tt) {
                Ok(mut program) => {
                    println!("{:?}", program);

                    program.typeck();

                    println!("{:?}", program);
                    codegen(program)
                }
                Err(parse_errs) => parse_errs
                    .into_iter()
                    .for_each(|e| println!("Parse error: {:?}", e)),
            }
        }
        Err(tokenize_errs) => tokenize_errs
            .into_iter()
            .for_each(|e| println!("Tokenize error: {:?}", e)),
    }
}
