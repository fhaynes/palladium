//! Contains the parser for an entire program. This is the
//! topmost level parser
use nom::types::CompleteStr;

use expression_parsers::*;
use tokens::Token;

/// Parses an entire program, which is just a collection of expressions
named!(pub program<CompleteStr, Token>,
    ws!(
        do_parse!(
            expressions: many1!(expression) >>
            (
                Token::Program {
                    expressions: expressions
                }
            )
        )
    )
);

mod tests {
    use super::*;
    #[test]
    fn test_parse_program() {
        let test_program = CompleteStr("1+2");
        let result = program(test_program);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_parse_nested_program() {
        let test_program = CompleteStr("(1+2)*3");
        let result = program(test_program);
        assert_eq!(result.is_ok(), true);
    }
}
