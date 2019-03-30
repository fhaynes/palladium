use iridium::assembler::Assembler;
use tokens::Token;

pub trait Visitor {
    fn visit_token(&mut self, node: &Token);
}

#[derive(Default)]
pub struct Compiler{
    free_registers: Vec<u8>,
    used_registers: Vec<u8>,
    assembly: Vec<String>,
    assembler: Assembler,
}

impl Compiler {
    pub fn new() -> Compiler {
        let mut free_registers = vec![];
        for i in 0..31 {
            free_registers.push(i);
        }
        free_registers.reverse();
        Compiler{
            free_registers: free_registers,
            used_registers: vec![],
            assembly: vec![],
            assembler: Assembler::new()
        }
    }

    pub fn compile(&mut self) -> Vec<u8> {
        let program = self.assembly.join("\n");
        let bytecode = self.assembler.assemble(&program);
        match bytecode {
            Ok(b) => { b },
            Err(_e) => { vec![] }
        }
    }

    pub fn print_asm(&self) {
        for line in &self.assembly {
            println!("{:#?}", line);
        }
    }

    pub fn print_used_registers(&self) {
        println!("--------------------");
        println!("|  Used Registers  |");
        println!("--------------------");
        for r in &self.used_registers {
            println!("{:#?}", r);
        }
    }

    pub fn print_free_registers(&self) {
        println!("--------------------");
        println!("|  Free Registers  |");
        println!("--------------------");
        for r in &self.free_registers {
            println!("{:#?}", r);
        }
    }
}

impl Visitor for Compiler {
    fn visit_token(&mut self, node: &Token) {
        match node {
            &Token::AdditionOperator => {
                // TODO: Need to clean this up. Remove the unwraps.
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!("ADD ${} ${} ${}", left_register, right_register, result_register);
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            },
            &Token::SubtractionOperator => {
                // TODO: Need to clean this up. Remove the unwraps.
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!("SUB ${} ${} ${}", right_register, left_register, result_register);
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            },
            &Token::MultiplicationOperator => {
                // TODO: Need to clean this up. Remove the unwraps.

                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!("MUL ${} ${} ${}", left_register, right_register, result_register);
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            },
            &Token::DivisionOperator => {
                // TODO: Need to clean this up. Remove the unwraps.
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!("DIV ${} ${} ${}", left_register, right_register, result_register);
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            },
            &Token::GreaterThan => {

            },
            &Token::LessThan => {

            },
            &Token::Integer{ value } => {
                let next_register = self.free_registers.pop().unwrap();
                let line = format!("LOAD ${} #{}", next_register, value);
                self.used_registers.push(next_register);
                self.assembly.push(line);
            },
            &Token::Float{ value } => {
                let next_register = self.free_registers.pop().unwrap();
                let line = format!("LOAD ${} #{}", next_register, value);
                self.used_registers.push(next_register);
                self.assembly.push(line);
            },
            &Token::Identifier{ ref value } => {

            },
            &Token::Factor{ ref value } => {
                self.visit_token(value);
            },
            &Token::Term{ ref left, ref right } => {
                self.visit_token(left);
                for factor in right {
                    self.visit_token(&factor.1);
                    self.visit_token(&factor.0);
                }
            },
            &Token::FunctionName{ ref name } => {

            },
            &Token::FunctionArgs{ ref args } => {

            },
            &Token::FunctionBody{ ref expressions } => {

            },
            &Token::Function{ ref name, ref args, ref body } => {

            },
            &Token::ForLoop{ ref start, ref body } => {

            },
            &Token::ForLoopStart{ ref variable_name, ref collection_name } => {

            },
            &Token::ForLoopBody{ ref expressions } => {

            },
            &Token::WhileLoop{ ref start, ref body } => {

            },
            &Token::WhileLoopStart{ ref expression } => {

            },
            &Token::WhileLoopBody{ ref expressions } => {

            },
            &Token::Expression{ ref left, ref right } => {
                self.visit_token(left);
                for term in right {
                    self.visit_token(&term.1);
                    self.visit_token(&term.0);
                }

            },
            &Token::Program{ ref expressions } => {
                self.assembly.push(".data".into());
                self.assembly.push(".code".into());
                for expression in expressions {
                    self.visit_token(expression);
                }
            }
        }
    }
}

mod tests {
    use super::*;
    use nom::types::CompleteStr;
    use program_parsers::program;

    fn generate_test_program(expr: &str) -> Token {
        let source = CompleteStr(expr);
        let (_, tree) = program(source).unwrap();
        tree
    }

    #[test]
    fn test_visit_addition_token() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1+2");
        compiler.visit_token(&test_program);
    }

    #[test]
    fn test_visit_subtraction_token() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("2-1");
        compiler.visit_token(&test_program);
    }

    #[test]
    fn test_visit_multiplication_token() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("2*1");
        compiler.visit_token(&test_program);
        let bytecode = compiler.compile();
    }

    #[test]
    fn test_nested_operators() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("(4*3)-1");
        compiler.visit_token(&test_program);
        let bytecode = compiler.compile();
    }
}
