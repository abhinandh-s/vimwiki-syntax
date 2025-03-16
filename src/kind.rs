use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum SyntaxKind {
    /// '['
    LeftSqBrackets,
    /// ']'
    RightSqBrackets,
    /// '('
    LeftParen,
    /// ')'
    RightParen,
    /// '{'
    LeftCurlyBraces,
    /// '}'
    RightCurlyBraces,
    /// '/'
    Slash,
    /// '*'
    Astrisk,
    /// '~' ordered list
    Tilda,
    /// '-' unodered list
    Hyphen,
    // `:`
    SemiColon,
    // `#`
    HashTag,
    // `=`
    Equal,
    // `%` for commets
    Percentage,
    /// spaces and tabs (preserved)
    WhiteSpace,
    /// '\n'
    NewLine,
    /// any other text
    Text,
    /// represents end of file, added by lexer
    Eof,
    /// '_'
    Underscore,
    /// '@' at symbol
    At,
    /// Error Node
    Error,
    /// root node of the document
    Root,
    /// ` `
    IndentWhiteSpace,
    /// "`"
    CodeMarker,
    /// `^`
    SuperScriptMarker,
    /// `,`
    SubScriptMarker,
}

impl SyntaxKind {
    pub fn is_grouping(&self) -> bool {
        matches!(self, Self::LeftCurlyBraces)
    }
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}

impl Display for SyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SyntaxKind::Slash => "SLASH",
                SyntaxKind::WhiteSpace => "WHITESPACE",
                SyntaxKind::NewLine => "NEWLINE",
                SyntaxKind::Text => "TEXT",
                SyntaxKind::Eof => "EOF",
                SyntaxKind::Astrisk => "ASTRISK",
                SyntaxKind::Underscore => "UNDERSCORE",
                SyntaxKind::Tilda => "TILDA",
                SyntaxKind::Hyphen => "HYPHEN",
                SyntaxKind::At => "AT",
                SyntaxKind::Error => "ERROR",
                SyntaxKind::Root => "ROOT",
                SyntaxKind::IndentWhiteSpace => "INDENTWHITESPACE",
                SyntaxKind::LeftSqBrackets => "TODO",
                SyntaxKind::RightSqBrackets => "TODO",
                SyntaxKind::LeftParen => "TODO",
                SyntaxKind::RightParen => "TODO",
                SyntaxKind::LeftCurlyBraces => "TODO",
                SyntaxKind::RightCurlyBraces => "TODO",
                SyntaxKind::SemiColon => "TODO",
                SyntaxKind::HashTag => "TODO",
                SyntaxKind::Equal => "TODO",
                SyntaxKind::Percentage => "TODO",
                SyntaxKind::CodeMarker => "TODO",
                SyntaxKind::SuperScriptMarker => "TODO",
                SyntaxKind::SubScriptMarker => "TODO",
            }
        )
    }
}
