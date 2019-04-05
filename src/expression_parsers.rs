//! Contains parsers related to parsing Expressions
use nom::types::CompleteStr;

use tokens::Token;
use term_parsers::term;
use operator_parsers::operator;
use function_parsers::function;
/// Parses a complete expression
/// 
/// # Expressions
/// 
/// An expression consists of a `Term` and an `Operator` and `Term` on the right side.
named!(pub expression<CompleteStr, Token>,
    do_parse!(
        left: alt!(
            function |
            term
        ) >>
        right: many0!(
            tuple!(
                alt!(
                    operator
                ),
                term
            )
        ) >>
        (
            {
                Token::Expression{left: Box::new(left), right: right}
            }
        )
    )
);

mod tests {
    use super::*;

    #[test]
    fn test_parse_expression() {
        let result = expression(CompleteStr("3>4"));
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_parse_nested_expression() {
        let result = expression(CompleteStr("(3*4)+1"));
        assert_eq!(result.is_ok(), true);
    }
}
