use neorg_syntax::lexer::Lexer;
use ropey::Rope;

fn main() {
 
    let input =
        "* Header\n *bold *\n/italic text/\n_underline_\n~ o list\n- u list\n @code/italic \n text/";
    // Lexing: preserve all whitespace.
    let mut lexer = Lexer::new(Rope::from_str(input));
    let tokens = lexer.lex();
    for token in &tokens {
        println!("{}", token);
    }

    //let mut parser = Parser::new(tokens);
    //parser.parse().children.into_iter().for_each(|i| {
    //    println!("{:#?}", i);
    //});
    // println!("{:#?}", parser.parse());
}
