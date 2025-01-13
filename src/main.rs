mod token;
mod tokenizer;
use crate::tokenizer::Tokenizer;
use crate::token::*;

fn main() {
    let s = String::from("123 + \"Hallo\"");
    let mut t = Tokenizer::new(s);
    let mut token = t.next_token();
    while match token.token_type {
        TokenType::EOF => {
            false
        },
        _ => {
            true
        }
    }{

        println!("{}", token);
        token = t.next_token();


    }
}
