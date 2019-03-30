//! Contains parsers related to loops

use nom::*;
use nom::types::CompleteStr;

use tokens::Token;
use expression_parsers::expression;

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
