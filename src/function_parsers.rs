//! This contains functions related to parsing functions. In Palladium, a function
//! looks like this:
//! def somefunction(arg1, arg2):
//!     expressions
//! \n

use nom::*;
use nom::types::CompleteStr;

use tokens::Token;
use expression_parsers::expression;

/// Function to extract a function name. A function name is comprised of:
/// `def` `a-zA-Z0-9`
named!(pub function_name<CompleteStr, Token>,
    ws!(
        do_parse!(
            ws!(tag!("def")) >>
            many0!(tag!(" ")) >>
            func_name: take_until!("(") >>
            ( 
                {
                    Token::FunctionName{ name: func_name.to_string() }
                }
            )
        )
    )
);

/// Function to look for an individual arg. So in `def func(x, y, z)` it is meant to look for x, y and z
named!(function_arg<CompleteStr, String>,
    ws!(
        do_parse!(
            // Any alphanumeric counts as a valid character in a function
            arg: ws!(alphanumeric) >>
            // If there is more than one arg, we optionally consume the separating `,`
            opt!(ws!(tag!(","))) >>
            (
                {
                    arg.to_string()
                }
            )
        )
    )
);

/// Extracts all of the arguments from a function definition
/// This calls `function_arg` repeatedly to get all the args and put them in a list
named!(pub function_args<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("(") >>
            args: many0!(function_arg) >>
            tag!(")") >>
            (
                {
                    Token::FunctionArgs{ args: args }
                }
            )
        )
    )
);

/// Extracts all the expressions that make up a function body
named!(pub function_body<CompleteStr, Token>,
    ws!(
        do_parse!(
            // This signals the beginning of the body
            tag!(":") >>
            expressions: many0!(expression) >>
            (
                Token::FunctionBody{ expressions: expressions }
            )
        )
    )
);

/// Top level function that uses all of the previously defined functions
/// to parse out a complete function_parsers
named!(pub function<CompleteStr, Token>,
    ws!(
        do_parse!(
            fname: function_name >>
            args: function_args >>
            body: function_body >>
            (
                Token::Function{ name: Box::new(fname), args: Box::new(args), body: Box::new(body) }
            )
        )
    )
);

mod tests {
    use super::*;

    #[test]
    fn test_parse_function_header() {
        let result = function_name(CompleteStr("def testfunc("));
        assert!(result.is_ok());
        
    }

    #[test]
    fn test_parse_function_arg() {
        let result = function_arg(CompleteStr("arg1)"));
        assert!(result.is_ok());
        let result = function_arg(CompleteStr("arg1,"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_function_args() {
        let result = function_args(CompleteStr("(arg1, arg2)"));
    }

    #[test]
    fn test_parse_function() {
        let test_function = CompleteStr(
r#"
def test(arg1, arg2):
    3+1

"#
);
        let result = function(test_function);
        assert!(result.is_ok());
    }
}
