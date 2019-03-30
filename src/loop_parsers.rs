//! Contains parsers for loops
//! 
//! # Loops
//! 
//! In Palladium, there are two types of loops: for, and while. Each is for a different purpose.
//! 
//! ## For Loops
//! 
//! A for loop in Palladium works like a for loop in Python, or a foreach loop in most other languages.
//! It iterates over a collection.
//! 
//! ## While Loops
//! 
//! A while loop is used to evaluate an expression each iteration and continue looping
//! until the expression is no longer True.

use nom::*;
use nom::types::CompleteStr;

use tokens::Token;
use expression_parsers::expression;

/// Parses a for loop start
/// 
/// # Example
/// 
/// ```
/// for tile in tiles:
/// ```
/// 
/// `tile` must be a valid `Identifier`, and `tiles` must be a valid collection.
/// 
named!(pub for_loop_start<CompleteStr, Token>,
    ws!(
        do_parse!(
            ws!(tag!("for")) >>
            variable_name: ws!(alphanumeric) >>
            ws!(tag!("in")) >>
            collection_name: ws!(alphanumeric) >>
            ws!(tag!(":")) >>
            (
                {
                    Token::ForLoopStart{
                        variable_name: variable_name.to_string(),
                        collection_name: collection_name.to_string()
                    }
                }
            )
        ) 
    )
);

/// Parses the body of a for loop. The body contains an arbitrary number of
/// expressions
named!(pub for_loop_body<CompleteStr, Token>,
    ws!(
        do_parse!(
            expressions: many0!(expression) >>
            (
                {
                    Token::ForLoopBody{
                        expressions: expressions
                    }
                }
            )
        ) 
    )
);

/// Higher level parse that looks for both the start of a for loop and its body
/// 
/// # Example
/// 
/// ```
/// for tile in tiles:
///     print(tile)
/// ```
/// 
named!(pub for_loop<CompleteStr, Token>,
    ws!(
        do_parse!(
            start: for_loop_start >>
            body: for_loop_body >>
            (
                {
                    Token::ForLoop{
                        start: Box::new(start),
                        body: Box::new(body)
                    }
                }
            )
        )
    )
);

/// Looks for the beginning of a while loop
/// 
/// # Example
/// 
/// ```
/// while x < 1:
/// ```
/// 
named!(pub while_loop_start<CompleteStr, Token>,
    ws!(
        do_parse!(
            ws!(tag!("while")) >>
            loop_expression: expression >>
            tag!(":") >>
            (
                {
                    Token::WhileLoopStart{ expression: Box::new(loop_expression) }
                }
            )
        )
    )
);

/// Looks for a while loop body, which is an arbitrary number of expressions
named!(pub while_loop_body<CompleteStr, Token>,
    ws!(
        do_parse!(
            // This signals the beginning of the body
            expressions: many0!(expression) >>
            (
                Token::WhileLoopBody{ expressions: expressions }
            )
        )
    )
);

/// Higher level parser that looks for a complete while loop
/// 
/// # Example
/// 
/// ```
/// while x > 1:
///     x = x + 1
/// ```
///
named!(pub while_loop<CompleteStr, Token>,
    ws!(
        do_parse!(
            start: while_loop_start >>
            body: while_loop_body >>
            (
                {
                    Token::WhileLoop{
                        start: Box::new(start),
                        body: Box::new(body)
                    }
                }
            )
        )
    )
);

/// Highest level parser that looks for either a for loop or while loop
named!(pub a_loop<CompleteStr, Token>,
    ws!(
        do_parse!(
            l: alt!(
                while_loop |
                for_loop
            ) >> 
            (
                {
                    l
                }
            )
        )
    )
);

mod tests {
    use super::*;

    #[test]
    fn test_parse_for_loop_start() {
        let result = for_loop_start(CompleteStr("for file in files:"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_while_loop_start() {
        let result = while_loop_start(CompleteStr("while x < 1:"));
        assert!(result.is_ok());
    }
}
