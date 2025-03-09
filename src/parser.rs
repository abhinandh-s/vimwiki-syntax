#![allow(dead_code)]

use crate::ParseTools;
use crate::kind::SyntaxKind;
use crate::lexer::{Lexer, Token};
use crate::span::Span;
use ecow::EcoString;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(source: impl Into<String>) -> Self {
        let mut lexer = Lexer::new(source.into().into());
        let tokens = lexer.lex();
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        while !self.is_at_end() {
            if self.peek().kind == SyntaxKind::Astrisk {
                if self.is_at_line_start() {
                    // not always the case, make it first non white space
                    // char == `*` then heading
                    nodes.push(self.heading());
                } else {
                    nodes.push(self.bold());
                }
            }
            self.current += 1;
        }
        nodes
    }

    fn heading(&mut self) -> Node {
        let mut level = 0;
        let mut string = EcoString::new();
        let start = self.peek().span.start;
        let mut end = self.peek().span.end;
        while !self.is_at_end() && self.peek().kind != SyntaxKind::NewLine {
            match self.peek().kind {
                SyntaxKind::WhiteSpace => {
                    string.push_str(&self.peek().text);
                    end = self.peek().span.end
                }
                SyntaxKind::Text => {
                    string.push_str(&self.peek().text);
                    end = self.peek().span.end
                }
                SyntaxKind::Astrisk => {
                    string.push_str(&self.peek().text);
                    level += 1;
                    end = self.peek().span.end
                }
                _ => {
                    string.push_str(&self.peek().text);
                    println!("Error: unknown kind `{}`", self.peek().kind);
                    return Node {
                        text: string,
                        span: Span { start, end },
                        attr: Some(Attr::HeadingAttr(level)),
                        children: None,
                        kind: SyntaxKind::Error,
                    };
                }
            }
            self.current += 1;
        }
        Node {
            text: string,
            span: Span { start, end },
            attr: Some(Attr::HeadingAttr(level)),
            children: None,
            kind: SyntaxKind::Heading,
        }
    }

    fn is_at_line_start(&self) -> bool {
        if self.current == 0 {
            return true;
        }

        // Look at previous token
        let prev_idx = self.current - 1;
        matches!(self.tokens[prev_idx].kind, SyntaxKind::NewLine)
    }

    fn eat_until(&mut self) {}

    fn bold(&mut self) -> Node {
        let mut string = EcoString::new();
        let start = self.peek().span.start;
        let mut end = self.peek().span.end;

        Node {
            text: string,
            kind: SyntaxKind::Bold,
            span: Span::new(start, end),
            attr: None,
            children: None,
        }
    }
}

impl ParseTools for Parser {
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.peek().kind == SyntaxKind::Eof
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        //  self.tokens[self.current - 1].clone()
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        if !self.is_at_end() {
            self.current -= 1;
        }
        //  self.tokens[self.current - 1].clone()
        self.tokens[self.current].clone()
    }
}

#[derive(Debug)]
pub struct Node {
    pub text: EcoString,
    pub kind: SyntaxKind,
    pub span: Span,
    pub attr: Option<Attr>,
    pub children: Option<Vec<Node>>,
}

impl Node {
    pub fn new(
        text: EcoString,
        span: Span,
        attr: Option<Attr>,
        children: Option<Vec<Node>>,
        kind: SyntaxKind,
    ) -> Self {
        Self {
            text,
            span,
            attr,
            children,
            kind,
        }
    }
}

#[derive(Debug)]
pub enum Attr {
    HeadingAttr(u8),
}

#[cfg(test)]
mod test {
    use crate::kind::SyntaxKind;
    use crate::parser::Parser;
    use crate::span::Span;

    #[test]
    fn test_heading_001() {
        let input =
            "* Main Heading\nSome _italic_ text\n- List item\n@ metadata** Main / Heading\n";
        let mut vec = Vec::new();
        let parsed = Parser::new(input).parse();
        println!("{:#?}", parsed);
        for i in parsed {
            if i.kind == SyntaxKind::Heading {
                println!("i is heading");
                vec.push(i);
            }
        }

        let one = vec[0].span;
        assert_eq!(one, Span::new(0, 14));
        assert_eq!(vec[0].text, input[0..14])
    }

    #[test]
    fn test_bold_text() {
        let input = "This is *bold* text";
        let parsed = Parser::new(input).parse();

        assert_eq!(parsed.len(), 1); // One paragraph node

        if let Some(children) = &parsed[0].children {
            assert_eq!(children.len(), 3); // "This is ", bold node, " text"

            // Check the bold node
            assert_eq!(children[1].kind, SyntaxKind::Bold);
            assert_eq!(children[1].text, "bold");
        } else {
            panic!("Expected paragraph to have children");
        }
    }

    #[test]
    fn test_heading_vs_bold() {
        let input = "* Heading\nThis is *bold* text";
        let parsed = Parser::new(input).parse();

        assert_eq!(parsed.len(), 2); // Heading and paragraph
        assert_eq!(parsed[0].kind, SyntaxKind::Heading);
        assert_eq!(parsed[1].kind, SyntaxKind::Paragraph);

        if let Some(children) = &parsed[1].children {
            assert_eq!(children[1].kind, SyntaxKind::Bold);
            assert_eq!(children[1].text, "bold");
        }
    }
}
