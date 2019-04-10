//! This contains functions related to parsing functions. In Palladium, a function
//! looks like this:
//! def somefunction(arg1, arg2):
//!     expressions
//! \n

use nom::*;
use nom::types::CompleteStr;

use tokens::Token;
use expression_parsers::expression;
use factor_parsers::{identifiers, identifier, factor};

/// Function to extract a function name. A function name is comprised of:
/// `def` `a-zA-Z0-9`
named!(pub function_name<CompleteStr, Token>,
    ws!(
        do_parse!(
            ws!(tag!("def")) >>
            func_name: take_until!("(") >>
            ( 
                {
                    Token::FunctionName{ name: func_name.to_string() }
                }
            )
        )
    )
);

/// Function to look for an individual arg. 
/// In `def func(x, y, z):` it is meant to look for x, y and z
named!(function_arg<CompleteStr, String>,
    ws!(
        do_parse!(
            // Any alphanumeric counts as a valid character in a function
            arg: ws!(identifier) >>
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
            expressions: many0!(expression) >>
            (
                {
                    Token::FunctionBody{ expressions: expressions }
                }
            )
        )
    )
);

/// Top level function that uses all of the previously defined functions
/// to parse out a complete function
named!(pub function<CompleteStr, Token>,
    ws!(
        do_parse!(
            fname: function_name >>
            args: function_args >>
            ws!(tag!(":")) >>
            body: function_body >>
            rs: return_statement >>
            (
                {
                    Token::Function{ name: Box::new(fname), args: Box::new(args), body: Box::new(body), return_statement: Box::new(rs) }
                }
            )
        )
    )
);

named!(pub function_call<CompleteStr, Token>,
    ws!(
        do_parse!(
            name: identifier >>
            parameters: function_args >>
            (
                {
                    Token::FunctionCall{
                        name: name,
                        parameters: Box::new(parameters)
                    }
                }
            )
        )
    )
);

named!(pub return_arg<CompleteStr, Token>,
    ws!(
        do_parse!(
            f: factor >>
            opt!(ws!(tag!(","))) >>
            (
                {
                    Token::Factor{ value: Box::new(f) }
                }
            )
        )
    )
);

named!(pub return_args<CompleteStr, Token>,
    ws!(
        do_parse!(
            args: many0!(return_arg) >>
            ws!(tag!(";")) >>
            (
                {
                    Token::ReturnArgs{ args: args }
                }
            )
        )
    )
);

named!(pub return_statement<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("return") >>
            arg: return_arg >>
            ws!(tag!(";")) >>
            
            (
                {
                    Token::ReturnStatement{ parameters: Box::new(arg) }
                }
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
    fn test_parse_function_def() {
        let test_function = CompleteStr("def test(arg1, arg2):\n\t1+2\n\treturn 0;\n");
        let result = function(test_function);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_function_call() {
        let test_function = CompleteStr(
r#"
test(arg1, arg2)
"#
);
        let result = function_call(test_function);
        assert!(result.is_ok());
    }
}
