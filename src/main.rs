mod token;
mod tokenizer;
mod ast;
mod parser;
use crate::tokenizer::Tokenizer;
use crate::token::*;
use crate::parser::*;

fn main() {
    let s = String::from("123 + 2; 2 + 2;");
    let mut t = Tokenizer::new(s);
    let mut p = Parser::new(t);
    let stmt = p.parse_stmt();
    p.print_stmt(stmt);
    let stmt = p.parse_stmt();
    p.print_stmt(stmt);

}
