#![allow(unused)]

use neorg_syntax::*;

use self::lexer::Token;
use self::parser::{Parser, Repr};

fn main() {
    let input = " / italic/ \n  /italc _ under lined text_ \n next line \n* ** _this o _ _ ~ this is in tilda ~ - this is in hypens - -";
    // let lexed = lexer::Lexer::new(input.into()).lex();
    let lexed = lexer::Lexer::new(input.into()).lex();
    let mut binding = parser::Parser::new(lexed.clone());
    binding.parse().iter().for_each(|i| {
        if i.kind() == "ErrorNode" {
            println!("{}", i.errors().unwrap());
        }
    })
    // print_parsed(binding);
    // println!("\n\n\n");
    // print_lexed(lexed);
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
