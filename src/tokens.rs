use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Eof,
    AdditionOperator,
    SubtractionOperator,
    MultiplicationOperator,
    DivisionOperator,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    EqualTo,
    LogicalOr,
    LogicalAnd,
    LogicalNot,
    Assignment,
    Integer { value: i64 },
    Float { value: f64},
    Identifier { values: Vec<String> },
    Factor { value: Box<Token> },
    Term { left: Box<Token>, right: Vec<(Token, Token)> },
    If { expr: Box<Token>, body: Vec<Token> },
    Elif { expr: Box<Token>, body: Vec<Token> },
    Else { body: Vec<Token> },
    FunctionCall { name: String, parameters: Box<Token> },
    FunctionName { name: String },
    FunctionArgs { args: Vec<String> },
    FunctionBody { expressions: Vec<Token> },
    Function { name: Box<Token>, args: Box<Token>, body: Box<Token>, return_statement: Box<Token> },
    ReturnArgs { args: Vec<Token> },
    ReturnStatement { parameters: Box<Token> },
    WhileLoop { start: Box<Token>, body: Box<Token> },
    WhileLoopStart { expression: Box<Token> },
    WhileLoopBody { expressions: Vec<Token> },
    ForLoop { start: Box<Token>, body: Box<Token> },
    ForLoopStart { variable_name: String, collection_name: String},
    ForLoopBody { expressions: Vec<Token> },
    Expression { left: Box<Token>, right: Vec<(Token, Token)> },
    Program { expressions: Vec<Token> },
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::FunctionName{ name } => {
                write!(f, "{}", name)
            },
            Token::Assignment => {
                write!(f, "Assignment")
            },
            Token::Identifier{ values } => {
                write!(f, "{:#?}", values)
            },
            Token::Integer{ value } => {
                write!(f, "{}", value)
            },
            Token::Expression{ left, right } => {
                write!(f, "Expression")
            },
            Token::Program{ expressions } => {
                write!(f, "Program")
            },
            Token::Term{ left, right } => {
                write!(f, "Term")
            },
            Token::Factor{ value } => {
                write!(f, "{}", value)
            },
            _ => {
                write!(f, "unknown")
            }
        }
        
    }
}