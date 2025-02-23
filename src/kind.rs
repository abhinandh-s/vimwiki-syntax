use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum SyntaxKind {
    /// '*'
    Star,
    /// '/'
    Slash,
    /// spaces and tabs (preserved)
    WhiteSpace,
    /// '\n'
    NewLine,
    /// any other text
    Text,
    /// represents end of file, added by lexer
    Eof,
    /// '*'
    Astrisk,
    /// '_'
    Underscore,
    /// '~' ordered list
    Tilda,
    /// '-' unodered list
    Hyphen,
    /// '@' at symbol
    At,
    /// Error Node
    Error,
}

impl Display for SyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SyntaxKind::Star => "STAR",
                SyntaxKind::Slash => "SLASH",
                SyntaxKind::WhiteSpace => "WHITESPACE",
                SyntaxKind::NewLine => "NEWLINE",
                SyntaxKind::Text => "TEXT",
                SyntaxKind::Eof => "EOF",
                SyntaxKind::Astrisk => "ASTRISK",
                SyntaxKind::Underscore => "UNDERSCORE",
                SyntaxKind::Tilda => "TILDA",
                SyntaxKind::Hyphen => "HYPEN",
                SyntaxKind::At => "AT",
                SyntaxKind::Error => "ERROR",
            }
        )
    }
}

