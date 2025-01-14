mod token;
mod tokenizer;
mod ast;
mod parser;
use std::env;
use std::net::{TcpListener, TcpStream};

use crate::tokenizer::Tokenizer;
use crate::parser::*;

fn main() {
    let s = std::fs::read_to_string("test.test").unwrap();
    let t = Tokenizer::new(s);

    let mut p = Parser::new(t);
    let stmt = p.parse_stmt();
    p.print_stmt(stmt);

}

