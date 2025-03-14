use neorg_syntax::*;

fn main() {
    let input = " /italic/ \n  /italc _under lined text_ \n next line \n* ** _this o _ _";
    // let lexed = lexer::Lexer::new(input.into()).lex();
    let lexed = lexer::Lexer::new(input.into()).lex();
    let parsed = parser::Parser::new(lexed).parse();
    
    // println!("{:?}", parsed);
    for i in parsed {
        println!("{}", i)
    }
}
