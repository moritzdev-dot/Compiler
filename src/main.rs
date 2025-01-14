mod token;
mod tokenizer;
mod ast;
mod parser;
use std::env;
use std::net::{TcpListener, TcpStream};

use crate::tokenizer::Tokenizer;
use crate::parser::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 0 {
        run_server();
    } else {
        let s = std::fs::read_to_string("test.test").unwrap();
        let t = Tokenizer::new(s);

        let mut p = Parser::new(t);
        let stmt = p.parse_stmt();
        p.print_stmt(stmt);
    }

}

