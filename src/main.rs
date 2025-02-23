use neorg_syntax::kind::SyntaxKind;
use neorg_syntax::parser::Parser;

fn main() {
    let input = "* Main Heading\nSome _italic_ text\n- List item\n@ metadata** Main / Heading\n";
    // let lexed = Lexer::new(input.into()).lex();
    // for l in lexed {
    //     println!("{}", l);
    // }
    let parsed = Parser::new(input).parse();
    println!("{:#?}", parsed);
    for i in parsed {
        if i.kind == SyntaxKind::Heading {
            println!("i is heading")
        }
    }
}

