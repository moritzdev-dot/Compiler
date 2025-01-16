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
    Call,
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
        match self.cur.token_type {
            TokenType::Plus => {
                self.shift();
                return self.parse(Prio::Prefix);

            }

            TokenType::Minus => {
                self.shift();
                let operand = self.parse(Prio::Prefix);
                self.new_expression(Box::new(
                    Expression::PrefixExpression { op: TokenType::Minus, right: operand }
                ))
            }

            _ => {
                panic!("WHY?");
            }

        }
    }
    pub fn get_program(&self) -> Program {
        return self.program.clone();
    }
    fn parse_block(&mut self) -> Vec<Box<Statement>> {
        self.shift();
        self.shift();
        let mut body: Vec<Box<Statement>> = Vec::new();
        while self.cur.token_type != TokenType::RBrace {
            let stmt = self.parse_stmt();
            body.push(Box::new(stmt));
        }
        return body;
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
    fn parse_param_list(&mut self) -> Vec<Parameter> {
        if self.cur.token_type != TokenType::LParent {
            panic!("No Left Parenteses Found");
        }
        self.shift();
        let mut list: Vec<Parameter> = Vec::new();
        while self.cur.token_type != TokenType::RParent {
            if self.cur.token_type != TokenType::Identifier {
                panic!("NOT AN IDENTIFIER IN PARAMETER LIST");
            }
            let name = self.cur.value.clone();
            self.shift();
            if self.cur.token_type != TokenType::Colon {
                panic!("NO TYPE ANOTATION");
            }
            self.shift();
            if self.cur.token_type != TokenType::Identifier {
                panic!("TYPE MISSING IN PARAMETER LIST");
            }
            let param_type = self.cur.value.clone();

            list.push(Parameter{
                name,
                param_type
            });

            self.shift();
            if self.cur.token_type == TokenType::Comma {
                self.shift();
            }
        }
        return list;
    }

    fn parse_list(&mut self) -> Vec<ExpRef> {
        if self.cur.token_type != TokenType::LParent {
            panic!("No Left Parenteses Found");
        }
        self.shift();
        let mut list: Vec<ExpRef> = Vec::new();
        while self.cur.token_type != TokenType::RParent {
            let exp = self.parse(Prio::None);
            list.push(exp);
            self.shift();
            if self.cur.token_type == TokenType::Comma {
                self.shift();
            }
        }
        return list;
    }
    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut v = Vec::new();
        while self.next.token_type != TokenType::EOF {
            v.push(self.parse_stmt());
        }
        return v;
    }
    pub fn parse_stmt(&mut self) -> Statement {
        let stmt = match self.cur.token_type {
            TokenType::Var => {
                self.shift();
                let name = self.cur.value.clone();
                if self.next.token_type != TokenType::Colon {
                    panic!("Missing Type");
                }
                self.shift();
                if self.next.token_type != TokenType::Identifier {
                    panic!("Missing Type");
                }
                self.shift();
                let var_type = self.cur.value.clone();
                if self.next.token_type == TokenType::Semicolon {
                    self.shift();
                    self.shift();
                    return Statement::VarStatement { 
                        name,
                        value: None,
                        var_type
                    }
                }
                self.shift();
                self.shift();
                let value = self.parse(Prio::None);
                self.shift();
                self.shift();
                return Statement::VarStatement { 
                    name,
                    value: Some(value),
                    var_type
                }
            }
            TokenType::If => {
                self.shift();
                let cond = self.parse(Prio::None);
                let if_block = self.parse_block();
                if self.next.token_type != TokenType::Else {
                    return Statement::IfElseStatement { 
                        condition: cond, 
                        if_body: if_block, 
                        else_body: None 
                    }
                }
                self.shift();
                let else_block = self.parse_block();
                Statement::IfElseStatement { 
                    condition: cond, 
                    if_body: if_block, 
                    else_body: Some(else_block)
                }

            }
            TokenType::Return => {
                self.shift();
                let s = Statement::ReturnStatement{
                    value: self.parse(Prio::None)
                };
                self.shift();
                self.shift();
                s
            }
            TokenType::Func => {
                self.shift();
                let name = self.cur.value.clone();
                self.shift();
                let list = self.parse_param_list();
                let body = self.parse_block();
                self.shift();
                Statement::FuncStatement { 
                    name: name.clone(),
                    call_inputs: list,
                    body 
                }
            }
            _ => {
                let s = Statement::ExpressionStatement(self.parse(Prio::None));
                self.shift();
                self.shift();
                s
            }
        };
        return stmt;

    }
    fn get_prio(t: &TokenType) -> Prio {
        match t {
            TokenType::Plus | TokenType::Minus => {
                return Prio::Add
            }
            TokenType::Astrik | TokenType::Slash => {
                return Prio::Mult
            }
            TokenType::LParent => {
                return Prio::Call
            }
            TokenType::Assign => {
                return Prio::Assign
            }
            TokenType::And => {
                return Prio::And
            }
            TokenType::Or => {
                return Prio::Or
            }
            TokenType::LT | TokenType::GT | TokenType::LTEQ | TokenType::GTEQ => {
                return Prio::Compare
            }
            TokenType::EQ => {
                return Prio::Equal;
            }


            _ => {
                return Prio::None
            }

        }
    }

    fn parse_infix(&mut self, left: ExpRef) -> ExpRef {
        let op = self.cur.token_type.clone();
        let p = Self::get_prio(&self.cur.token_type);
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
            TokenType::Identifier => {
                self.new_expression(Box::new(Expression::Identifier {
                    value: self.cur.value.clone(),
                    ident_type: String::new(), 
                }))
            }
            TokenType::LParent => {
                self.shift();
                let l = self.parse(Prio::None);
                if self.next.token_type != TokenType::RParent {
                    panic!("NO RIGHT PARENTH FOUND");
                }
                self.shift();

                l
            }

            _ => {
                panic!("NOT VALID TOKENTYPE: {}",self);
            }
        };
        while !(self.next.token_type == TokenType::Semicolon) && p < Self::get_prio(&self.next.token_type){
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
            Expression::Identifier { value, ident_type } => {
                return format!("{}", value);
            }
            _ => {
                return format!("");
            }
        }
    }
    pub fn print_stmt(&self, stmt: Statement) {
        print!("{}", self.stmt_to_string(stmt, 0));
    }
    pub fn stmt_to_string(&self, stmt: Statement, ident: i64) -> String{
        let mut indent = String::new();
        for _ in 0..ident {
            indent += "\t";
        }
        match stmt {
            Statement::IfElseStatement { condition, if_body, else_body } => {
                let mut val = indent.clone();
                val += &format!("if({}) {{\n", self.exp_to_string(condition));
                for i in if_body {
                    val += &indent;
                    val += &self.stmt_to_string(*i, ident + 1);
                }
                val += &indent;
                val += "}";
                if else_body.is_none() {
                    return val + "\n";
                }
                val += " else {\n";
                for i in else_body.unwrap() {
                    val += &indent;
                    val += &self.stmt_to_string(*i, ident + 1);
                }
                val += &indent;
                val += "}\n";
                return val;
            }
            Statement::FuncStatement { name, call_inputs, body } => {
                let s = call_inputs
                    .iter()
                    .map(|x| format!("{}: {}", x.name, x.param_type))
                    .collect::<Vec<String>>()
                    .join(", ");

                let mut val = indent.clone();
                val += &format!("func {}({}) {{\n", name, s);
                for i in body {
                    val += &indent;
                    val += &self.stmt_to_string(*i, ident + 1);
                }
                val += &indent;
                val += "}\n";
                return val;
            }
            Statement::VarStatement { name, value, var_type } => {
                let mut val = indent.clone();
                val += &format!("var {}: {}", name, var_type);
                if value.is_none() {
                    return val + "\n";
                }
                let v = value.unwrap();
                val += &format!(" = {}\n", self.exp_to_string(v));
                return val;
            }
            Statement::ReturnStatement { value } => {
                if ident > 0 {
                    return format!("\treturn {}\n", self.exp_to_string(value));
                } 
                return format!("return {}\n", self.exp_to_string(value));
            }
            Statement::ExpressionStatement(exp) => {
                if ident > 0 {
                    return format!("\t{}\n", self.exp_to_string(exp));
                } 
                return format!("{}\n", self.exp_to_string(exp));
            }
        }
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
