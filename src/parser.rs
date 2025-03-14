use ecow::EcoString;

use crate::kind::SyntaxKind;
use crate::lexer::Token;
use crate::span::Span;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parser {
    pub start: usize,
    pub current: usize,
    pub tokens: Vec<Token>,
    nodes: Vec<SyntaxNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxNode {
    kind: SyntaxKind,
    text: EcoString,
    span: Span,
}

impl SyntaxNode {
    pub fn new(kind: SyntaxKind, text: EcoString, span: Span) -> Self {
        Self { kind, text, span }
    }
}

impl std::fmt::Display for SyntaxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?} {}", self.kind, self.text, self.span)
    }
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            current: 0,
            tokens,
            nodes: Vec::new(),
            start: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    // peek at the current charecter without consuming it.
    fn peek(&self) -> Option<Token> {
        if self.is_at_end() {
            None
        } else {
            self.tokens[self.current..].iter().next().cloned()
        }
    }

    // eat current char and return next char, updating current
    fn advance(&mut self) -> Option<Token> {
        if self.is_at_end() {
            None
        } else {
            self.tokens[self.current..]
                .iter()
                .next()
                .map(|c| {
                    self.current += 1;
                    c
                })
                .cloned()
        }
    }

    pub fn parse(&mut self) -> Vec<SyntaxNode> {
        loop {
            let token = self.scan();
            self.nodes.push(token.clone());
            if token.kind == SyntaxKind::Eof {
                break;
            }
        }
        self.nodes.clone()
    }

    pub fn scan(&mut self) -> SyntaxNode {
        self.start = self.current;

        match self.advance() {
            Some(i) => {
                match i.kind {
                    SyntaxKind::Underscore => {
                        SyntaxNode::new(SyntaxKind::Underscore, "this".into(), Span::default())
                    }
                    SyntaxKind::Text => {
                        SyntaxNode::new(SyntaxKind::Text, "this".into(), Span::default())
                    }
                    _ => SyntaxNode::new(SyntaxKind::Text, "this".into(), Span::default()),
                    //'\n' => self.make_token(SyntaxKind::NewLine),
                    //' ' | '\t' => {
                    //    let prev_token = self
                    //        .tokens
                    //        .last()
                    //        .map(|t| t.kind == SyntaxKind::NewLine)
                    //        .unwrap_or(false);
                    //    // Consume contiguous spaces and tabs.
                    //    while let Some(ch) = self.peek() {
                    //        if ch == ' ' || ch == '\t' {
                    //            self.advance();
                    //        } else {
                    //            break;
                    //        }
                    //    }
                    //
                    //    if prev_token || self.tokens.is_empty() {
                    //        self.make_token(SyntaxKind::IndentWhiteSpace)
                    //    } else {
                    //        self.make_token(SyntaxKind::WhiteSpace)
                    //    }
                    //}
                    //_ => {
                    //    while let Some(char) = self.peek() {
                    //        if char.is_special_char() {
                    //            break;
                    //        }
                    //        self.advance();
                    //    }
                    //    self.make_token(SyntaxKind::Text)
                    //}
                }
            }
            _ => SyntaxNode { kind: SyntaxKind::Eof, text: "".into(), span: Span::default() },
        }
    }
}
