//! Contains parsers for `Factors`
use nom::*;
use nom::types::CompleteStr;

use tokens::Token;
use expression_parsers::expression;
use function_parsers::function_call;

/// Parser for a 64-bit float. A float can be negative, and must contain a `.`.
/// 
/// # Example
/// 
/// ```
/// x = 4.5
/// y = -4.5
/// ```
named!(float64<CompleteStr, Token>,
    ws!(
        do_parse!(
            sign: opt!(tag!("-")) >>
            left_nums: digit >>
            tag!(".") >>
            right_nums: digit >>
            (
                {
                    let mut tmp = String::from("");
                    if sign.is_some() {
                        tmp.push_str("-");
                    }
                    tmp.push_str(&left_nums.to_string());
                    tmp.push_str(".");
                    tmp.push_str(&right_nums.to_string());
                    let converted = tmp.parse::<f64>().unwrap();
                    Token::Factor{ value: Box::new(Token::Float{value: converted}) }
                }
            )
        )
    )
);

/// Parser for a signed 64-bit integer.
/// 
/// # Example
/// 
/// ```
/// x = 4
/// y = -4
/// ```
named!(integer<CompleteStr, Token>,
    ws!(
        do_parse!(
            sign: opt!(tag!("-")) >>
            reg_num: digit >>
            (
                {
                    let mut tmp = String::from("");
                    if sign.is_some() {
                        tmp.push_str("-");
                    }
                    tmp.push_str(&reg_num.to_string());
                    let converted = tmp.parse::<i64>().unwrap();
                    Token::Integer{ value: converted }
                }
            )
        )
    )
);

/// Parse for a variable identifier
/// 
/// # Example
/// 
/// ```
/// x
/// ````
/// 
/// An Identifier can consist only of letters and are case-sensitive.
named!(pub identifier<CompleteStr, Token>,
    ws!(
        do_parse!(
            not!(reserved) >>
            id: alpha >>
            (
                {
                    Token::Factor{ value: Box::new(Token::Identifier{ value: id.to_string() })}
                }
            )
        )
    )
);

named!(pub reserved<CompleteStr, CompleteStr>,
    ws!(
        peek!(
            alt!(
                complete!(tag!("def")) |
                complete!(tag!("if")) |
                complete!(tag!("elif")) |
                complete!(tag!("else")) |
                complete!(tag!("return"))
            )
        )
    )
);
/// Parser for a `Factor`. A Factor consists of an integer, float, identifier,
/// or a parenthized expression
/// 
/// # Example
/// 
/// ```
/// (1+2)
/// ```
/// 
named!(pub factor<CompleteStr, Token>,
    ws!(
        do_parse!(
            f: alt!(
                integer |
                float64 |
                function_call |
                identifier |
                ws!(delimited!( tag_s!("("), expression, tag_s!(")") ))
            ) >>
            (
                {
                    Token::Factor{value: Box::new(f)}
                }
            )
        )
    )
);

mod tests {
    use super::*;
    #[test]
    fn test_factor() {
        let test_program = CompleteStr("(1+2)");
        let result = factor(test_program);
        assert_eq!(result.is_ok(), true);
        let (_, tree) = result.unwrap();
    }

    #[test]
    fn test_parse_floats() {
        let test_floats = vec!["100.4", "1.02", "-1.02"];
        for o in test_floats {
            let parsed_o = o.parse::<f64>().unwrap();
            let result = float64(CompleteStr(o));
            assert_eq!(result.is_ok(), true);
        }
    }

    #[test]
    fn test_parse_integer() {
        let test_integers = vec!["0", "-1", "1"];
        for o in test_integers {
            let parsed_o = o.parse::<i64>().unwrap();
            let result = integer(CompleteStr(o));
            assert_eq!(result.is_ok(), true);
        }
    }

    #[test]
    fn test_parse_identifier() {
        let result = identifier(CompleteStr("x"));
        assert!(result.is_ok());
    }
}
