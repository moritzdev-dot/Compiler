use symbols::*;

use crate::{ast::{ExpRef, Expression, Program, Statement}, token::TokenType};
mod symbols;



#[derive(Debug)]
enum OpCodeTypes {
    Add,
    Sub,
    Mul,
    Mov, 
    Push,
    Pop
}

pub struct Instruction {
    opcode: OpCodeTypes,    
    operands: Vec<String>
}
#[derive(Debug)]
pub enum Registers {
    RAX,
    RBI,
    RBX,
    RSP,
    RBP, 
}

impl Registers {
    pub fn to_string(&self) -> String {
        return format!("{:?}", self).to_uppercase()
    }
}

pub struct Compiler {
    stmts: Vec<Statement>,
    program: Program,
    output: Vec<Instruction>,
    table: SymbolTable
}

impl Compiler {
    pub fn new(stmts: Vec<Statement>, program: Program) -> Self{
        return Compiler {
            stmts,
            program,
            output: Vec::new(),
            table: SymbolTable::new()
        };
    }

    fn create_instruction(&mut self, opcode: OpCodeTypes, operands: Vec<String>) {
        let instruction = Instruction{
            opcode,
            operands
        };
        self.output.push(instruction);
    }
    fn allocate_memory(&mut self, bytes: u32) {
        self.create_instruction(OpCodeTypes::Sub, vec![Registers::RBP.to_string(), format!("{}", bytes)]);
    }
    fn mov_reg_to_rax(&mut self, reg: Registers) {
        self.create_instruction(OpCodeTypes::Mov, vec![Registers::RAX.to_string(), reg.to_string()]);
    }
    fn mov_to_rax(&mut self, constant: String) {
        self.create_instruction(OpCodeTypes::Mov, vec![Registers::RAX.to_string(), constant]);
    }

    fn pop(&mut self, reg: Registers) {
        self.create_instruction(OpCodeTypes::Pop, vec![reg.to_string()]);
    }

    fn register_op(&mut self, opcode: OpCodeTypes, reg1: Registers, reg2: Registers) {
        self.create_instruction(opcode, vec![reg1.to_string(), reg2.to_string()]);
    }

    fn compile_infix(&mut self, left: ExpRef, right: ExpRef, op: TokenType) {
        self.compile_expression(left);
        self.compile_expression(right);
        self.pop(Registers::RAX);
        self.pop(Registers::RBX);
        match op {
            TokenType::Plus => {
                self.register_op(OpCodeTypes::Add, Registers::RAX, Registers::RBX);
                self.create_instruction(OpCodeTypes::Push, vec![Registers::RAX.to_string()]);
            }
            TokenType::Minus => {
                self.register_op(OpCodeTypes::Sub, Registers::RAX, Registers::RBX);
                self.create_instruction(OpCodeTypes::Push, vec![Registers::RAX.to_string()]);
            }
            TokenType::Astrik => {
                self.register_op(OpCodeTypes::Mul, Registers::RAX, Registers::RBX);
                self.create_instruction(OpCodeTypes::Push, vec![Registers::RAX.to_string()]);
            }
            _ => {
                panic!();
            }
        }
    }

    pub fn compile_expression(&mut self, exp: ExpRef) {
        let expression = *self.program[exp].clone();
        match expression {
            Expression::InfixExpression { left, op, right } => {
                self.compile_infix(left, right, op);
            }
            Expression::Integer(i) => {
                self.create_instruction(OpCodeTypes::Push, vec![format!("{}", i)]);
            }
            _ => {}
        }
    }
    fn store_on_stack(&mut self, offset: u32, constant: String) {
        self.create_instruction(OpCodeTypes::Mov, vec![
            format!("QWORD [rbp-{}]", offset),
            constant
        ]);
    }
    fn store_ref_on_stack(&mut self, offset: u32, constant: Registers) {
        self.create_instruction(OpCodeTypes::Mov, vec![
            format!("QWORD [rbp-{}]", offset),
            constant.to_string()
        ]);
    }

    pub fn compile_stmt(&mut self, stmt: Statement) {
        match stmt {
            Statement::VarStatement { name, value, var_type } => {
            }
            Statement::ExpressionStatement(exp) => {
                self.compile_expression(exp)
            }
            _ => {}
        }
    }

    pub fn compile(&mut self) {
        for stmt in self.stmts.clone() {
            self.compile_stmt(stmt);
        }
    }

}


impl std::fmt::Display for Instruction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.operands.join(", ");
        write!(f, "{} {}", format!("{:?}", self.opcode).to_uppercase(), s)
    }
}

impl std::fmt::Display for Compiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.output.iter()
            .map(|inst| format!("{}", inst))
            .collect::<Vec<String>>()
            .join("\n")
        )
    }
}
