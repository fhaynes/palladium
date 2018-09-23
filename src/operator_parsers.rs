use nom::types::CompleteStr;

use tokens::Token;

/// Parses the "+" operator
named!(pub addition_operator<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("+") >>
            (
                Token::AdditionOperator
            )
        )
    )
);

/// Parses the "-" operator
named!(pub subtraction_operator<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("-") >>
            (
                Token::SubtractionOperator
            )
        )
    )
);

/// Parses the "*" operator
named!(pub multiplication_operator<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("*") >>
            (
                Token::MultiplicationOperator
            )
        )
    )
);

/// Parses the "/" operator
named!(pub division_operator<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("/") >>
            (
                Token::DivisionOperator
            )
        )
    )
);

/// Looks for any of the operands
named!(pub operator<CompleteStr, Token>,
    ws!(
        alt!(
            addition_operator |
            subtraction_operator |
            multiplication_operator |
            division_operator
        )
    )
);

mod tests {
    use super::*;
    use tokens::Token;
    use nom::types::CompleteStr;

    #[test]
    fn test_parse_addition_operator() {
        let result = addition_operator(CompleteStr("+"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::AdditionOperator);
    }

    #[test]
    fn test_parse_subtraction_operator() {
        let result = subtraction_operator(CompleteStr("-"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::SubtractionOperator);
    }

    #[test]
    fn test_parse_multiplication_operator() {
        let result = multiplication_operator(CompleteStr("*"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::MultiplicationOperator);
    }

    #[test]
    fn test_parse_division_operator() {
        let result = division_operator(CompleteStr("/"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::DivisionOperator);
    }

    #[test]
    fn test_parse_operator() {
        let operators = vec!["+", "*", "-", "/"];
        for o in operators {
            let result = operator(CompleteStr(o));
            assert_eq!(result.is_ok(), true);
        }
    }
}
