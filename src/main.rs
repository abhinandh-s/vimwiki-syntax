use neorg_syntax::*;

fn main() {
    let input = " /italic/ \n /italc ";
    //let lexed = lexer::Lexer::new(input.into()).lex();
    //for l in lexed {
    //    println!("{}", l);
    //}
    let parsed = parser::Parser::new(input).parse();
    // println!("{:?}", parsed);
    for i in parsed {
        println!("{}", i)
    }
}
