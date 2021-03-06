//! Contains parsers related to `Terms`
use nom::types::CompleteStr;

use tokens::Token;
use factor_parsers::factor;
use operator_parsers::operator;

/// Looks for `Terms`. A `Term` consists of a `Factor` on the left,
/// and then an `Operator` and `Factor` on the right.
/// 
/// # Example
/// 
/// ```
/// (3*4)*2
/// ```
/// 
named!(pub term<CompleteStr,Token>,
    do_parse!(
        left: factor >>
        right: many0!(
            tuple!(
                alt!(
                    operator
                ),
                factor
            )
        ) >>
        (
            {
                Token::Term{left: Box::new(left), right: right}
            }
        )
    )
);

mod tests {
    use super::*;

    #[test]
    fn test_parse_term() {
        let result = term(CompleteStr("3*4"));
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_parse_nested_term() {
        let result = term(CompleteStr("(3*4)*2"));
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_parse_really_nested_term() {
        let result = term(CompleteStr("((3*4)*2)"));
        assert_eq!(result.is_ok(), true);
    }
}
