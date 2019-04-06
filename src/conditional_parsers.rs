//! Contains parsers for conditionals
//! 
//! ## Conditionals
//! 
//! These are statements that allow for choosing a code path based on some criteria.
//! If, elif, else, case, switch and similar are all examples of conditionals.
//! 
//! ### If/Elif/Else
//! 
//! In Palladium, an if statement looks like:
//! 
//! ```
//! if x > 5:
//!     <expressions>
//! elif x > 3:
//!     <expressions>
//! else:
//!     <expressions>
//!```
//! 
//! There may be any number of elif statements. The first one with an expression that 
//! evaluates to true will be chosen.
//! 
//! An optional else statement that follows an if statement or the elif statements can serve 
//! as a catch-all.
//! 
use nom::*;
use nom::types::CompleteStr;

use tokens::Token;
use expression_parsers::expression;

/// Parses the start of an if block
/// 
/// # Example
/// 
/// ```
/// if x > 5:
/// ```
/// 
named!(pub if_block_start<CompleteStr, Token>,
    ws!(
        do_parse!(
            ws!(tag!("if")) >>
            expr: expression >>
            ws!(tag!(":")) >>
            body: many0!(expression) >>
            (
                {
                    Token::If {
                        expr: Box::new(expr),
                        body: body
                    }
                }
            )
        ) 
    )
);

/// Parses an elif block
/// 
/// # Example
/// 
/// ```
/// elif x > 3:
///     <expressions>
/// ```
/// 
named!(pub elif_block<CompleteStr, Token>,
    ws!(
        do_parse!(
            ws!(tag!("elif")) >>
            expr: expression >>
            ws!(tag!(":")) >>
            body: many0!(expression) >>
            (
                {
                    Token::Elif {
                        expr: Box::new(expr),
                        body: body
                    }
                }
            )
        )
    )
);

named!(pub else_block<CompleteStr, Token>,
    ws!(
        do_parse!(
            ws!(tag!("else:")) >>
            body: many0!(expression) >>
            (
                {
                    Token::Else {
                        body: body
                    }
                }
            )
        )
    )
);

mod tests {
    use super::*;

    #[test]
    fn test_parse_if_start() {
        let result = if_block_start(CompleteStr("if x > 3:\n1+2"));
        assert!(result.is_ok());
        let result = if_block_start(CompleteStr("if x > 3:\n3+2"));
        assert!(result.is_ok());
        let result = if_block_start(CompleteStr("if x > 3:\n3+2\n2+1"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_elif_start() {
        let result = elif_block(CompleteStr("elif x > 3:"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_else_start() {
        let result = else_block(CompleteStr("else:"));
        assert!(result.is_ok());
    }
}
