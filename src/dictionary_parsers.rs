//! This contains functions related to parsing dictionaries. In Palladium, a dictionary
//! looks like this:
//! ```python
//! x = {key: value}
//! ```
//! 
//! Dictionaries may use any type as a key and value
//! 

use nom::*;
use nom::types::CompleteStr;

use tokens::Token;
use expression_parsers::expression;
use factor_parsers::{identifiers, identifier, factor, integer, float64};

/// Extracts everything between `{` and `}`
named!(dictionary_interior<CompleteStr, CompleteStr>, 
    ws!(
        do_parse!(
            ws!(tag!("{")) >>
            contents: take_until_and_consume!("}") >>
            (
                {
                    contents
                }
            )
        )
    )
);

named!(key_value_pair<CompleteStr, Token>,
    ws!(
        do_parse!(
            key: expression >>
            ws!(tag!(":")) >>
            value: expression >>
            (
                {
                    Token::DictionaryKeyValuePair{ key: Box::new(key), value: Box::new(value) }
                }
            )
        )
    )
);
/// Top level parser for a dictionary
named!(pub dictionary<CompleteStr, Token>,
    ws!(
        do_parse!(
            dictionary_interior >>
            (
                {
                    Token::Dictionary{keys: vec![], values: vec![] }
                }
            )
        )
    )
);

mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_dictionary() {
        let result = dictionary(CompleteStr("{}"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_interior() {
        let result = dictionary_interior(CompleteStr("{\"test\": 1}"));
        println!("{:#?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_key_value_pair() {
        let result = key_value_pair(CompleteStr("10: 1"));
        println!("{:#?}", result);
        assert!(result.is_ok());
    }
}