use crate::ast::*;
use crate::tokenizer::*;
use crate::token::*;


#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Prio {
    None,

    Assign,
    Or,
    And,
    Equal,
    Compare,
    Add,
    Mult,
    Prefix,
}


pub struct Parser {
    t: Tokenizer,
    cur: Token,
    next: Token,
    program: Program,
}


impl Parser {
    pub fn new(mut t: Tokenizer) -> Self {
        return Parser {
            cur: t.next_token(),
            next: t.next_token(),
            program: Vec::new(),
            t,
        };
    }


    fn shift(&mut self) {
        self.cur = self.next.clone();
        self.next = self.t.next_token();
    }

    fn parse_prefix(&mut self) -> ExpRef{
        panic!("NOT IMPLEMENTED");
    }
    
    fn parse_literal(&mut self) -> ExpRef {
        match self.cur.token_type {
            TokenType::String => {
                let exp = Expression::String(
                    self.cur.value.clone()
                );
                self.new_expression(Box::new(exp))
            }
            TokenType::Integer => {
                let exp = Expression::Integer(
                    self.cur.value.parse::<i64>().unwrap()
                );
                self.new_expression(Box::new(exp))
            }
            _ => {
                panic!("I Don't Know What Happend Here");
            }
        }
    }
    pub fn parse_stmt(&mut self) -> Statement {
        let stmt = match self.cur.token_type {
            _ => {
                Statement::ExpressionStatement(self.parse(Prio::None))
            }
        };
        self.shift();
        self.shift();
        return stmt;

    }
    fn get_prioritie(t: &TokenType) -> Prio {

        match t {
            TokenType::Plus | TokenType::Minus => {
                return Prio::Add
            }
            TokenType::Astrik | TokenType::Slash => {
                return Prio::Mult
            }
            _ => {
                panic!("Prio for {:?} not implemented", t);
            }

        }
    }

    fn parse_infix(&mut self, left: ExpRef) -> ExpRef {
        let op = self.cur.token_type.clone();
        let p = Self::get_prioritie(&self.cur.token_type);
        self.shift();
        let right = self.parse(p);
        let exp = Expression::InfixExpression {
            left,
            op,
            right,
        };

        return self.new_expression(Box::new(exp))
    }

    fn parse(&mut self, p: Prio) -> ExpRef  {
        let mut left = match self.cur.token_type {
            TokenType::Plus | TokenType::Minus=> {
                self.parse_prefix()
            }
            TokenType::Integer | TokenType::String => {
                self.parse_literal()
            }
            _ => {
                panic!("NOT VALID TOKENTYPE: {}", self.cur);
            }
        };
        while !(self.next.token_type == TokenType::Semicolon) && p < Self::get_prioritie(&self.next.token_type){
            self.shift();
            left = self.parse_infix(left);
        }

        return left;

    }
    fn exp_to_string(&self, exp: ExpRef) -> String{
        let e = self.program[exp].clone();
        match *e {
            Expression::InfixExpression { left, op, right } => {
                let l_string = self.exp_to_string(left);
                let r_string = self.exp_to_string(right);
                return format!("({} {:?} {})", l_string, op, r_string);
            }
            Expression::Integer(i) => {
                return format!("{}", i);
            }
            Expression::String(i) => {
                return format!("{}", i);
            }
            _ => {
                return format!("");
            }
        }
    }
    pub fn print_stmt(&self, stmt: Statement) {
        match stmt {
            Statement::ExpressionStatement(exp) => {
                println!("{}", self.exp_to_string(exp))
            }
        }
        println!()
    }

    fn new_expression(&mut self, exp: Box<Expression>) -> ExpRef {
        self.program.push(exp);
        return self.program.len() - 1;
    }

}

impl std::fmt::Display for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Current Token: {}, Next Token {}", self.cur, self.next)
    }
}
