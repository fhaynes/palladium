//! This contains functions related to parsing lists. In Palladium, a list
//! looks like this:
//! ```python
//! x = [1, 2, 3]
//! ```
//! 
//! Lists can contain any number and mix of valid identifiers and literals
//! 

use nom::*;
use nom::types::CompleteStr;

use tokens::Token;
use expression_parsers::expression;
use factor_parsers::{identifiers, identifier, factor, integer, float64};

/// Extracts everything between `[` and `]`
named!(list_interior<CompleteStr, Vec<String>>, 
    ws!(
        do_parse!(
            ws!(tag!("[")) >>
            contents: take_until_and_consume!("]") >>
            (
                {
                    let mut result: Vec<String> = vec![];
                    let cloned = contents.to_string();
                    let split = cloned.split(",");
                    for s in split {
                        let s = s.trim();
                        result.push(s.to_string());
                    }
                    result
                }
            )
        )
    )
);

/// Top level parser for a list
named!(pub list<CompleteStr, Token>,
    ws!(
        do_parse!(
            interior: list_interior >>
            (
                {
                    let mut elements = vec![];
                    for i in &interior {
                        let result = expression(CompleteStr(i));
                        let result = result.unwrap().1;
                        elements.push(result);
                    }
                    Token::List{ elements: elements }
                }
            )
        )
    )
);

mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_list() {
        let result = list(CompleteStr("[]"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_list_with_literals_and_identifiers() {
        let result = list(CompleteStr("[1, a]"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_list_with_expression() {
        let result = list(CompleteStr("[1, a, 1+2]"));
        println!("Result is: {:#?}", result);
        assert!(result.is_ok());
    }
}