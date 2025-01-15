use actix_web::*;
use serde::Deserialize;
use serde::Serialize;
use crate::tokenizer::*;
use crate::parser::*;
use crate::ast::*;


pub async fn index() -> impl Responder {
    let s = std::fs::read_to_string("index.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(s)
}
pub async fn js() -> impl Responder {
    let s = std::fs::read_to_string("index.js").unwrap();
    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(s)
}


#[derive(Deserialize)]
pub struct SourceCode {
    code: String,
}
#[derive(Serialize)]
struct Response {
    stmt: Statement,
    program: Program,
}

pub async fn parse(data: web::Json<SourceCode>) -> impl Responder {
    let code = data.code.clone();
    println!("{}", code);
    let t = Tokenizer::new(code);
    let mut p = Parser::new(t);
    let stmt = p.parse_stmt();
    let program = p.get_program();
    web::Json(
        Response {
            stmt,
            program
        }
    )
}

