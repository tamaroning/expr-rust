mod ast;
//mod codegen;
mod lexer;
mod parser;
mod ty;

//use codegen::codegen;
use chumsky::Parser;
use lexer::lexer;
use parser::parser;

fn main() {
    let file_name = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(file_name).unwrap();

    match parser().parse(src) {
        Ok(program) => {
            println!("{:?}", program);
            //codegen(program)
        }
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| println!("Parse error: {}", e)),
    }
}
