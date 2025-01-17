use std::{collections::HashMap, fmt};

use symbols::*;

use crate::{ast::{ExpRef, Expression, Parameter, Program, Statement}, token::TokenType};
mod symbols;



#[derive(Debug)]
enum OpCodeTypes {
    Add,
    Sub,
    Mul,
    Mov, 
    Push,
    Pop,
    Xor,
    Call,
    Func(String),
    Global, 
    Extern,
    Leave,
    Ret,
}
impl fmt::Display for OpCodeTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCodeTypes::Func(s) => {
                write!(f, "{}:", s)
            }
            OpCodeTypes::Extern => {
                write!(f, "extern")
            }
            OpCodeTypes::Global=> {
                write!(f, "global")
            }
            _ => { 
                write!(f, "{}", format!("\t{:?}", self).to_uppercase())
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
    EAX,
    RAX,
    RDI,
    RSI,
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
    table: SymbolTable,
    data_section: Vec<Instruction>,
    functions: HashMap<String, Vec<Parameter>>,
}

impl Compiler {
    pub fn new(stmts: Vec<Statement>, program: Program) -> Self{
        return Self {
            stmts,
            program,
            output: Vec::new(),
            data_section: Vec::new(),
            table: SymbolTable::new(),
            functions: HashMap::new(),

        };
    }


    fn new_instruction(&mut self, opcode: OpCodeTypes, operands: Vec<String>) {
        let instruction = Instruction{
            opcode,
            operands
        };
        self.output.push(instruction);
    }
    fn alloc(&mut self, bytes: u32) {
        self.new_instruction(OpCodeTypes::Sub, vec![Registers::RSP.to_string(), format!("{}", bytes)]);
    }

    fn push_reg(&mut self, reg: Registers) {
        self.new_instruction(OpCodeTypes::Push, vec![reg.to_string()]);
    }
    fn push_const(&mut self, constant: String) {
        self.new_instruction(OpCodeTypes::Push, vec![constant]);
    }

    fn pop(&mut self, reg: Registers) {
        self.new_instruction(OpCodeTypes::Pop, vec![reg.to_string()]);
    }

    fn register_op(&mut self, opcode: OpCodeTypes, reg1: Registers, reg2: Registers) {
        self.new_instruction(opcode, vec![reg1.to_string(), reg2.to_string()]);
    }

    fn compile_infix(&mut self, left: ExpRef, right: ExpRef, op: TokenType) {
        self.compile_expression(left);
        self.compile_expression(right);
        self.pop(Registers::RBX);
        self.pop(Registers::RAX);
        match op {
            TokenType::Plus => {
                self.register_op(OpCodeTypes::Add, Registers::RAX, Registers::RBX);
                self.new_instruction(OpCodeTypes::Push, vec![Registers::RAX.to_string()]);
            }
            TokenType::Minus => {
                self.register_op(OpCodeTypes::Sub, Registers::RAX, Registers::RBX);
                self.new_instruction(OpCodeTypes::Push, vec![Registers::RAX.to_string()]);
            }
            TokenType::Astrik => {
                self.register_op(OpCodeTypes::Mul, Registers::RAX, Registers::RBX);
                self.new_instruction(OpCodeTypes::Push, vec![Registers::RAX.to_string()]);
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
                self.new_instruction(OpCodeTypes::Push, vec![format!("{}", i)]);
            }
            Expression::Identifier { value, ident_type } => {
                let s = self.table.get(value);
                if s.is_none() {
                    return;
                };
                self.get_from_stack(s.unwrap().offset, Registers::RAX);
                self.new_instruction(OpCodeTypes::Push, vec![Registers::RAX.to_string()]);
            }
            Expression::FunctionCall { left, parameters } => {
                let (func_params, name) = match *self.program[left].clone() {
                    Expression::Identifier { value, ident_type } => {
                        let par = self.functions.get(&value);
                        (par.unwrap(), value)
                    }
                    _ => {
                        panic!();
                    }
                };
                if func_params.len() != parameters.len() {
                    panic!("NOT THE SAME EMOUNT OF PARAMETERS");
                }
                for par in parameters.iter().rev() {
                    self.compile_expression(*par);
                }
                self.new_instruction(OpCodeTypes::Call, vec![name]);
            }
            _ => {}
        }
    }

    fn store_reg_on_stack(&mut self, offset: u64, reg: Registers) {
        self.new_instruction(OpCodeTypes::Mov, vec![
            format!("QWORD [rbp-{}]", offset),
            reg.to_string()
        ]);
    }

    fn get_from_stack(&mut self, offset: u64, reg: Registers) {
        self.new_instruction(OpCodeTypes::Mov, vec![
            reg.to_string(),
            format!("QWORD [rbp-{}]", offset),
        ]);
    }

    fn setup_stackfram(&mut self) {
        self.push_reg(Registers::RBP);
        self.register_op(OpCodeTypes::Mov, Registers::RBP, Registers::RSP);
    }

    fn cleanup_stackframe(&mut self) {
        self.register_op(OpCodeTypes::Mov, Registers::RSP, Registers::RBP);
        self.pop(Registers::RBP);
    }

    fn print_builtin(&mut self) {
        self.setup_stackfram();
        self.new_instruction(OpCodeTypes::Mov, vec![
            Registers::RDI.to_string(),
            String::from("fmt")
        ]);
        self.new_instruction(OpCodeTypes::Mov, vec![
            Registers::RSI.to_string(),
            format!("[RBP + 16]")
        ]);
        self.new_instruction(OpCodeTypes::Xor, vec![
            Registers::RAX.to_string(),
            Registers::RAX.to_string(),
        ]);
        self.new_instruction(OpCodeTypes::Call, vec![
            String::from("printf")
        ]);
        //self.pop(Registers::RBP);
        self.new_instruction(OpCodeTypes::Leave, vec![]);
        self.new_instruction(OpCodeTypes::Ret, vec![]);
    }

    pub fn compile_stmt(&mut self, stmt: Statement) {
        match stmt {
            Statement::FuncStatement { name, call_inputs, body } => {
                self.functions.insert(name.clone(), call_inputs.clone());
                self.new_instruction(OpCodeTypes::Func(name.clone()), vec![]);
                self.setup_stackfram();
                self.alloc(16);
                let idx = self.output.len();
                self.table = SymbolTable::new_from_outer(self.table.clone());
                let mut offset = 8;
                for inp in call_inputs {
                    offset += 8;
                    self.table.add(inp.name, Symbol{
                        symb_type: inp.param_type,
                        offset
                    });
                    self.new_instruction(OpCodeTypes::Mov, vec![
                        Registers::RAX.to_string(),
                        format!("QWORD [RBP + {}]", offset)
                    ]);
                    self.new_instruction(OpCodeTypes::Mov, vec![
                        format!("QWORD [RBP - {}]", offset),
                        Registers::RAX.to_string(),
                    ]);
                }
                for i in body {
                    self.compile_stmt(*i);
                }
                self.output[idx-1] = Instruction{
                    opcode: OpCodeTypes::Sub,
                    operands: vec![
                        Registers::RSP.to_string(),
                        format!("{}", std::cmp::max(self.table.cur_offset, 16))
                    ]
                };

                self.table = self.table.move_out();
                if name != "main" {
                    self.cleanup_stackframe();
                    self.new_instruction(OpCodeTypes::Ret, vec![]);
                } else {
                    self.register_op(
                        OpCodeTypes::Xor,
                        Registers::EAX,
                        Registers::EAX
                    );
                    self.new_instruction(OpCodeTypes::Leave, vec![]);
                    self.new_instruction(OpCodeTypes::Ret, vec![]);
                }
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
    pub fn add_builtin_function(&mut self, name: String) {
        self.functions.insert(
            name.clone(),
            vec![
                Parameter{
                    name: "x".to_string(),
                    param_type: "int".to_string()
                }
            ]
        );
        self.new_instruction(OpCodeTypes::Func(name), vec![]);
        self.print_builtin();
    }

    pub fn compile(&mut self) {
        self.new_instruction(OpCodeTypes::Global, vec![
            String::from("main"),
        ]);
        self.new_instruction(OpCodeTypes::Extern, vec![
            String::from("printf"),
        ]);
        self.add_builtin_function(String::from("print"));
        for stmt in self.stmts.clone() {
            self.compile_stmt(stmt);
        }
    }

}


impl std::fmt::Display for Instruction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.operands.join(", ");
        write!(f, "{} {}", format!("{}", self.opcode), s)
    }
}

impl std::fmt::Display for Compiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.data_section.len() >= 0 {
            write!(f, "section .data\n")?;
            write!(f, "\tfmt db \"%d \", 0")?;
            write!(f, "{}", self.data_section.iter()
                .map(|inst| format!("{}", inst))
                .collect::<Vec<String>>()
                .join("\n")
            )?;
            write!(f, "\n")?;
        }
        write!(f, "section .text\n")?;
        write!(f, "{}", self.output.iter()
            .map(|inst| format!("{}", inst))
            .collect::<Vec<String>>()
            .join("\n")
        )
    }
}

