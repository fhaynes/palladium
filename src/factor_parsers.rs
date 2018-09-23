use nom::*;
use nom::types::CompleteStr;

use tokens::Token;
use expression_parsers::expression;

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

named!(pub factor<CompleteStr, Token>,
    ws!(
        do_parse!(
            f: alt!(
                integer |
                float64 |
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
        println!("{:#?}", tree);
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
}
