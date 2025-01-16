use std::fmt;

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
    Pop,
    Func(String),
    Ret,
}
impl fmt::Display for OpCodeTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCodeTypes::Func(s) => {
                write!(f, "{}:", s)
            }
            _ => { 
                write!(f, "\t{:?}", self) 
            }
        }
    }
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
        self.pop(Registers::RBX);
        self.pop(Registers::RAX);
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
            Expression::Identifier { value, ident_type } => {
                let s = self.table.get(value);
                if s.is_none() {
                    return;
                };
                self.get_from_stack(s.unwrap().offset, Registers::RAX);
                self.create_instruction(OpCodeTypes::Push, vec![Registers::RAX.to_string()]);
            }
            _ => {}
        }
    }
    fn store_reg_on_stack(&mut self, offset: u64, reg: Registers) {
        self.create_instruction(OpCodeTypes::Mov, vec![
            format!("QWORD [rbp-{}]", offset),
            reg.to_string()
        ]);
    }
    fn get_from_stack(&mut self, offset: u64, reg: Registers) {
        self.create_instruction(OpCodeTypes::Mov, vec![
            reg.to_string(),
            format!("QWORD [rbp-{}]", offset),
        ]);
    }


    fn push_reg(&mut self, reg: Registers) {
        self.create_instruction(OpCodeTypes::Push, vec![reg.to_string()]);
    }


    fn setup_stackfram(&mut self) {
        self.push_reg(Registers::RBP);
        self.create_instruction(OpCodeTypes::Mov, vec![
            Registers::RBP.to_string(),
            Registers::RSP.to_string(),
        ]);
    }

    fn cleanup_stackframe(&mut self) {
        self.create_instruction(OpCodeTypes::Mov, vec![
            Registers::RSP.to_string(),
            Registers::RBP.to_string(),
        ]);
        self.pop(Registers::RBP);
    }

    pub fn compile_stmt(&mut self, stmt: Statement) {
        match stmt {
            Statement::FuncStatement { name, call_inputs, body } => {
                if name == "main" {
                }
                self.create_instruction(OpCodeTypes::Func(name.clone()), vec![]);
                self.setup_stackfram();
                self.allocate_memory(128);
                let idx = self.output.len();
                self.table = SymbolTable::new_from_outer(self.table.clone());
                let mut offset = 0;
                for inp in call_inputs {
                    offset += 8;
                    self.table.add(inp.name, Symbol{
                        symb_type: inp.param_type,
                        offset
                    });
                }
                for i in body {
                    self.compile_stmt(*i);
                }
                self.output[idx-1] = Instruction{
                    opcode: OpCodeTypes::Sub,
                    operands: vec![
                        Registers::RBP.to_string(),
                        format!("{}", self.table.cur_offset)
                    ]
                };
                self.table = self.table.move_out();
                self.cleanup_stackframe();
                self.create_instruction(OpCodeTypes::Ret, vec![]);
            }
            Statement::VarStatement { name, value, var_type } => {
                let offset = self.table.cur_offset + 8;
                self.table.add(name, Symbol{
                    symb_type: var_type,
                    offset 
                });
                if value.is_some() {
                    self.compile_expression(value.unwrap());
                    self.pop(Registers::RAX);
                    self.store_reg_on_stack(offset, Registers::RAX);
                }
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
        write!(f, "{} {}", format!("{}", self.opcode).to_uppercase(), s)
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
