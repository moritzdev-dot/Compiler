use crate::token::*;


pub type ExpRef = usize;

pub type Program = Vec<Box<Expression>>;

pub enum Statement {
    IfElseStatement {
        condition: ExpRef,
        if_body: Vec<Box<Statement>>,
        else_body: Option<Vec<Box<Statement>>>
    },
    FuncStatement {
        name: String,
        call_inputs: Vec<ExpRef>,
        body: Vec<Box<Statement>>,
    },

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
