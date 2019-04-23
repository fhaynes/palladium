//! Contains the `Compiler` and `Visitor` trait. These are used to compile
//! a Palladium program to assembler code for the `Iridium` VM.
use std::collections::HashMap;
use iridium::assembler::Assembler;
use tokens::Token;
use scope::Scope;

pub trait Visitor {
    /// This function is called for ever Token in the AST
    fn visit_token(&mut self, node: &Token);
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
    scopes: Vec<Scope>,
    scope_pointer: usize,
    identifier_buffer: Vec<String>,
    in_return: bool
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
            scopes: vec![Scope::new()],
            scope_pointer: 1,
            identifier_buffer: vec![],
            in_return: false
        }
    }

    /// Takes a Vector of Strings that represent the text of a program and compiles
    /// it into bytecode
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

    pub fn get_variable(&self, variable: &str) -> Option<u8> {
        for scope in self.scopes.iter().rev() {
            if let Some(register) = scope.get_variable(variable) {
                return Some(register);
            }
        }
        None
    }

    pub fn new_variable(&mut self, identifier: &str, register: u8) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.new_variable(identifier, register);
        }
    }

    pub fn new_scope(&mut self) {
        self.scopes.push(Scope::new());
        self.scope_pointer += 1;
    }

    pub fn remove_scope(&mut self) {
        if let Some(newest_scope) = self.scopes.pop() {
            self.free_registers.extend(newest_scope.get_registers());
            self.scope_pointer -= 1;
        }
    }

    pub fn pop_current_scope(&mut self) -> Option<Scope> {
        self.scope_pointer -= 1;
        self.scopes.pop()
    }

    pub fn current_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    pub fn write_prologue(&mut self) {
        
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

            },
            &Token::Integer{ value } => {
                if self.in_return {
                    let line = format!("LOAD $31 #{}", value);
                    self.assembly.push(line);
                } else {
                    let next_register = self.free_registers.pop().unwrap();
                    let line = format!("LOAD ${} #{}", next_register, value);
                    {
                        let mut current_scope = self.current_scope();
                        current_scope.used_registers.push(next_register);
                    }
                    self.assembly.push(line);
                }

            },
            &Token::Float{ value } => {
                let next_register = self.free_registers.pop().unwrap();
                let line = format!("LOAD ${} #{}", next_register, value);
                self.used_registers.push(next_register);
                self.assembly.push(line);
            },
            &Token::Identifier{ ref values } => {
                for value in values {
                    self.identifier_buffer.push(value.to_string());
                }
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
                // If we are doing a variable assignment, we need to handle it a bit differently
                if right.len() > 0 {
                    if right[0].0 == Token::Assignment {
                        self.visit_token(left);
                        self.visit_token(&right[0].1);
                        let identifier = self.identifier_buffer.pop().unwrap();
                        let mut current_scope = self.pop_current_scope().unwrap();
                        self.new_variable(&identifier, 31);
                    }
                } else {
                    self.visit_token(left);
                    for factor in right {
                        self.visit_token(&factor.1);
                        self.visit_token(&factor.0);
                    }
                }
            },
            &Token::FunctionName{ ref name } => {

            },
            &Token::FunctionArgs{ ref args } => {
                for arg in args {
                    let next_register = self.free_registers.pop().unwrap();
                    self.new_variable(arg, next_register);
                }
            },
            &Token::FunctionBody{ ref expressions } => {
                for expr in expressions {
                    self.visit_token(expr);
                }
            },
            &Token::Function{ ref name, ref args, ref body, ref return_statement } => {
                self.new_scope();
                self.visit_token(args);
                let mut line = format!("{}:", Box::new(name));
                self.assembly.push(line);
                self.visit_token(&body);
                self.visit_token(&return_statement);
                self.assembly.push(format!("RET"));
            },
            &Token::FunctionCall{ ref name, ref parameters } => {
                match *parameters.clone() {
                    Token::FunctionArgs{ args } => {
                        for arg in args {
                            let register = self.get_variable(&arg);
                            let line = format!("PUSH {}", register.unwrap());
                            self.assembly.push(line);
                        }
                    },
                    _ => {}
                };
                let mut line = format!("CALL @{}", name);
                self.assembly.push(line);
            },
            &Token::ReturnStatement{ ref parameters } => {
                self.in_return = true;
                self.visit_token(&parameters);
                self.in_return = false;
            },
            &Token::ReturnArgs{ ref args } => {
                for arg in args {
                    self.visit_token(arg);
                }
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
            &Token::Eof => {

            },
            &Token::ListInterior{ ref body } => {

            },
            &Token::List { ref elements } => {

            },
            &Token::Dictionary { ref keys, ref values } => {

            },
            &Token::DictionaryKeyValuePair { ref key, ref value } => {

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
                self.assembly.push("HLT".into());
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
    fn test_function_call_with_args() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("a = 1\ndef testfunc(a):\n\t3+4\n\treturn 0;");
        compiler.visit_token(&test_program);
        println!("{:#?}", compiler.assembly);   
    }
    #[test]
    fn test_function_declaration() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("def testfunc():\n\t3+4\n\treturn 0;");
        compiler.visit_token(&test_program);
    }

    #[test]
    fn test_function_call_assignment() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("x = testfunc()");
        compiler.visit_token(&test_program);
    }

    #[test]
    fn test_function_return_values() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program(
r#"
def test():
    return 1;

y = test()
"#
        );
        compiler.visit_token(&test_program);
        println!("{:#?}", compiler.assembly);
    }
}
