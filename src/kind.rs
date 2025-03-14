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
    /// root node of the document
    Root,
    /// `/` Italics
    Italics,
    Heading,
    ListItem,
    Bold,
    Strikethrough,
    UnderLined,
    IndentWhiteSpace,
    BoldMarker,
    HeadingMarker,
}

impl SyntaxKind {
    pub fn is_special() {}
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
                SyntaxKind::Hyphen => "HYPHEN",
                SyntaxKind::At => "AT",
                SyntaxKind::Error => "ERROR",
                SyntaxKind::Root => "ROOT",
                SyntaxKind::Italics => "ITALIC",
                SyntaxKind::Heading => "HEADING",
                SyntaxKind::ListItem => "LISTITEM",
                SyntaxKind::Bold => "BOLD",
                SyntaxKind::Strikethrough => "STRIKETHROUGH",
                SyntaxKind::UnderLined => "UNDERLINED",
                SyntaxKind::IndentWhiteSpace => "INDENTWHITESPACE",
                SyntaxKind::BoldMarker => "BOLDMARKER",
                SyntaxKind::HeadingMarker => "HEADINGMARKER",
            }
        )
    }
}
