#![allow(dead_code, unused_variables)]

use crate::kind::SyntaxKind;
use crate::span::Span;

trait AstNode<'a>: Sized {
    fn from_untyped(node: &'a SyntaxNode) -> Option<Self>
    where
        Self: std::marker::Sized;
    fn to_untyped(self) -> &'a SyntaxNode;
  }

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SyntaxNode {
    pub kind: SyntaxKind,
    pub text: String,
    span: Span,
    pub children: Vec<SyntaxNode>,
}

//
//pub struct Parser {
//    tokens: Vec<Token>,
//    current: usize,
//}
//
//pub struct ChildrenIterator {
//    inner: std::vec::IntoIter<LeafNode>,
//}
//
//impl ChildrenIterator {
//    pub fn new(children: Vec<LeafNode>) -> Self {
//        Self {
//            inner: children.into_iter(),
//        }
//    }
//}
//
//impl Iterator for ChildrenIterator {
//    type Item = LeafNode;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        self.inner.next()
//    }
//}
//
//impl Parser {
//    pub fn new(tokens: Vec<Token>) -> Self {
//        Self { tokens, current: 0 }
//    }
//
//    fn peek(&self) -> &Token {
//        &self.tokens[self.current]
//    }
//
//    fn previous(&self) -> &Token {
//        &self.tokens[self.current - 1]
//    }
//
//    fn is_at_end(&self) -> bool {
//        self.peek().kind == SyntaxKind::Eof
//    }
//
//    fn eat(&mut self) -> &Token {
//        if !self.is_at_end() {
//            self.current += 1;
//        }
//        self.previous()
//    }
//
//    fn eat_while(&mut self, pat: SyntaxKind) -> &Token {
//        loop {
//            let ch = self.peek();
//            if ch.kind != pat {
//                self.eat();
//            } else {
//                break;
//            }
//        }
//        self.previous()
//    }
//
//    fn parse(&mut self) -> InnerNode {
//        let mut root = InnerNode {
//            kind: SyntaxKind::Root,
//            // we are lying here
//            span: Span::new(1, self.tokens.len(), Range::default()),
//            children: Vec::new(),
//            len: 1,
//            descendants: 2,
//            erroneous: false,
//        };
//
//        while !self.is_at_end() {
//            let node = self.parse_element();
//            if node.kind != SyntaxKind::Error {
//                root.children.push(SyntaxNode::leaf(node.kind, node.text));
//            }
//        }
//        root
//    }
//
//    fn parse_element(&mut self) -> LeafNode {
//        let token = self.eat();
//        match token.kind {
//            // SyntaxKind::Astrisk => self.parse_astrisk(),
//            SyntaxKind::Slash => self.parse_italics(),
//            _ => self.error_node("unexpexted token"),
//        }
//    }
//
//    fn parse_astrisk(&self) -> LeafNode {
//        LeafNode::default()
//    }
//
//    fn parse_text(&self) -> LeafNode {
//        LeafNode::default()
//    }
//
//    /// Parses italics by consuming tokens until the closing slash is found.
//    fn parse_italics(&mut self) -> LeafNode {
//        // The starting slash has been consumed.
//        let start_token = self.previous();
//        let mut italic_text = EcoString::new();
//        let start_span = start_token.offset;
//
//        let start_line = self.peek().line;
//        let start_col = self.peek().col;
//
//        // Consume tokens until we find the closing slash.
//        while !self.is_at_end() && self.peek().kind != SyntaxKind::Slash {
//            let token = self.eat();
//            italic_text.push_str(&token.text);
//        }
//
//        // Consume the closing slash if it's there.
//        if !self.is_at_end() && self.peek().kind == SyntaxKind::Slash {
//            let end_token = self.eat();
//            let end_span = end_token.offset - 1;
//            LeafNode {
//                kind: SyntaxKind::Italics,
//                text: italic_text.clone(),
//                span: Span::new(
//                    start_span,
//                    end_span,
//                    Range {
//                        start: Position {
//                            line: start_line,
//                            col: start_col,
//                        },
//                        end: Position {
//                            line: end_token.line,
//                            col: self.previous().col,
//                        },
//                    },
//                ),
//                children: Vec::new(),
//            }
//        } else {
//            // No closing slash found; return an error node.
//            self.error_node("unexpexted token")
//        }
//    }
//
//    fn error_node(&self, text: &str) -> LeafNode {
//        LeafNode {
//            kind: SyntaxKind::Error,
//            text: text.into(),
//            span: Span::new(0, 0, Range::default()),
//            children: Vec::with_capacity(0),
//        }
//    }
//}
//
///// A node in the untyped syntax tree.
//#[derive(Debug, Clone, Eq, PartialEq, Hash)]
//pub struct SyntaxNode(Repr);
//
///// The three internal representations.
//#[derive(Debug, Clone, Eq, PartialEq, Hash)]
//enum Repr {
//    /// A leaf node.
//    Leaf(LeafNode),
//    /// A reference-counted inner node.
//    Inner(Arc<InnerNode>),
//    /// An error node.
//    Error(Arc<ErrorNode>),
//}
//
///// An inner node in the untyped syntax tree.
//#[derive(Debug, Clone, Eq, PartialEq, Hash)]
//struct InnerNode {
//    /// What kind of node this is (each kind would have its own struct in a
//    /// strongly typed AST).
//    kind: SyntaxKind,
//    /// The byte length of the node in the source.
//    len: usize,
//    /// The node's span.
//    span: Span,
//    /// The number of nodes in the whole subtree, including this node.
//    descendants: usize,
//    /// Whether this node or any of its children are erroneous.
//    erroneous: bool,
//    /// This node's children, losslessly make up this node.
//    children: Vec<SyntaxNode>,
//}
//impl InnerNode {
//    fn new(kind: SyntaxKind, children: Vec<SyntaxNode>) -> Self {
//        Self {
//            kind,
//            len: 0,
//            span: Span::default(),
//            descendants: 0,
//            erroneous: false,
//            children: Vec::new(),
//        }
//    }
//}
//
///// An error node in the untyped syntax tree.
//#[derive(Clone, Eq, PartialEq, Hash)]
//struct ErrorNode {
//    /// The source text of the node.
//    text: EcoString,
//    /// The syntax error.
//    error: SyntaxError,
//}
//
//impl ErrorNode {
//    /// Create new error node.
//    fn new(error: SyntaxError, text: impl Into<EcoString>) -> Self {
//        Self {
//            text: text.into(),
//            error,
//        }
//    }
//
//    /// The byte length of the node in the source text.
//    fn len(&self) -> usize {
//        self.text.len()
//    }
//
//    /// Add a user-presentable hint to this error node.
//    fn hint(&mut self, hint: impl Into<EcoString>) {
//        self.error.hints.push(hint.into());
//    }
//
//    /// Whether the two leaf nodes are the same apart from spans.
//    fn spanless_eq(&self, other: &Self) -> bool {
//        self.text == other.text && self.error.spanless_eq(&other.error)
//    }
//}
//
//impl Debug for ErrorNode {
//    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//        write!(f, "Error: {:?} ({})", self.text, self.error.message)
//    }
//}
//
///// A syntactical error.
//#[derive(Debug, Clone, Eq, PartialEq, Hash)]
//pub struct SyntaxError {
//    /// The node's span.
//    pub span: Span,
//    /// The error message.
//    pub message: EcoString,
//    /// Additional hints to the user, indicating how this error could be avoided
//    /// or worked around.
//    pub hints: EcoVec<EcoString>,
//}
//
//impl SyntaxNode {
//    /// Create a new leaf node.
//    pub fn leaf(kind: SyntaxKind, text: impl Into<EcoString>) -> Self {
//        Self(Repr::Leaf(LeafNode::new(kind, text)))
//    }
//
//    /// Create a new inner node with children.
//    pub fn inner(kind: SyntaxKind, children: Vec<SyntaxNode>) -> Self {
//        Self(Repr::Inner(Arc::new(InnerNode::new(kind, children))))
//    }
//
//    /// Create a new error node.
//    pub fn error(error: SyntaxError, text: impl Into<EcoString>) -> Self {
//        Self(Repr::Error(Arc::new(ErrorNode::new(error, text))))
//    }
//}
//impl SyntaxError {
//    /// Create a new detached syntax error.
//    pub fn new(message: impl Into<EcoString>) -> Self {
//        Self {
//            span: Span::default(),
//            message: message.into(),
//            hints: eco_vec![],
//        }
//    }
//
//    /// Whether the two errors are the same apart from spans.
//    fn spanless_eq(&self, other: &Self) -> bool {
//        self.message == other.message && self.hints == other.hints
//    }
//}
//#[derive(Debug, Clone, Eq, PartialEq, Hash)]
//pub struct LeafNode {
//    kind: SyntaxKind,
//    text: EcoString,
//    span: Span,
//    children: Vec<LeafNode>,
//}
//impl LeafNode {
//    fn new(kind: SyntaxKind, text: impl Into<EcoString>) -> Self {
//        Self {
//            kind,
//            text: text.into(),
//            span: Span::default(),
//            children: Vec::new(),
//        }
//    }
//}
//impl Default for LeafNode {
//    fn default() -> LeafNode {
//        LeafNode {
//            kind: SyntaxKind::Text,
//            text: "".into(),
//            span: Span {
//                start: 1,
//                end: 1,
//                range: Range::default(),
//            },
//            children: Vec::new(),
//        }
//    }
//}
//
//#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
//pub enum SyntaxKind {
//    Root,
//    Heading,
//    Bold,
//    Text,
//    WhiteSpace,
//    Error,
//    Eof,
//    Italics,
//}
//
