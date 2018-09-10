use nom::types::CompleteStr;

use tokens::Token;
use operand_parsers::operand;
use operator_parsers::operator;

named!(pub expression<CompleteStr, Token>,
    ws!(
        do_parse!(
            left: operand >>
            op: operator >>
            right: operand >>
            (
                Token::Expression{
                    left: Box::new(left),
                    right: Box::new(right),
                    op: Box::new(op)
                }
            )
        )
    )
);
