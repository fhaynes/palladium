#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    AdditionOperator,
    SubtractionOperator,
    MultiplicationOperator,
    DivisionOperator,
    Integer{ value: i64 },
    Float{ value: f64},
    Factor{ value: Box<Token> },
    Term{ left: Box<Token>, right: Vec<(Token, Token)> },
    Expression{ left: Box<Token>, right: Vec<(Token, Token)> },
    Program{ expressions: Vec<Token> }
}
