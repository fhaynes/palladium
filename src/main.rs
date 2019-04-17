#![allow(unused)]
#[macro_use]
extern crate nom;
extern crate iridium;

mod tokens;
mod operator_parsers;
mod expression_parsers;
mod program_parsers;
mod visitor;
mod factor_parsers;
mod term_parsers;
mod function_parsers;
mod loop_parsers;
mod conditional_parsers;
mod scope;
mod list_parsers;
fn main() {
    
}
