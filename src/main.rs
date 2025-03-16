#![allow(unused)]

use self::kind::SyntaxKind;
use self::lexer::Token;
use self::parser::Bold;
use self::parser::{Parser, Repr};
use neorg_syntax::*;

fn main() {
    // let input = " /italic/ \n  /italc _under lined text_ \n next line \n* ** _this o_ _ ~this is in tilda~ -this is in hypens- -";
    let input = include_str!("../examples/tests/italics.norg");
    let lexed = lexer::Lexer::new(input.into()).lex();
    let mut binding = parser::Parser::new(lexed.clone());
    crate::ast::print_ast(input);
}

fn print_lexed(l: Vec<Token>) {
    let parsed = l;

    for i in parsed {
        println!("{}", i)
    }
}
fn print_parsed(mut p: Parser) {
    let parsed = p.parse();

    for i in parsed {
        println!("{}", i)
    }
}
