use std::fmt::Display;

use ecow::EcoString;

use crate::kind::SyntaxKind;
use crate::lexer::Token;
use crate::span::Span;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parser {
    pub start: usize,
    pub current: usize,
    pub tokens: Vec<Token>,
    nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node(Repr);

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Repr {
    ErrorNode(ErrorNode),
    SyntaxNode(SyntaxNode),
}

impl Display for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {:?} {}",
            match self {
                Self::SyntaxNode(syn) => syn.kind,
                Self::ErrorNode(err) => err.kind,
            },
            match self {
                Self::ErrorNode(err) => err.text.clone(),
                Self::SyntaxNode(syn) => syn.text.clone(),
            },
            match self {
                Self::SyntaxNode(_) => "".to_owned(),
                Self::ErrorNode(err) => err.error.clone().unwrap_or_default(),
            }
        )
    }
}

impl Display for ErrorNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErrorNode {
    kind: SyntaxKind,
    text: EcoString,
    error: Option<String>,
    hint: Option<String>,
    span: Span,
}

impl ErrorNode {
    pub fn new(
        kind: SyntaxKind,
        text: EcoString,
        error: Option<String>,
        hint: Option<String>,
        span: Span,
    ) -> Self {
        Self {
            kind,
            text,
            error,
            hint,
            span,
        }
    }
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
            self.tokens[self.current..].first().cloned()
        }
    }

    // eat current Token and return next Token, updating current
    fn advance(&mut self) -> Option<Token> {
        if self.is_at_end() {
            None
        } else {
            self.tokens[self.current..]
                .iter()
                .next()
                .inspect(|_| {
                    self.current += 1;
                })
                .cloned()
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        // Use a while let loop instead of checking self.current directly
        while let Some(token) = self.peek() {
            if token.kind == SyntaxKind::Eof {
                break;
            }
            let node = Node(self.scan());
            self.nodes.push(node);
        }
        std::mem::take(&mut self.nodes)
    }

    pub fn scan(&mut self) -> Repr {
        self.start = self.current;

        let text = EcoString::new();

        match self.advance() {
            Some(i) => {
                let start = i.span.start;
                let mut parse_inline_expr = |pat: SyntaxKind,
                                      kind: SyntaxKind,
                                      mut text: EcoString,
                                      error: Option<String>,
                                      hint: Option<String>| {
                    if let Some(next_token) = self.peek() {
                        if next_token.kind == SyntaxKind::WhiteSpace {
                            text.push_str(&next_token.text);
                            self.advance();
                        }
                    }
                    if let Some(next_token) = self.peek() {
                        if next_token.kind == SyntaxKind::Text {
                            text.push_str(&next_token.text);
                            self.advance();

                            if let Some(closing_token) = self.peek() {
                                if closing_token.kind == pat {
                                    self.advance();
                                    return Repr::SyntaxNode(SyntaxNode::new(
                                        kind,
                                        text.clone(),
                                        Span::new(start, closing_token.span.end),
                                    ));
                                }
                            }
                        }
                    }

                    Repr::ErrorNode(ErrorNode {
                        kind: SyntaxKind::Error,
                        text,
                        error,
                        hint,
                        span: i.span,
                    })
                };
                match i.kind {
                    SyntaxKind::Slash => parse_inline_expr(
                        SyntaxKind::Slash,
                        SyntaxKind::Italics,
                        text,
                        Some("Incomplete italic text".to_string()),
                        Some("Text must be wrapped in slash pairs like /text/".to_string()),
                    ),
                    SyntaxKind::Underscore => parse_inline_expr(
                        SyntaxKind::Underscore,
                        SyntaxKind::UnderLined,
                        text,
                        Some("Incomplete underlined text".to_string()),
                        Some("Text must be wrapped in underscore pairs like _text_".to_string()),
                    ),
                    SyntaxKind::Tilda => parse_inline_expr(
                        SyntaxKind::Tilda,
                        SyntaxKind::ListItem,
                        text,
                        Some("Incomplete list text".to_string()),
                        Some("Text must be wrapped in tilda pairs like ~text~".to_string()),
                    ),
                    SyntaxKind::Hyphen => parse_inline_expr(
                        SyntaxKind::Hyphen,
                        SyntaxKind::Strikethrough,
                        text,
                        Some("Incomplete italic text".to_string()),
                        Some("Text must be wrapped in tilda pairs like ~text~".to_string()),
                    ),

                    _ => Repr::SyntaxNode(SyntaxNode::new(i.kind, i.text.into(), i.span)),
                }
            }
            _ => Repr::SyntaxNode(SyntaxNode {
                kind: SyntaxKind::Eof,
                text: "".into(),
                span: Span::default(),
            }),
        }
    }
}

impl Iterator for Parser {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            None
        } else {
            // .cloned() cuz the type is &Token but we need to return owned Token
            // can we eleminate it
            self.tokens[self.current..].iter().next().cloned()
        }
    }
}
