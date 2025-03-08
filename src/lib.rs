use self::lexer::Token;

pub mod ast;
pub mod parser;
pub mod lexer;
pub mod node;
pub mod span;
pub mod kind;

pub(crate) mod error;


pub trait NeoChar {
    /// returns true if the char matches with any syntax char of neorg
    fn is_special_char(&self) -> bool;
}

impl NeoChar for char {
    fn is_special_char(&self) -> bool {
        matches!(self, '*' | '/' | '_' | '\n' | '\t')
    }
}

pub trait ParseTools {
    fn peek(&self) -> &Token;
    fn advance(&mut self) -> Token;
    fn previous(&mut self) -> Token;
    fn is_at_end(&self) -> bool;
}
