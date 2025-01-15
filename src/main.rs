mod token;
mod tokenizer;
mod ast;
mod parser;
mod server;
mod optimizer;
mod compiler;


use std::env;
use crate::compiler::*;

use actix_web::web;
use actix_web::*;
use crate::server::*;

use crate::tokenizer::Tokenizer;
use crate::parser::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        HttpServer::new(|| {
            App::new()
                .route("/parse", web::post().to(parse))
                .route("/", web::get().to(index))
                .route("/index.js", web::get().to(js))
        })
        .bind("127.0.0.1:5000")?
        .run()
        .await
    } else {
        let s = std::fs::read_to_string("test.test").unwrap();
        let t = Tokenizer::new(s);

        let mut p = Parser::new(t);
        let stmt = p.parse_program();
        let mut c = Compiler::new(stmt, p.get_program());
        c.compile();
        println!("{}", c);

        Ok(())
    }
}


