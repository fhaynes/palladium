//! Contains the `Compiler` and `Visitor` trait. These are used to compile
//! a Palladium program to assembler code for the `Iridium` VM.
use std::collections::HashMap;
use iridium::assembler::Assembler;
use tokens::Token;
use scope::Scope;

pub trait Visitor {
    /// This function is called for ever Token in the AST
    fn visit_token(&mut self, node: &Token);
    /// Convenience function to write out the assembly for creating a stack frame
    fn create_stack_frame(&mut self);
}

#[derive(Default)]
/// Compiles the code into assembly. Also contains an Assembler so it can just
/// write out bytecode. It also handles register allocation.
pub struct Compiler {
    /// Unused Registers
    free_registers: Vec<u8>,
    /// Used Registers
    used_registers: Vec<u8>,
    /// The assembly statements created so far. These are just Strings that are
    /// emitted by the `Compiler` as it walks the tree
    assembly: Vec<String>,
    /// An `Assembler` for the Iridium VM, so the `Compiler` can emit bytecode
    /// directly
    assembler: Assembler,
    variables: HashMap<String, String>,
    scopes: Vec<Scope>,
    scope_pointer: usize
}

impl Compiler {
    /// Creates and returns a new `Compiler`
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
            assembler: Assembler::new(),
            variables: HashMap::new(),
            scopes: vec![Scope::new()],
            scope_pointer: 0
        }
    }

    /// Takes a 
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
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!("GT ${} ${} ${}", left_register, right_register, result_register);
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            },
            &Token::LessThan => {
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!("LT ${} ${} ${}", left_register, right_register, result_register);
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            },
            &Token::GreaterThanOrEqual => {
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!("GTE ${} ${} ${}", left_register, right_register, result_register);
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            },
            &Token::LessThanOrEqual => {
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!("LTE ${} ${} ${}", left_register, right_register, result_register);
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            },
            &Token::EqualTo => {
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!("EQ ${} ${} ${}", left_register, right_register, result_register);
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            },
            &Token::LogicalAnd => {
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!("AND ${} ${} ${}", left_register, right_register, result_register);
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            },
            &Token::LogicalNot => {
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let line = format!("NOT ${} ${}", left_register, result_register);
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
            },
            &Token::LogicalOr => {
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!("OR ${} ${} ${}", left_register, right_register, result_register);
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            },
            &Token::Assignment => {
                let right_register = self.used_registers.pop().unwrap();
                let variable_register = self.used_registers.pop().unwrap();
                let line = format!("LOAD ${} ${}", right_register, variable_register);
                self.assembly.push(line);
                self.used_registers.push(variable_register);
                self.free_registers.push(right_register);
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
                let current_scope = &self.scopes[self.scope_pointer];
                let save_register = self.free_registers.pop().unwrap();
                self.variables.insert(value.to_string(), save_register.to_string());
                self.used_registers.push(save_register);
            },
            &Token::If{ ref expr, ref body} => {

            },
            &Token::Elif{ ref expr, ref body} => {

            },
            &Token::Else{ ref body } => {

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
                let mut line = format!("{}:", Box::new(name));
                self.assembly.push(line);
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

    fn create_stack_frame(&mut self) {
        unimplemented!()
    }
}

mod tests {
    use super::*;
    use nom::types::CompleteStr;
    use program_parsers::program;

    fn generate_test_program(expr: &str) -> Token {
        let source = CompleteStr(expr);
        let result = program(source);
        let (_, tree) = result.unwrap();
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
    fn test_visit_division_token() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("2/1");
        compiler.visit_token(&test_program);
        let bytecode = compiler.compile();
    }

    #[test]
    fn test_visit_greater_than_token() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("2>1");
        compiler.visit_token(&test_program);
        let bytecode = compiler.compile();
    }

    #[test]
    fn test_visit_less_than_token() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("2<1");
        compiler.visit_token(&test_program);
        let bytecode = compiler.compile();
    }

    #[test]
    fn test_visit_less_than_or_equal_token() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("2<=1");
        compiler.visit_token(&test_program);
        let bytecode = compiler.compile();
    }

    #[test]
    fn test_visit_greater_than_or_equal_token() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("2>=1");
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

    #[test]
    fn test_variable_assignment() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("x = 4");
        compiler.visit_token(&test_program);
        let bytecode = compiler.compile();
    }

    #[test]
    fn test_function_declaration() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("def testfunc():\n3+4\n");
        compiler.visit_token(&test_program);
        println!("{:?}", compiler.assembly);
    }
}
