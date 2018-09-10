#[derive(Debug, PartialEq)]
pub enum Token {
    AdditionOperator,
    SubtractionOperator,
    MultiplicationOperator,
    DivisionOperator,
    Integer{ value: i64 },
    Expression{ left: Box<Token>, op: Box<Token>, right: Box<Token> },
    Program{ expressions: Vec<Token> }
}
