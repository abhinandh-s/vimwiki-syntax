#![allow(dead_code)]

use std::fmt::Display;
use std::num::NonZeroI8;

use crate::ParseTools;
use crate::kind::SyntaxKind;
use crate::lexer::{Lexer, Token};
use crate::span::Span;
use ecow::EcoString;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Delimiter {
    Opened,
    Closed,
}

impl Parser {
    pub fn new(source: impl Into<String>) -> Self {
        let mut lexer = Lexer::new(source.into().into());
        let tokens = lexer.lex();
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<SyntaxNode> {
        let mut nodes = Vec::new();
        while !self.is_at_end() {
            match self.peek().kind {
                SyntaxKind::Astrisk => {
                    // not always the case, make it first non white space
                    // char == `*` then heading
                    nodes.push(self.heading());
                }
                SyntaxKind::Slash => nodes.push(self.parse_italics()),
                SyntaxKind::Underscore => nodes.push(self.parse_underline()),
                _ => (), /* {
                             nodes.push(Node {
                                 text: "dsd".into(),
                                 kind: SyntaxKind::Text,
                                 span: Span::default(),
                                 attr: None,
                                 children: None,
                             });
                         } */
            }
            self.current += 1;
        }
        nodes
    }

    // get slice of vec from current token and returns true it found the SyntaxKind in any Token
    // -- TEST: this
    fn search_kind(&self, kind: SyntaxKind) -> bool {
        self.tokens[self.current..].iter().any(|f| f.kind == kind)
    }

    // -- TEST: this
    fn search_kind_by_span(&self, idx: usize) -> Option<&Token> {
        self.tokens[..]
            .iter()
            .find(|d| d.span.start <= idx && d.span.end >= idx)
    }

    fn parse_underline(&mut self) -> SyntaxNode {
        let mut delimiter = Delimiter::Closed;
        let mut string = EcoString::new();
        let start = self.peek().span.start;
        let mut end = self.peek().span.end;
        let _cond = matches!(
            self.peek().kind,
            SyntaxKind::ListItem | SyntaxKind::At | SyntaxKind::NewLine | SyntaxKind::Tilda
        );

        let s = |kind: SyntaxKind| self.tokens[self.current..].iter().any(|f| f.kind == kind);
        if s(SyntaxKind::Underscore) {
            println!("got an Underscore");
        }

        while self.peek().kind == SyntaxKind::Text
            || self.peek().kind == SyntaxKind::WhiteSpace
            || self.peek().kind == SyntaxKind::Underscore
        /* && self.peek().kind != SyntaxKind::Slash */
        {
            match self.peek().kind {
                SyntaxKind::WhiteSpace => {
                    if delimiter == Delimiter::Opened {
                        string.push_str(&self.peek().text);
                    }
                    end = self.peek().span.end
                }
                SyntaxKind::Text => {
                    string.push_str(&self.peek().text);
                    end = self.peek().span.end
                }
                SyntaxKind::Underscore => {
                    //  string.push_str(&self.peek().text);
                    if delimiter == Delimiter::Opened {
                        delimiter = Delimiter::Closed
                    } else {
                        delimiter = Delimiter::Opened
                    }
                    end = self.peek().span.end
                }

                _ => {
                    string.push_str(&self.peek().text);
                    // println!("Error: unknown kind `{}`", self.peek().kind);
                    return SyntaxNode {
                        text: string,
                        span: Span { start, end },
                        attr: None,
                        children: None,
                        kind: SyntaxKind::Error,
                    };
                }
            }
            self.current += 1;
        }
        println!("Error: unknown kind `{}`", string);
        match delimiter {
            Delimiter::Closed => SyntaxNode {
                text: string,
                span: Span { start, end },
                attr: None,
                children: None,
                kind: SyntaxKind::UnderLined,
            },
            Delimiter::Opened => SyntaxNode {
                text: string,
                span: Span { start, end },
                attr: None,
                children: None,
                kind: SyntaxKind::Error,
            },
        }
    }

    fn parse_italics(&mut self) -> SyntaxNode {
        let mut delimiter = Delimiter::Closed;
        let mut string = EcoString::new();
        let start = self.peek().span.start;
        let mut end = self.peek().span.end;
        let _cond = matches!(
            self.peek().kind,
            SyntaxKind::ListItem | SyntaxKind::At | SyntaxKind::NewLine | SyntaxKind::Tilda
        );

        while self.peek().kind == SyntaxKind::Text
            || self.peek().kind == SyntaxKind::WhiteSpace
            || self.peek().kind == SyntaxKind::Slash
        /* && self.peek().kind != SyntaxKind::Slash */
        {
            match self.peek().kind {
                SyntaxKind::WhiteSpace => {
                    if delimiter == Delimiter::Opened {
                        string.push_str(&self.peek().text);
                    }
                    end = self.peek().span.end
                }
                SyntaxKind::Text => {
                    string.push_str(&self.peek().text);
                    end = self.peek().span.end
                }
                SyntaxKind::Slash => {
                    //  string.push_str(&self.peek().text);
                    if delimiter == Delimiter::Opened {
                        delimiter = Delimiter::Closed
                    } else {
                        delimiter = Delimiter::Opened
                    }
                    end = self.peek().span.end
                }

                _ => {
                    string.push_str(&self.peek().text);
                    println!("Error: unknown kind `{}`", self.peek().kind);
                    return SyntaxNode {
                        text: "this is a dummy error".into(),
                        span: Span { start, end },
                        attr: None,
                        children: None,
                        kind: SyntaxKind::Error,
                    };
                }
            }
            self.current += 1;
        }
        match delimiter {
            Delimiter::Closed => SyntaxNode {
                text: string,
                span: Span { start, end },
                attr: None,
                children: None,
                kind: SyntaxKind::Italics,
            },
            Delimiter::Opened => SyntaxNode {
                text: string,
                span: Span { start, end },
                attr: None,
                children: None,
                kind: SyntaxKind::Error,
            },
        }
    }

    fn heading(&mut self) -> SyntaxNode {
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
                    return SyntaxNode {
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
        SyntaxNode {
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
pub struct SyntaxNode {
    pub text: EcoString,
    pub kind: SyntaxKind,
    pub span: Span,
    pub attr: Option<Attr>,
    pub children: Option<Vec<SyntaxNode>>,
}

impl Display for SyntaxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} \"{}\" {}", self.kind, self.text, self.span)
    }
}

impl SyntaxNode {
    pub fn new(
        text: EcoString,
        span: Span,
        attr: Option<Attr>,
        children: Option<Vec<SyntaxNode>>,
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
