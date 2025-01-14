use crate::token::*;


pub type ExpRef = usize;

pub type Program = Vec<Box<Expression>>;

pub enum Statement {
    ExpressionStatement(ExpRef)
}


#[derive(Clone)]
pub enum Expression {

    InfixExpression { left: ExpRef, op: TokenType, right: ExpRef },
    PrefixExpression { op: TokenType, right: ExpRef },
    AssignExpression { left: ExpRef, right: ExpRef },

    Integer(i64),
    String(String), 
    Identifier { value: String , ident_type: String},

}
