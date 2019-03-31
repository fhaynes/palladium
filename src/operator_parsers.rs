//! Parsers related to `Operators`
//! 
use nom::types::CompleteStr;

use tokens::Token;

/// Looks for any of the operators and returns a Token
/// for the appropriate one
named!(pub operator<CompleteStr, Token>,
    ws!(
        do_parse!(
            token: alt!(
                tag!("not") |
                tag!("or") |
                tag!("and") |
                tag!("<=") |
                tag!(">=") |
                tag!("==") |
                tag!("+") |
                tag!("-") |
                tag!("*") |
                tag!("/") |
                tag!(">") |
                tag!("<") |
                tag!("=")
        ) >>
        (
            {
                match token {
                    CompleteStr("or") => Token::LogicalOr,
                    CompleteStr("and") => Token::LogicalAnd,
                    CompleteStr("+") => Token::AdditionOperator,
                    CompleteStr("-") => Token::SubtractionOperator,
                    CompleteStr("*") => Token::MultiplicationOperator,
                    CompleteStr("/") => Token::DivisionOperator,
                    CompleteStr(">") => Token::GreaterThan,
                    CompleteStr("<") => Token::LessThan,
                    CompleteStr(">=") => Token::GreaterThanOrEqual,
                    CompleteStr("<=") => Token::LessThanOrEqual,
                    CompleteStr("==") => Token::EqualTo,
                    CompleteStr("=") => Token::Assignment,
                    CompleteStr(&_) => { unreachable!() },
                }
            }
        )
        )

    )
);

mod tests {
    use super::*;
    use tokens::Token;
    use nom::types::CompleteStr;

    #[test]
    fn test_parse_addition_operator() {
        let result = operator(CompleteStr("+"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::AdditionOperator);
    }

    #[test]
    fn test_parse_subtraction_operator() {
        let result = operator(CompleteStr("-"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::SubtractionOperator);
    }

    #[test]
    fn test_parse_multiplication_operator() {
        let result = operator(CompleteStr("*"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::MultiplicationOperator);
    }

    #[test]
    fn test_parse_division_operator() {
        let result = operator(CompleteStr("/"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::DivisionOperator);
    }

    #[test]
    fn test_parse_operator() {
        let operators = vec!["+", "*", "-", "/", ">", "<", "<=", ">="];
        for o in operators {
            let result = operator(CompleteStr(o));
            assert_eq!(result.is_ok(), true);
        }
    }
}
